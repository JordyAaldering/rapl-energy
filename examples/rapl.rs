use std::thread;
use std::time::Duration;

use rapl_energy::{EnergyProbe, Rapl};

fn main() {
    let rapl = Rapl::now(true).unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("{:?}", rapl.elapsed());
}
