use std::thread;
use std::time::Duration;

use rapl_energy::Msr;

fn main() {
    let msr = Msr::now().unwrap();

    thread::sleep(Duration::from_secs(1));

    let elapsed = msr.elapsed();
    println!("{:?}", elapsed);
}
