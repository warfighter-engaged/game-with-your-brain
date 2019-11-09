#![warn(clippy::all)]

mod eeg;
mod error;
mod myo;
mod sdft;
mod springboard;

pub use error::*;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Widget};
use tui::Terminal;

// The simple-signal crate is used to handle incoming signals
use simple_signal::{self, Signal};

use rppal::system::DeviceInfo;

mod event {
    use std::io;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use termion::event::Key;
    use termion::input::TermRead;

    pub enum Event<I> {
        Input(I),
        Tick,
    }

    /// A small event handler that wrap termion input and tick events. Each event
    /// type is handled in its own thread and returned to a common `Receiver`
    pub struct Events {
        rx: mpsc::Receiver<Event<Key>>,
        input_handle: thread::JoinHandle<()>,
        tick_handle: thread::JoinHandle<()>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Config {
        pub exit_key: Key,
        pub tick_rate: Duration,
    }

    impl Default for Config {
        fn default() -> Config {
            Config {
                exit_key: Key::Char('q'),
                tick_rate: Duration::from_millis(250),
            }
        }
    }

    impl Events {
        pub fn new() -> Events {
            Events::with_config(Config::default())
        }

        pub fn with_config(config: Config) -> Events {
            let (tx, rx) = mpsc::channel();
            let input_handle = {
                let tx = tx.clone();
                thread::spawn(move || {
                    let stdin = io::stdin();
                    for evt in stdin.keys() {
                        match evt {
                            Ok(key) => {
                                if let Err(_) = tx.send(Event::Input(key)) {
                                    return;
                                }
                                if key == config.exit_key {
                                    return;
                                }
                            }
                            Err(_) => {}
                        }
                    }
                })
            };
            let tick_handle = {
                let tx = tx.clone();
                thread::spawn(move || {
                    let tx = tx.clone();
                    loop {
                        tx.send(Event::Tick).unwrap();
                        thread::sleep(config.tick_rate);
                    }
                })
            };
            Events {
                rx,
                input_handle,
                tick_handle,
            }
        }

        pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
            self.rx.recv()
        }
    }
}

enum DeviceSignal {
    Eeg(u8, u8, u8),
    Myo1((num::Complex<f64>, u16, Vec<num::Complex<f64>>)),
    Myo2((num::Complex<f64>, u16, Vec<num::Complex<f64>>)),
}

pub fn main() -> Result<()> {
    println!("Running wfpi on a {}.", DeviceInfo::new()?.model());

    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = event::Events::new();

    let running = Arc::new(AtomicBool::new(true));

    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, atomically set `running` to false.
    simple_signal::set_handler(&[Signal::Int, Signal::Term], {
        let running = running.clone();
        move |_| {
            running.store(false, Ordering::SeqCst);
        }
    });

    // Create a multi-producer, single-consumer FIFO queue for signals
    // This will collect data from the EEG and myo devices on separate threads,
    // and then collapse them down into a single thread for consumption.
    let (tx, rx) = std::sync::mpsc::channel();

    // We have to clone the sender across all threads it will be used in, along with
    // the atomic `running` variable. This running variable must be used to shut down
    let eeg_tx = tx.clone();
    let eeg_run = running.clone();
    let eeg_join = std::thread::spawn(move || {
        let mut mindwave = eeg::Mindwave::init().expect("failed to initialize mindwave");
        println!("Initialized mindwave");
        while eeg_run.load(Ordering::SeqCst) {
            if let Err(err) = mindwave.update() {
                println!("failed to update mindwave: {}", err);
                println!("sleeping for 5 seconds...");
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
            if mindwave.has_new_data() {
                if let Err(_err) = eeg_tx.send(DeviceSignal::Eeg(mindwave.get_attention(), mindwave.get_meditation(), mindwave.get_quality())) {
                    println!("failed to send data"); // This happens if the receiver has already hung up. Not really an error, but we don't want to keep shouting at a hung-up receiver, so we just break the loop.
                    break;
                }
            }
        }
    });

    let myo_tx = tx.clone();
    let myo_run = running.clone();
    let myo_join = std::thread::spawn(move || {
        let mut myo_parser = myo::MyoParser::new().expect("MYO parser failed to initialize");
        println!("Initialized myo");
        while myo_run.load(Ordering::SeqCst) {
            match myo_parser.update() {
                Err(err) => {
                    println!("failed to update myo: {}", err);
                    println!("sleeping for 5 seconds...");
                    std::thread::sleep(std::time::Duration::from_secs(5)); // TODO: Look for ways to interrupt this from a Ctrl-C signal.
                    continue;
                }
                Ok(true) => {
                    if let Err(_err) = myo_tx.send(DeviceSignal::Myo1((myo_parser.get_value(myo::Side::Left).unwrap(), myo_parser.get_raw_value(myo::Side::Left), myo_parser.get_values(myo::Side::Left).to_owned()))) {
                        println!("failed to send data");
                        break;
                    }
                    if let Err(_err) = myo_tx.send(DeviceSignal::Myo2((myo_parser.get_value(myo::Side::Right).unwrap(), myo_parser.get_raw_value(myo::Side::Right), myo_parser.get_values(myo::Side::Right).to_owned()))) {
                        println!("failed to send data");
                        break;
                    }
                }
                Ok(false) => () // no new data
            }
        }
    });

    let (tx_o, rx_o) = std::sync::mpsc::channel();

    let collector_tx = tx_o.clone();
    let collector_running = running.clone();
    let collector_join = std::thread::spawn(move || {
        let mut output = {
            let mut res = springboard::Springboard::init();
            while res.is_err() && collector_running.load(Ordering::SeqCst) {
                println!("failed to connect to XAC: {}", res.err().unwrap());
                println!("sleeping for 5 seconds...");
                std::thread::sleep(std::time::Duration::from_secs(5));
                res = springboard::Springboard::init();
            }
            res.unwrap()
        };

        let mut last_data = [0; 3];
        let mut eeg_data: Vec<(f64, [u16; 3])> = vec![];

        let mut myo_left_data: Vec<(f64, f64)> = vec![];
        let mut myo_right_data: Vec<(f64, f64)> = vec![];

        let mut myo_left_raw: Vec<(f64, f64)> = vec![];
        let mut myo_right_raw: Vec<(f64, f64)> = vec![];

        let mut myo_left_values: Vec<(f64, f64)> = vec![];
        let mut myo_right_values: Vec<(f64, f64)> = vec![];

        let mut pressed = [false; 2];

        let mut current_time = 0f64;

        while collector_running.load(Ordering::SeqCst) {
            let data = rx.recv().unwrap();
            match data {
                DeviceSignal::Eeg(attention, meditation, signal_quality) => {
                    last_data[0] = u16::from(attention);
                    last_data[1] = u16::from(meditation);
                    last_data[2] = u16::from(signal_quality);

                    eeg_data.push((current_time, last_data));
    
                    // Shift attention to only count a range from 20-80
                    // Values outside that range are compressed to 0 or 100
                    let attention = f64::from(attention);
                    let attention = if attention < 20f64 {
                        20f64
                    } else if attention > 80f64 {
                        80f64
                    } else {
                        attention
                    };
                    let attention = (attention - 20f64) * (100f64 / 60f64);
                    output.update_trigger(attention).expect("failed to write to XAC");
                }
                DeviceSignal::Myo1(val) => {
                    // println!("MYO (Left): {}", val);
                    if myo_left_data.len() > 512 {
                        myo_left_data.remove(0);
                        myo_left_raw.remove(0);
                    }
                    myo_left_data.push((current_time, val.0.re));
                    myo_left_raw.push((current_time, val.1 as f64));
                    myo_left_values = val.2.iter().enumerate().map(|(idx, val)| {
                        (idx as f64, val.re)
                    }).collect();
                }
                DeviceSignal::Myo2(val) => {
                    if myo_right_data.len() > 512 {
                        myo_right_data.remove(0);
                        myo_right_raw.remove(0);
                    }
                    myo_right_data.push((current_time, val.0.re));
                    myo_right_raw.push((current_time, val.1 as f64));
                    myo_right_values = val.2.iter().enumerate().map(|(idx, val)| {
                        (idx as f64, val.re)
                    }).collect();
                }
            }
            collector_tx.send((eeg_data.clone(), myo_left_data.clone(), myo_right_data.clone(), myo_left_raw.clone(), myo_right_raw.clone(), myo_left_values.clone(), myo_right_values.clone(), current_time)).expect("failed to send");
            current_time += 0.5f64;
        }
    });
    
    let mut myo_left_val_avg: Vec<(f64, f64)> = vec![];
    let mut myo_left_val_std_dev: Vec<(f64, f64)> = vec![];
    let mut current_time = 0f64;

    while running.load(Ordering::SeqCst) {
        // println!("{:?}", last_data);

        let mut eeg_data: Vec<(f64, [u16;3])> = vec![];
        let mut myo_left_data: Vec<(f64, f64)> = vec![];
        let mut myo_right_data: Vec<(f64, f64)> = vec![];
        let mut myo_left_raw: Vec<(f64, f64)> = vec![];
        let mut myo_right_raw: Vec<(f64, f64)> = vec![];
        let mut myo_left_vals: Vec<(f64, f64)> = vec![];
        let mut myo_right_vals: Vec<(f64, f64)> = vec![];
        let mut had_some = false;
        while let Some((eeg, left, right, left_raw, right_raw, left_vals, right_vals, _curr_time)) = rx_o.try_iter().next() {
            eeg_data = eeg;
            myo_left_data = left;
            myo_right_data = right;
            myo_left_raw = left_raw;
            myo_right_raw = right_raw;
            myo_left_vals = left_vals;
            myo_right_vals = right_vals;
            had_some = true;
        }

        if !had_some {
            continue;
        }

        current_time += 1f64;

        // spiders georg is an outlier adn should not have been counted
        // aka remove the DC component
        if myo_left_vals.len() > 1 {
            myo_left_vals.remove(0);
        }

        let myo_left_avg = (&myo_left_vals).iter().fold(0f64, |sum, val| {
            sum + val.1
        }) / myo_left_vals.len() as f64;
        if myo_left_val_avg.len() > 128 {
            myo_left_val_avg.remove(0);
        }
        myo_left_val_avg.push((current_time, myo_left_avg));

        let myo_left_std_dev = (&myo_left_vals).iter().fold(0f64, |sum, val| {
            sum + (val.1 - myo_left_avg).powf(2f64)
        }).sqrt();
        if myo_left_val_std_dev.len() > 128 {
            myo_left_val_avg.remove(0);
        }
        myo_left_val_std_dev.push((current_time, myo_left_std_dev));

        

        let myo_min = (&myo_left_val_std_dev).iter().fold(None, |min, x| match min {
            None => Some(x.1),
            Some(y) => Some(if x.1 < y { x.1 } else { y })
        }).unwrap_or(0f64);
        let myo_max = (&myo_left_val_std_dev).iter().fold(None, |max, x| match max {
            None => Some(x.1),
            Some(y) => Some(if x.1 > y { x.1 } else { y })
        }).unwrap_or(0f64);

        let myo_min_x = (&myo_left_val_std_dev).iter().fold(None, |min, x| match min {
            None => Some(x.0),
            Some(y) => Some(if x.0 < y { x.0 } else { y })
        }).unwrap_or(0f64);
        let myo_max_x = (&myo_left_val_std_dev).iter().fold(None, |max, x| match max {
            None => Some(x.0),
            Some(y) => Some(if x.0 > y { x.0 } else { y })
        }).unwrap_or(0f64);

        let eeg_data_1: Vec<(f64, f64)> = eeg_data.iter().map(|(ct, datas)| (*ct, datas[0] as f64)).collect();
        let eeg_data_2: Vec<(f64, f64)> = eeg_data.iter().map(|(ct, datas)| (*ct, datas[1] as f64)).collect();
        let eeg_data_3: Vec<(f64, f64)> = eeg_data.iter().map(|(ct, datas)| (*ct, datas[2] as f64)).collect();

        terminal.draw(|mut f| {
            let size = f.size();

            let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
            let chunks = Layout::default()
                .constraints(constraints)
                .direction(Direction::Vertical)
                .split(size);

            // EEG Chart

            Chart::default()
                .block(
                    Block::default()
                        .title("Myo Data")
                        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                    .title("Ticks")
                    .style(Style::default().fg(Color::Gray))
                    .labels_style(Style::default().modifier(Modifier::ITALIC))
                    .bounds([myo_min_x, myo_max_x])
                    .labels(&[&format!("{}", myo_min_x), &format!("{}", myo_max_x)])
                )
                .y_axis(
                    Axis::default()
                        .title("SDFT")
                        .style(Style::default().fg(Color::Gray))
                        .labels_style(Style::default().modifier(Modifier::ITALIC))
                        .bounds([myo_min, myo_max])
                        .labels(&[&format!("{}", myo_min), &format!("{}", (myo_min + myo_max) / 2f64), &format!("{}", myo_max)])
                )
                .datasets(&[
                    Dataset::default()
                        .name("attention")
                        .marker(Marker::Dot)
                        .style(Style::default().fg(Color::Yellow))
                        .data(&eeg_data_1),
                    Dataset::default()
                        .name("meditation")
                        .marker(Marker::Dot)
                        .style(Style::default().fg(Color::Cyan))
                        .data(&eeg_data_2),
                    Dataset::default()
                        .name("signal quality")
                        .marker(Marker::Dot)
                        .style(Style::default().fg(Color::Red))
                        .data(&eeg_data_3),
                ])
                .render(&mut f, chunks[0]);

            // MYO Chart
            Chart::default()
                .block(
                    Block::default()
                        .title("Myo Data")
                        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                    .title("Ticks")
                    .style(Style::default().fg(Color::Gray))
                    .labels_style(Style::default().modifier(Modifier::ITALIC))
                    .bounds([myo_min_x, myo_max_x])
                    .labels(&[&format!("{}", myo_min_x), &format!("{}", myo_max_x)])
                )
                .y_axis(
                    Axis::default()
                        .title("SDFT")
                        .style(Style::default().fg(Color::Gray))
                        .labels_style(Style::default().modifier(Modifier::ITALIC))
                        .bounds([myo_min, myo_max])
                        .labels(&[&format!("{}", myo_min), &format!("{}", (myo_min + myo_max) / 2f64), &format!("{}", myo_max)])
                )
                .datasets(&[
                    Dataset::default()
                        .name("left")
                        .marker(Marker::Dot)
                        .style(Style::default().fg(Color::Cyan))
                        .data(&myo_left_val_std_dev),
                        /*
                    Dataset::default()
                        .name("right")
                        .marker(Marker::Braille)
                        .style(Style::default().fg(Color::Yellow))
                        .data(&myo_right_vals),
                        */
                ])
                .render(&mut f, chunks[1]);
        }).unwrap();

        match events.next()? {
            event::Event::Input(input) => {
                if input == termion::event::Key::Char('q') {
                    let running = running.clone();
                    running.store(false, Ordering::SeqCst);
                }
            },
            _ => ()
        }
    }

    // Join all the threads - waits for anything they need to clean up to finish before we do any cleanup from the main thread
    eeg_join.join().expect("EEG thread failed to join");
    myo_join.join().expect("Myo thread failed to join");
    collector_join.join().expect("Collector thread failed to join");

    // Cleanup phase

    Ok(())
}
