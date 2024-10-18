mod mtd;
mod letterbox;
mod controller;
mod controller_energy;
mod controller_runtime;
mod controller_fixed;

//use std::{ffi::{c_char, CStr}, fs, io::Write, path::Path};

pub use mtd::*;

/*
#[no_mangle]
extern "C" fn MTDcreate(max_threads: i32, num_measurements_per_adjustment: usize, mtd_out: *mut *mut MTDynamic) {
    let mtd = MtdBuilder::new(max_threads)
        .letterbox_size(num_measurements_per_adjustment)
        .build();
    unsafe {
        *mtd_out = Box::into_raw(Box::new(mtd));
    }
}

#[no_mangle]
extern "C" fn MTDupdate(mtd: *mut &mut MTDynamic, funname: *const c_char, runtime: f64, usertime: f64, energy: f64) {
    if energy == 0.0 {
        return;
    }

    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();
    mtd.update(funname, runtime, usertime, energy);
}

#[no_mangle]
extern "C" fn MTDnumThreads(mtd: *mut &mut MTDynamic, funname: *const c_char) -> i32 {
    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();
    mtd.num_threads(funname)
}

#[no_mangle]
extern "C" fn MTDfree(mtd: *mut MTDynamic) {
    let mtd = unsafe { std::ptr::read(mtd) };

    fs::create_dir_all("mtd").unwrap();
    let date = chrono::offset::Local::now();

    for (name, (_controller, letterbox)) in &mtd.controllers {
        if letterbox.history.len() > 10 {
            print!("{},{:?},", name, letterbox);

            let filename = format!("{}-{}.csv", name, date.format("%Y-%m-%d-%H-%M-%S"));
            if let Ok(mut file) = fs::File::create(Path::new("mtd").join(filename)) {
                file.write("energy,runtime,usertime,threads\n".as_bytes()).unwrap();
                for sample in &letterbox.history {
                    file.write_fmt(format_args!("{:?}\n", sample)).unwrap();
                }
            }
        }
    }

    println!();

    drop(mtd);
}
*/
