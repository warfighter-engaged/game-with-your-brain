#![warn(clippy::all)]

mod eeg;
mod error;
mod myo;
mod springboard;

pub use error::*;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// The simple-signal crate is used to handle incoming signals
use simple_signal::{self, Signal};

use rppal::system::DeviceInfo;

enum DeviceSignal {
    Eeg(u8, u8, u8),
    Myo1(u16),
    Myo2(u16),
}

pub fn main() -> Result<()> {
    println!("Running wfpi on a {}.", DeviceInfo::new()?.model());

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
        let mut myo_reader = myo::MyoReader::init().expect("Myo reader failed to initialize");
        println!("Initialized myo");
        while myo_run.load(Ordering::SeqCst) {
            if let Err(err) = myo_reader.update() {
                println!("failed to update myo: {}", err);
                println!("sleeping for 5 seconds...");
                std::thread::sleep(std::time::Duration::from_secs(5)); // TODO: Look for ways to interrupt this from a Ctrl-C signal.
                continue;
            }
            if myo_reader.has_new_data() {
                if let Err(_err) = myo_tx.send(DeviceSignal::Myo1(myo_reader.get_value(myo::Side::Left))) {
                    println!("failed to send data");
                    break;
                }
                if let Err(_err) = myo_tx.send(DeviceSignal::Myo2(myo_reader.get_value(myo::Side::Right))) {
                    println!("failed to send data");
                    break;
                }
            }
        }
    });

    let mut output = {
        let mut res = springboard::Springboard::init();
        while res.is_err() && running.load(Ordering::SeqCst) {
            println!("failed to connect to XAC: {}", res.err().unwrap());
            println!("sleeping for 5 seconds...");
            std::thread::sleep(std::time::Duration::from_secs(5));
            res = springboard::Springboard::init();
        }
        res.unwrap()
    };

    let mut last_data = [0; 5];

    while running.load(Ordering::SeqCst) {
        let data = rx.recv()?;
        match data {
            DeviceSignal::Eeg(attention, meditation, signal_quality) => {
                last_data[0] = u16::from(attention);
                last_data[1] = u16::from(meditation);
                last_data[2] = u16::from(signal_quality);

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
                output.update_left_btn(val > 200);
                last_data[3] = val;
            }
            DeviceSignal::Myo2(val) => {
                output.update_right_btn(val > 200);
                last_data[4] = val;
            }
        }
        println!("{:?}", last_data);
    }

    // Join all the threads - waits for anything they need to clean up to finish before we do any cleanup from the main thread
    eeg_join.join().expect("EEG thread failed to join");
    myo_join.join().expect("Myo thread failed to join");

    // Cleanup phase

    Ok(())
}
