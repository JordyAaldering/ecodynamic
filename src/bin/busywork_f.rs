use std::{process::Command, sync::mpsc::{self, Sender}, thread, time::Instant};

use cpu_time::ProcessTime;

fn runner(rx: mpsc::Receiver<()>) {
    loop {
        match rx.try_recv() {
            Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                // Do nothing
            }
        }
    }
}

fn start_busywork(max_threads: usize) -> Vec<Sender<()>> {
    (0..max_threads).map(|_| {
        let (tx, rx) = mpsc::channel();
        let _handle = thread::spawn(move || runner(rx));
        tx
    }).collect()
}

fn stop_busywork(txs: Vec<Sender<()>>) {
    //println!("Stopping {} busy threads", txs.len());
    for tx in txs {
        tx.send(()).unwrap()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <max_threads> <script> [arguments...]", args[0]);
        return;
    }

    let max_threads = args[1].parse::<usize>().unwrap();
    let mut cmd = Command::new(&args[2]);
    cmd.args(&args[3..]);

    let senders = if max_threads > 0 {
        Some(start_busywork(max_threads))
    } else {
        None
    };

    match cmd.spawn() {
        Ok(mut child) => {
            let user = ProcessTime::now();
            let real = Instant::now();

            match child.wait() {
                Err(e) => eprintln!("Failed to wait on child process: {}", e),
                Ok(_) => {},
            }

            let real = real.elapsed();
            let user = user.elapsed();
            println!(",{:.8},{:.8}", real.as_secs_f64(), user.as_secs_f64());
        }
        Err(e) => eprintln!("Failed to start command: {}", e),
    }

    if let Some(senders) = senders {
        stop_busywork(senders);
    }
}
