use std::thread;
use std::time::Duration;

use rapl_energy::Rapl;

fn main() {
    let rapl = Rapl::now().unwrap();

    thread::sleep(Duration::from_secs(1));

    let elapsed = rapl.elapsed();
    println!("{:?}", elapsed);
}
