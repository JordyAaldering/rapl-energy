use std::{thread, time::Duration};

use rapl_energy::Rapl;

fn main() {
    let rapl = Rapl::now(true).unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("{:?}", rapl.elapsed());
}
