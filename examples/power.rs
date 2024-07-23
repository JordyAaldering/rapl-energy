use std::thread;
use std::time::Duration;

use rapl_energy::Energy;

fn main() {
    const DURATION: Duration = Duration::from_millis(50);

    let mut rapl = Energy::rapl();

    for _ in 0..10 {
        thread::sleep(DURATION);

        let power = rapl.power(DURATION);

        println!("{}", power.values()
                            .map(f64::to_string)
                            .collect::<Vec<String>>()
                            .join(", "));
    }
}
