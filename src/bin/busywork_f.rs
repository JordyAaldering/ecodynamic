use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread::{self, JoinHandle};

fn runner(rx: Receiver<()>) {
    loop {
        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {}
        }
    }
}

fn start_busywork(max_threads: usize) -> Vec<(Sender<()>, JoinHandle<()>)> {
    core_affinity::get_core_ids()
        .unwrap()
        .into_iter()
        .take(max_threads)
        .map(|core_id| {
            assert!(core_affinity::set_for_current(core_id));
            let (sender, receiver) = mpsc::channel();
            let handle = thread::spawn(move || runner(receiver));
            (sender, handle)
        }).collect()
}

fn stop_busywork(threads: Vec<(Sender<()>, JoinHandle<()>)>) {
    for (sender, handle) in threads {
        sender.send(()).unwrap();
        handle.join().unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <max_threads> <script> [arguments...]", args[0]);
        return;
    }

    let max_threads = args[1].parse::<usize>().unwrap();
    let mut cmd = Command::new(&args[2]);
    cmd.args(&args[3..]);

    let threads = start_busywork(max_threads);

    match cmd.spawn() {
        Ok(mut child) => {
            match child.wait() {
                Err(e) => unreachable!("Failed to wait on child process: {}", e),
                Ok(_) => {},
            }
        }
        Err(e) => unreachable!("Failed to start command: {}", e),
    };

    stop_busywork(threads);
}
