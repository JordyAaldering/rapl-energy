use std::thread;
use std::time::Duration;

use rapl_energy::Energy;

fn main() {
    let url = std::env::var("ENERGY_STATS").unwrap();
    let header = "X-Electricity-Consumed-Total".to_string();
    let ina = Energy::url(url, header);

    thread::sleep(Duration::from_secs(1));

    let elapsed = ina.elapsed();
    println!("{:?}", elapsed);
}
