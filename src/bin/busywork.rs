use std::{process::Command, sync::mpsc::{self, Sender}, thread::{self, sleep, JoinHandle}, time::{Duration, Instant}};

fn runner(tid: usize, received: mpsc::Receiver<()>) {
    let sleep_duration = Duration::from_secs(2u64.pow(4 + (tid as u32 % 3)));

    loop {
        println!("Thread {} sleeping for {}s", tid, sleep_duration.as_secs());

        sleep(sleep_duration);

        println!("Thread {} working for {}s", tid, sleep_duration.as_secs());

        let now = Instant::now();
        while now.elapsed() < sleep_duration {
            match received.try_recv() {
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

fn start_busywork(max_threads: usize) -> (Vec<Sender<()>>, Vec<JoinHandle<()>>) {
    (0..max_threads).map(|tid| {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            runner(tid, receiver)
        });
        (sender, handle)
    }).unzip()
}

fn stop_busywork(senders: Vec<Sender<()>>, handles: Vec<JoinHandle<()>>) {
    senders.into_iter().for_each(|sender| sender.send(()).unwrap());
    handles.into_iter().for_each(|handle| handle.join().unwrap());
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

    let (senders, handles) = start_busywork(max_threads);

    match cmd.spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => eprintln!("Command exited with status: {:?}", status),
                Err(e) => eprintln!("Failed to wait on child process: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to start command: {}", e),
    }

    stop_busywork(senders, handles);
}
