mod mtd;
mod sample;
mod letterbox;
mod selection;
mod controller;

use std::{collections::HashMap, ffi::{c_char, CStr}, fs, io::Write, path::Path};

pub use mtd::Mtd;

#[repr(C)]
struct MTDs {
    max_threads: usize,
    samples_per_update: usize,
    mtds: HashMap<String, (Mtd, Vec<(f32, f32)>)>,
}

#[no_mangle]
extern "C" fn MTDcreate(max_threads: usize, samples_per_update: usize) -> *mut MTDs {
    let mtds = MTDs { max_threads, samples_per_update, mtds: HashMap::new() };
    Box::into_raw(Box::new(mtds))
}

#[no_mangle]
extern "C" fn MTDstart(mtd: *mut &mut MTDs, funname: *const c_char) {
    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();

    if !mtd.mtds.contains_key(&funname) {
        let controller = Mtd::energy_controller(mtd.max_threads, mtd.samples_per_update);
        mtd.mtds.insert(funname.clone(), (controller, Vec::new()));
    }

    let (controller, _) = mtd.mtds.get_mut(&funname).unwrap();
    controller.sample.start();
}

#[no_mangle]
extern "C" fn MTDstop(mtd: *mut &mut MTDs, funname: *const c_char) {
    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();

    let (controller, history) = mtd.mtds.get_mut(&funname).unwrap();

    let sample = controller.sample.stop();
    history.push((sample, controller.num_threads));
    controller.update(sample);
}

#[no_mangle]
extern "C" fn MTDnumThreads(mtd: *mut &mut MTDs, funname: *const c_char) -> i32 {
    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();

    if let Some((controller, _)) = mtd.mtds.get_mut(&funname) {
        controller.num_threads()
    } else {
        mtd.max_threads as i32
    }
}

#[no_mangle]
extern "C" fn MTDfree(mtd: *mut MTDs) {
    let mtd = unsafe { std::ptr::read(mtd) };

    let date = chrono::offset::Local::now();

    for (name, (_, history)) in mtd.mtds {
        if history.len() > 10 {
            fs::create_dir_all("mtd").unwrap();
            let filename = format!("{}-{}.csv", name, date.format("%Y-%m-%d-%H-%M-%S"));
            if let Ok(mut file) = fs::File::create(Path::new("mtd").join(filename)) {
                file.write("sample,thread_count\n".as_bytes()).unwrap();
                for (sample, thread_count) in &history {
                    file.write_fmt(format_args!("{},{}\n", sample, thread_count)).unwrap();
                }
            }

            let n = history.len() as f32;
            let samples: Vec<f32> = history.into_iter().map(|(x, _)| x).collect();
            let total: f32 = samples.into_iter().sum();
            println!("{},{},{}", name, total, total / n);
        }
    }
}
