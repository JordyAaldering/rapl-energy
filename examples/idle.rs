use std::thread;
use std::time::Duration;

use rapl_energy::Rapl;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <seconds> <precision>", args[0]);
        return;
    }

    let seconds: usize = args[1].parse().unwrap();
    let precision: usize = args[2].parse().unwrap();
    let sleep = Duration::from_secs_f32(1.0 / precision as f32);
    let len = seconds * precision;

    let mut energy = Vec::with_capacity(len);
    let mut rapl = Rapl::now().unwrap();

    for _ in 0..len {
        thread::sleep(sleep);
        let e = rapl.elapsed_mut()[0] * precision as f32;
        energy.push(e);
    }

    println!("{:?}", energy);
    println!("{}W", energy.into_iter().sum::<f32>() / len as f32)
}
