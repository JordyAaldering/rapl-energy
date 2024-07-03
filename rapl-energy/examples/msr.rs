use std::thread;
use std::time::Duration;

use rapl_energy::Energy;

fn main() {
    let msr = Energy::msr();
    thread::sleep(Duration::from_secs(1));
    let elapsed = msr.elapsed();
    println!("{:?}", serde_json::to_string(&elapsed));
}
