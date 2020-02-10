#![warn(clippy::all)]
#![allow(dead_code)]

mod eeg;
mod emg_process;
mod error;
mod myo;
mod springboard;

pub use error::*;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, List, Marker, Text, Widget};
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
        _input_handle: thread::JoinHandle<()>,
        _tick_handle: thread::JoinHandle<()>,
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
                        if let Ok(key) = evt {
                            if tx.send(Event::Input(key)).is_err() {
                                return;
                            }
                            if key == config.exit_key {
                                return;
                            }
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
                _input_handle: input_handle,
                _tick_handle: tick_handle,
            }
        }

        pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
            self.rx.recv()
        }

        pub fn next_nonblocking(&self) -> Result<Event<Key>, mpsc::TryRecvError> {
            self.rx.try_recv()
        }
    }
}

enum DeviceSignal {
    Eeg(u8, u8, u8),
    Myo1(bool, i32),
    Myo2(bool, i32),
}

fn fmin(v1: f64, v2: f64) -> f64 {
    if v1 < v2 {
        v1
    } else {
        v2
    }
}

fn fmax(v1: f64, v2: f64) -> f64 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
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
                if let Err(_err) = eeg_tx.send(DeviceSignal::Eeg(
                    mindwave.get_attention(),
                    mindwave.get_meditation(),
                    mindwave.get_quality(),
                )) {
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
                    let (s, v) = myo_parser.get_value(myo::Side::Left);
                    if let Err(_err) = myo_tx.send(DeviceSignal::Myo1(s, v)) {
                        println!("failed to send data");
                        break;
                    }
                    let (s, v) = myo_parser.get_value(myo::Side::Right);
                    if let Err(_err) = myo_tx.send(DeviceSignal::Myo2(s, v)) {
                        println!("failed to send data");
                        break;
                    }
                }
                Ok(false) => (), // no new data
            }
        }
    });

    let (mut rx_o, tx_o) = single_value_channel::channel_starting_with((
        vec![],
        vec![],
        vec![],
        (false, false, 0f64),
        0f64,
        false,
    ));

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

        let mut current_time = 0f64;

        let mut sending = (false, false, 0f64);

        let mut override_output = false;

        while collector_running.load(Ordering::SeqCst) {
            let data = rx.recv().unwrap();
            const EEG_LOWER_BOUND: f64 = 20f64;
            const EEG_UPPER_BOUND: f64 = 80f64;
            const DATA_AMOUNT: usize = 200;
            match data {
                DeviceSignal::Eeg(attention, meditation, signal_quality) => {
                    last_data[0] = u16::from(attention);
                    last_data[1] = u16::from(meditation);
                    last_data[2] = u16::from(signal_quality);

                    eeg_data.push((current_time, last_data));
                    if eeg_data.len() > DATA_AMOUNT {
                        eeg_data.remove(0);
                    }

                    // Shift attention to only count a range from 20-80
                    // Values outside that range are compressed to 0 or 100
                    let attention = f64::from(attention);
                    let attention = if attention < EEG_LOWER_BOUND {
                        EEG_LOWER_BOUND
                    } else if attention > EEG_UPPER_BOUND {
                        EEG_UPPER_BOUND
                    } else {
                        attention
                    };
                    let attention = (attention - EEG_LOWER_BOUND)
                        * (100f64 / (EEG_UPPER_BOUND - EEG_LOWER_BOUND));

                    if !override_output {
                        sending.2 = attention;
                        output
                            .update_trigger(attention)
                            .expect("failed to write to XAC");
                    }
                }
                DeviceSignal::Myo1(state, val) => {
                    // println!("MYO (Left): {}", val);
                    if myo_left_data.len() > DATA_AMOUNT {
                        myo_left_data.remove(0);
                    }
                    myo_left_data.push((current_time, val as f64));

                    if !override_output {
                        sending.0 = state;
                        output.update_left_btn(state);
                    }
                }
                DeviceSignal::Myo2(state, val) => {
                    if myo_right_data.len() > DATA_AMOUNT {
                        myo_right_data.remove(0);
                    }
                    myo_right_data.push((current_time, val as f64));

                    if !override_output {
                        sending.1 = state;
                        output.update_right_btn(state);
                    }
                }
            }
            tx_o.update((
                eeg_data.clone(),
                myo_left_data.clone(),
                myo_right_data.clone(),
                sending,
                current_time,
                override_output,
            ))
            .expect("failed to send");
            current_time += 0.5f64;

            if let Ok(event::Event::Input(input)) = events.next_nonblocking() {
                match input {
                    termion::event::Key::Char('q') => {
                        collector_running.store(false, Ordering::SeqCst);
                    }
                    termion::event::Key::Char('z') => {
                        if override_output {
                            output.update_left_btn(true);
                            sending.0 = true;
                        }
                    }
                    termion::event::Key::Char('x') => {
                        if override_output {
                            output.update_left_btn(false);
                            sending.0 = false;
                        }
                    }
                    termion::event::Key::Char('c') => {
                        if override_output {
                            output.update_right_btn(true);
                            sending.1 = true;
                        }
                    }
                    termion::event::Key::Char('v') => {
                        if override_output {
                            output.update_right_btn(false);
                            sending.1 = false;
                        }
                    }
                    termion::event::Key::Char('b') => {
                        if override_output {
                            if let Err(e) = output.update_trigger(100f64) {
                                println!("Error updating trigger: {:?}", e);
                            }
                            sending.2 = 100f64;
                        }
                    }
                    termion::event::Key::Char('n') => {
                        if override_output {
                            if let Err(e) = output.update_trigger(0f64) {
                                println!("Error updating trigger: {:?}", e)
                            }
                            sending.2 = 0f64;
                        }
                    }
                    termion::event::Key::Char('m') => {
                        override_output = !override_output;
                    }
                    _ => (),
                };
            }
        }
    });

    while running.load(Ordering::SeqCst) {
        let (eeg_data, myo_left_data, myo_right_data, sending, curr_time, override_output) =
            rx_o.latest();

        let myo_left_dataset = myo_left_data.clone(); // TODO: Change me!
        let myo_right_dataset = myo_right_data.clone();

        let myo_left_min = (&myo_left_dataset)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.1),
                Some(y) => Some(fmin(x.1, y)),
            })
            .unwrap_or(0f64);
        let myo_left_max = (&myo_left_dataset)
            .iter()
            .fold(None, |max, x| match max {
                None => Some(x.1),
                Some(y) => Some(fmax(x.1, y)),
            })
            .unwrap_or(0f64);
        let myo_right_min = (&myo_right_dataset)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.1),
                Some(y) => Some(fmin(x.1, y)),
            })
            .unwrap_or(0f64);
        let myo_right_max = (&myo_right_dataset)
            .iter()
            .fold(None, |max, x| match max {
                None => Some(x.1),
                Some(y) => Some(fmax(x.1, y)),
            })
            .unwrap_or(0f64);
        let myo_min = fmin(myo_left_min, myo_right_min);
        let myo_max = fmax(myo_left_max, myo_right_max);

        let myo_min_x = (&myo_left_dataset)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.0),
                Some(y) => Some(fmin(x.0, y)),
            })
            .unwrap_or(0f64);
        let myo_max_x = (&myo_left_dataset)
            .iter()
            .fold(None, |max, x| match max {
                None => Some(x.0),
                Some(y) => Some(fmax(x.0, y)),
            })
            .unwrap_or(0f64);

        let eeg_data_1: Vec<(f64, f64)> = eeg_data
            .iter()
            .map(|(ct, datas)| (*ct, datas[0] as f64))
            .collect();
        let eeg_data_2: Vec<(f64, f64)> = eeg_data
            .iter()
            .map(|(ct, datas)| (*ct, datas[1] as f64))
            .collect();
        let eeg_data_3: Vec<(f64, f64)> = eeg_data
            .iter()
            .map(|(ct, datas)| (*ct, datas[2] as f64))
            .collect();

        let eeg_min_1 = (&eeg_data_1)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.1),
                Some(y) => Some(fmin(x.1, y)),
            })
            .unwrap_or(0f64);
        let eeg_max_1 = (&eeg_data_1)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.1),
                Some(y) => Some(fmax(x.1, y)),
            })
            .unwrap_or(0f64);
        let eeg_min_2 = (&eeg_data_2)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.1),
                Some(y) => Some(fmin(x.1, y)),
            })
            .unwrap_or(0f64);
        let eeg_max_2 = (&eeg_data_2)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.1),
                Some(y) => Some(fmax(x.1, y)),
            })
            .unwrap_or(0f64);
        let eeg_min_3 = (&eeg_data_3)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.1),
                Some(y) => Some(fmin(x.1, y)),
            })
            .unwrap_or(0f64);
        let eeg_max_3 = (&eeg_data_3)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.1),
                Some(y) => Some(fmax(x.1, y)),
            })
            .unwrap_or(0f64);
        let eeg_min = fmin(eeg_min_1, fmin(eeg_min_2, eeg_min_3));
        let eeg_max = fmax(eeg_max_1, fmax(eeg_max_2, eeg_max_3));

        let eeg_min_x = (&eeg_data_1)
            .iter()
            .fold(None, |min, x| match min {
                None => Some(x.0),
                Some(y) => Some(fmin(x.0, y)),
            })
            .unwrap_or(0f64);
        let eeg_max_x = (&eeg_data_1)
            .iter()
            .fold(None, |max, x| match max {
                None => Some(x.0),
                Some(y) => Some(fmax(x.0, y)),
            })
            .unwrap_or(0f64);

        terminal
            .draw(|mut f| {
                let size = f.size();

                let constraints_1 = vec![Constraint::Percentage(80), Constraint::Percentage(20)];
                let constraints_2 = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
                let main_chunks = Layout::default()
                    .constraints(constraints_1)
                    .direction(Direction::Horizontal)
                    .split(size);
                let chunks = Layout::default()
                    .constraints(constraints_2)
                    .direction(Direction::Vertical)
                    .split(main_chunks[0]);

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
                            .bounds([eeg_min_x, eeg_max_x])
                            .labels(&[&format!("{}", eeg_min_x), &format!("{}", eeg_max_x)]),
                    )
                    .y_axis(
                        Axis::default()
                            .title("EEG")
                            .style(Style::default().fg(Color::Gray))
                            .labels_style(Style::default().modifier(Modifier::ITALIC))
                            .bounds([eeg_min, eeg_max])
                            .labels(&[
                                &format!("{}", eeg_min),
                                &format!("{}", (eeg_min + eeg_max) / 2f64),
                                &format!("{}", eeg_max),
                            ]),
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
                            .labels(&[&format!("{}", myo_min_x), &format!("{}", myo_max_x)]),
                    )
                    .y_axis(
                        Axis::default()
                            .title("MYO")
                            .style(Style::default().fg(Color::Gray))
                            .labels_style(Style::default().modifier(Modifier::ITALIC))
                            .bounds([myo_min, myo_max])
                            .labels(&[
                                &format!("{}", myo_min),
                                &format!("{}", (myo_min + myo_max) / 2f64),
                                &format!("{}", myo_max),
                            ]),
                    )
                    .datasets(&[
                        Dataset::default()
                            .name("left")
                            .marker(Marker::Dot)
                            .style(Style::default().fg(Color::Cyan))
                            .data(&myo_left_dataset),
                        Dataset::default()
                            .name("right")
                            .marker(Marker::Braille)
                            .style(Style::default().fg(Color::Yellow))
                            .data(&myo_right_dataset),
                    ])
                    .render(&mut f, chunks[1]);

                let events_list = vec![
                    Text::styled(
                        format!("Myo (L): {}", sending.0),
                        Style::default().fg(Color::White),
                    ),
                    Text::styled(
                        format!("Myo (R): {}", sending.1),
                        Style::default().fg(Color::White),
                    ),
                    Text::styled(
                        format!("EEG: {}", sending.2),
                        Style::default().fg(Color::White),
                    ),
                    Text::styled(
                        format!("Curr time: {}", curr_time),
                        Style::default().fg(Color::White),
                    ),
                    Text::styled(
                        format!("Keyboard Override: {}", override_output),
                        Style::default().fg(Color::White),
                    ),
                ];
                List::new(events_list.into_iter())
                    .block(Block::default().borders(Borders::ALL).title("XAC Output"))
                    .render(&mut f, main_chunks[1]);
            })
            .unwrap();

        // if let Ok(event::Event::Input(input)) = events.next() {
        //     if let termion::event::Key::Char('q') = input {
        //         running.store(false, Ordering::SeqCst);
        //     }
        // }
    }

    // Join all the threads - waits for anything they need to clean up to finish before we do any cleanup from the main thread
    eeg_join.join().expect("EEG thread failed to join");
    myo_join.join().expect("Myo thread failed to join");
    collector_join
        .join()
        .expect("Collector thread failed to join");

    // Cleanup phase

    Ok(())
}
