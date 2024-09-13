use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::{Instant, Duration};
use rapl_energy::Rapl;

const SLEEP: Duration = Duration::from_millis(50);

fn runner(rx: mpsc::Receiver<()>) {
    let mut res = Vec::new();

    let instant = Instant::now();
    let mut rapl = Rapl::now();
    //let mut msr = Msr::now();

    loop {
        match rx.try_recv() {
            Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                thread::sleep(SLEEP);
                res.push((
                    instant.elapsed().as_secs_f32(),
                    rapl.power(SLEEP),
                    //msr.power(SLEEP),
                ));
            }
        }
    }

    let mut file = File::create("energy.csv").unwrap();
    let (_, rapl0) = res.first().unwrap();
    let rapl_h = rapl0.iter().map(|(s, _)| s.clone()).collect::<Vec<String>>().join(",");
    file.write_fmt(format_args!("time,{}\n", rapl_h)).unwrap();

    for (time, rapl) in res {
        let rapl_s = rapl.iter().map(|(_, uj)| uj.to_string()).collect::<Vec<String>>().join(",");
        file.write_fmt(format_args!("{},{}\n", time, rapl_s)).unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <script> [arguments...]", args[0]);
        return;
    }

    let mut cmd = Command::new(&args[1]);
    cmd.args(&args[2..]);

    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || runner(rx));

    match cmd.spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("Command exited with status: {:?}", status);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to wait on child process: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to start command: {}", e);
        }
    }

    tx.send(()).unwrap();
    handle.join().unwrap();
}
