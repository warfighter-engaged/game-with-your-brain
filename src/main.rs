#![warn(clippy::all)]

mod eeg;
mod error;
mod myo;
mod pins;
mod springboard;

pub use error::*;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// The simple-signal crate is used to handle incoming signals
use simple_signal::{self, Signal};

use rppal::system::DeviceInfo;

enum DeviceSignal {
    Eeg(u8, u8, u8),
    Myo1(u8),
    Myo2(u8),
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
        while eeg_run.load(Ordering::SeqCst) {
            mindwave.update().expect("failed to update mindwave");
            if mindwave.has_new_data() {
                let res = eeg_tx.send(DeviceSignal::Eeg(mindwave.get_attention(), mindwave.get_meditation(), mindwave.get_quality()));
                if res.is_err() {
                    println!("Failed to send data"); // This happens if the receiver has already hung up. Not really an error, but we don't want to keep shouting at a hung-up receiver, so we just break the loop.
                    break;
                }
            }
        }
    });

    let myo1_tx = tx.clone();
    let myo1_run = running.clone();
    let myo1_join = std::thread::spawn(move || {
        while myo1_run.load(Ordering::SeqCst) {
            let res = myo1_tx.send(DeviceSignal::Myo1(200));
            if res.is_err() {
                println!("Failed to send data");
                break;
            }
        }
    });

    let myo2_tx = tx.clone();
    let myo2_run = running.clone();
    let myo2_join = std::thread::spawn(move || {
        while myo2_run.load(Ordering::SeqCst) {
            let res = myo2_tx.send(DeviceSignal::Myo2(100));
            if res.is_err() {
                println!("Failed to send data");
                break;
            }
        }
    });

    while running.load(Ordering::SeqCst) {
        let data = rx.recv()?;
        match data {
            DeviceSignal::Eeg(attention, meditation, signal_quality) => {
                println!("Received EEG data: {} | {} | {}", attention, meditation, signal_quality);
            }
            DeviceSignal::Myo1(val) => {
                println!("Myo 1: {}", val);
            }
            DeviceSignal::Myo2(val) => {
                println!("Myo 2: {}", val);
            }
        }
    }

    // Join all the threads - waits for anything they need to clean up to finish before we do any cleanup from the main thread
    eeg_join.join().expect("EEG thread failed to join");
    myo1_join.join().expect("Myo 1 thread failed to join");
    myo2_join.join().expect("Myo 2 thread failed to join");

    // Cleanup phase

    Ok(())
}
