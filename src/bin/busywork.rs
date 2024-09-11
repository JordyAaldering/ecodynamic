use std::{process::Command, sync::mpsc::{self, Sender}, thread::{self, sleep}, time::{Duration, Instant}};

fn runner(tid: usize, rx: mpsc::Receiver<()>) {
    let sleep_duration = Duration::from_secs(2u64.pow(4 + (tid as u32 % 3)));

    loop {
        println!("Thread {} sleeping for {}s", tid, sleep_duration.as_secs());

        sleep(sleep_duration);

        println!("Thread {} working for {}s", tid, sleep_duration.as_secs());

        let now = Instant::now();
        while now.elapsed() < sleep_duration {
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
}

fn start_busywork(max_threads: usize) -> Vec<Sender<()>> {
    println!("Starting {} busy threads", max_threads);
    core_affinity::get_core_ids().unwrap()
        .into_iter()
        .rev()
        .take(max_threads)
        .enumerate()
        .map(|(idx, id)| {
            let (tx, rx) = mpsc::channel();
            let _handle = thread::spawn(move || {
                let res = core_affinity::set_for_current(id);
                assert!(res);
                runner(idx, rx)
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
    if args.len() < 2 {
        eprintln!("Usage: {} <max_threads> <script> [arguments...]", args[0]);
        return;
    }

    let max_threads = args[1].parse::<usize>().unwrap();
    let mut cmd = Command::new(&args[2]);
    cmd.args(&args[3..]);

    let senders = start_busywork(max_threads);

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
