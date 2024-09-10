use std::thread;
use std::time::Duration;

use rapl_energy::Nvml;

fn main() {
    let nvml = Nvml::now().unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("{:?}", nvml.elapsed());
}
