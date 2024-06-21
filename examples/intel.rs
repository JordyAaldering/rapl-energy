use std::thread;
use std::time::Duration;

use rapl_energy as rapl;

fn main() {
    let packages = rapl::packages::<rapl::RaplIntel>();
    thread::sleep(Duration::from_secs(1));
    let elapsed: Vec<u64> = rapl::elapsed(packages).collect();
    println!("{:?}", elapsed);
}
