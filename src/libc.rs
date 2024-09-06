use crate::*;

#[no_mangle]
pub extern "C" fn start_msr(msr_out: *mut Box<dyn Energy>) {
    let msr = Msr::now().unwrap();
    unsafe {
        *msr_out = msr;
    }
}

#[no_mangle]
pub extern "C" fn start_rapl(rapl_out: *mut Box<dyn Energy>) {
    let rapl = Rapl::now().unwrap();
    unsafe {
        *rapl_out = rapl;
    }
}

#[cfg(feature = "http")]
#[no_mangle]
pub extern "C" fn start_ina(ina_out: *mut Box<dyn Energy>) {
    let url = std::env::var("ENERGY_STATS").unwrap();
    let header = "X-Electricity-Consumed-Total".to_string();
    let ina = Http::now(url, header).unwrap();
    unsafe {
        *ina_out = ina;
    }
}

#[no_mangle]
pub extern "C" fn elapsed(energy: *mut Box<dyn Energy>, elapsed_out: *mut *mut f64) -> usize {
    if energy.is_null() {
        eprintln!("nullptr");
        return 0;
    }

    let energy = unsafe { Box::from_raw(energy) };
    let elapsed = energy.elapsed();
    let size = elapsed.len();

    let mut elapsed = elapsed.into_values().collect::<Vec<f64>>();
    unsafe {
        *elapsed_out = elapsed.as_mut_ptr();
    }

    std::mem::forget(energy);
    std::mem::forget(elapsed);
    size
}

#[no_mangle]
pub extern "C" fn print_energy(energy: *mut Box<dyn Energy>) {
    if energy.is_null() {
        eprintln!("nullptr");
        return;
    }

    let energy = unsafe { Box::from_raw(energy) };
    let elapsed = energy.elapsed();
    std::mem::forget(energy);

    println!("{}", elapsed.values()
                          .map(f64::to_string)
                          .collect::<Vec<String>>()
                          .join(", "));
}

#[no_mangle]
pub extern "C" fn free_energy(energy: *mut Box<dyn Energy>) {
    if energy.is_null() {
        eprintln!("nullptr");
        return;
    }

    let energy = unsafe { Box::from_raw(energy) };
    drop(energy);
}
