use std::thread;
use std::time::Duration;

use rapl_energy::{Energy, Rapl};

fn main() {
    let rapl = Rapl::now().unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("{:?}", rapl.elapsed());
}
