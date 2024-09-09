use std::thread;
use std::time::Duration;

use rapl_energy::Nvml;

fn main() {
    let msr = Nvml::now().unwrap();

    thread::sleep(Duration::from_secs(1));

    let elapsed = msr.elapsed();
    println!("{:?}", elapsed);
}
