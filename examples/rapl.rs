use std::thread;
use std::time::Duration;

use rapl_energy::Energy;

fn main() {
    let rapl = Energy::rapl();

    thread::sleep(Duration::from_secs(1));

    let elapsed = rapl.elapsed();
    println!("{:?}", elapsed);
}
