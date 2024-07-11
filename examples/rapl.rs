use std::thread;
use std::time::Duration;

use rapl_energy::Energy;

fn main() {
    let rapl = Energy::rapl();

    thread::sleep(Duration::from_secs(1));

    let elapsed = rapl.elapsed();

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(std::io::stdout());
    wtr.serialize(elapsed).unwrap();
    wtr.flush().unwrap();
}
