use std::thread;
use std::time::Duration;

use rapl_energy::Energy;

fn main() {
    const DURATION: Duration = Duration::from_millis(50);

    let mut msr = Energy::msr();

    for _ in 0..10 {
        thread::sleep(DURATION);
        let power = msr.power(DURATION);
        println!("{}", serde_json::to_string(&power).unwrap());
    }
}
