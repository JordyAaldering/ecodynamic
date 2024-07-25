pub struct Controller {
    max_threads: i32,
    n: i32,
    t1: u64,
    t_last: u64,
    step_size: i32,
    step_direction: i32,
    corridor_scale: u64,
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            max_threads,
            n: max_threads,
            t1: 0,
            t_last: 0,
            step_size: 8,
            step_direction: -1,
            corridor_scale: 2,
        }
    }

    pub fn init(&mut self, runtime_results: &Vec<u64>) {
        let tn = freq_dist_best(runtime_results);
        self.t1 = tn * self.n as u64;
        self.t_last = tn;
    }

    pub fn adjust_threads(&mut self, runtime_results: &Vec<u64>) -> i32 {
        self.n += self.step_direction * self.step_size;
        self.n = i32::clamp(self.n, 1, self.max_threads);
        let tn = freq_dist_best(runtime_results);

        let improvement = self.t1 / tn;
        if improvement < self.n as u64 / self.corridor_scale {
            self.step_direction = -1;
            self.step_size = i32::max(self.n / 2, 1);
        } else {
            if improvement > self.n as u64 {
                self.t1 = tn * self.n as u64;
            }

            if tn > self.t_last {
                self.step_direction = -self.step_direction;
            }

            self.step_size = i32::max(self.step_size / 2, 1);
        }

        self.t_last = tn;
        self.n
    }
}

fn freq_dist_best(runtime_results: &Vec<u64>) -> u64 {
    let min = *runtime_results.iter().filter(|&&x| x > 0).min().unwrap();
    let max = *runtime_results.iter().filter(|&&x| x > 0).max().unwrap();
    let dist_size = (max - min) / 5;

    let dist_max = vec![
        min + dist_size * 1,
        min + dist_size * 2,
        min + dist_size * 3,
        min + dist_size * 4,
        min + dist_size * 5,
    ];
    let mut distributions = vec![Vec::<u64>::new(); 5];
    for &x in runtime_results {
        for (i, &dmax) in dist_max.iter().enumerate() {
            if x > 0 && x < dmax {
                distributions[i].push(x);
                break;
            }
        }
    }

    println!("{:?}", distributions.iter().map(|x| x.len()).collect::<Vec<usize>>());
    let biggest = distributions.iter().max_by_key(|x| x.len()).unwrap();
    println!("{:?}", &biggest);
    let best = *biggest.iter().min().unwrap();
    println!("{}", best);

    best
}

#[no_mangle]
pub extern "C" fn MTDcreateController(num_threads: i32, controller_out: *mut *mut Controller) {
    let controller = Controller::new(num_threads);
    unsafe {
        *controller_out = Box::into_raw(Box::new(controller));
    }
}

#[no_mangle]
pub extern "C" fn MTDupdateController(controller: *mut &mut Controller, letterbox: *mut &mut crate::Letterbox) {
    let letterbox = unsafe { std::ptr::read(letterbox) };

    if letterbox.runtimes.len() >= 20 {
        let controller = unsafe { std::ptr::read(controller) };

        if controller.t1 == 0 {
            controller.init(&letterbox.runtimes);
            println!("Controller init");
        } else {
            let num_threads = controller.adjust_threads(&letterbox.runtimes);
            println!("Controller num threads from {} to {}", letterbox.num_threads, num_threads);
            letterbox.num_threads = num_threads;
        }

        letterbox.runtimes.clear();
    }
}
