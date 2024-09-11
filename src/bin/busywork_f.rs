use std::{process::Command, sync::mpsc::{self, Sender}, thread};

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

fn start_busywork(step: usize, max_threads: usize) -> Vec<Sender<()>> {
    core_affinity::get_core_ids().unwrap()
        .into_iter()
        .rev()
        .step_by(step)
        .take(max_threads)
        .map(|id| {
            println!("Looping thread {:?}", id);
            let (tx, rx) = mpsc::channel();
            let _handle = thread::spawn(move || {
                let res = core_affinity::set_for_current(id);
                assert!(res);
                runner(rx)
            });
            tx
        }).collect()
}

fn stop_busywork(txs: Vec<Sender<()>>) {
    println!("Stopping {} busy threads", txs.len());
    for tx in txs {
        tx.send(()).unwrap()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <step> <max_threads> <script> [arguments...]", args[0]);
        return;
    }

    let step = args[1].parse::<usize>().unwrap();
    let max_threads = args[2].parse::<usize>().unwrap();
    let mut cmd = Command::new(&args[3]);
    cmd.args(&args[4..]);

    let senders = start_busywork(step, max_threads);

    match cmd.spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => eprintln!("Command exited with status: {:?}", status),
                Err(e) => eprintln!("Failed to wait on child process: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to start command: {}", e),
    }

    stop_busywork(senders);
}
