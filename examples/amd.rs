use std::thread;
use std::time::Duration;

use rapl_energy::{EnergyProbe, MsrAmd};

fn main() {
    let msr = MsrAmd::now().unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("{:?}", msr.elapsed());
}
