use sysinfo::{ System };
use std::{ thread, time};
use itertools::Itertools;

pub fn get_cpu_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("Global cpu usage: {:?}", sys.global_cpu_usage());
    println!("Total memory: {:?}", sys.total_memory());
    println!("available_memory: {:?}", sys.available_memory());
    println!("free_memory: {:?}", sys.free_memory());
    println!("used_memory: {:?}", sys.used_memory());

    thread::sleep(time::Duration::from_millis(100 * sys.cpus().len() as u64));
    sys.refresh_all();

    for process in sys.processes().iter()
        .sorted_by(|a, b
                | a.1.cpu_usage().partial_cmp(&b.1.cpu_usage()).
                unwrap_or(std::cmp::Ordering::Equal)) {
        println!("process: {:?}, {:?}", process.1.name(), process.1.cpu_usage());
   }
}   

mod test;