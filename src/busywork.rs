use std::{sync::mpsc::{self, Sender}, thread};

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

fn start_busywork(num_threads: usize) -> Vec<Sender<()>> {
    println!("Starting {} busy threads", num_threads);
    (0..num_threads).map(|_| {
        let (tx, rx) = mpsc::channel();
        let _handle = thread::spawn(move || runner(rx));
        tx
    }).collect()
}

fn stop_busywork(txs: Vec<Sender<()>>) {
    println!("Stopping {} busy threads", txs.len());
    for tx in txs {
        tx.send(()).unwrap()
    }
}
