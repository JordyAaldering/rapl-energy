use crate::*;

//type EnergyC = Option<Box<*mut dyn Energy>>;
type EnergyC = Option<*mut dyn Energy>;

#[no_mangle]
pub extern "C" fn start_msr() -> *mut EnergyC {
    let msr = Msr::now()
        .map(Box::into_raw)
        ;//.map(Box::new);
    Box::into_raw(Box::new(msr))
}

#[no_mangle]
pub extern "C" fn start_rapl() -> *mut EnergyC {
    let rapl = Rapl::now()
        .map(Box::into_raw)
        ;//.map(Box::new);
    Box::into_raw(Box::new(rapl))
}

#[cfg(feature = "http")]
#[no_mangle]
pub extern "C" fn start_ina() -> *mut EnergyC {
    let path = std::env::var("ENERGY_STATS").unwrap();
    let header = "X-Electricity-Consumed-Total".to_string();
    let ina = Http::now(path, header)
        .map(Box::into_raw)
        ;//.map(Box::new);
    Box::into_raw(Box::new(ina))
}

#[no_mangle]
pub extern "C" fn elapsed(energy: &mut EnergyC, elapsed_out: *mut *mut f64) -> usize {
    let energy = unsafe { std::ptr::read(energy) };
    if let Some(energy) = energy {
        let energy = unsafe { Box::from_raw(energy) };
        let elapsed = energy.elapsed();
        std::mem::forget(energy);

        let size = elapsed.len();

        let mut elapsed = elapsed.into_values().collect::<Vec<f64>>();
        unsafe {
            *elapsed_out = elapsed.as_mut_ptr();
        }

        size
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn print_energy(energy: &mut EnergyC) {
    let energy = unsafe { std::ptr::read(energy) };
    if let Some(energy) = energy {
        let energy = unsafe { Box::from_raw(energy) };
        let elapsed = energy.elapsed();
        std::mem::forget(energy);

        println!("{}", elapsed.values()
            .map(f64::to_string)
            .collect::<Vec<String>>()
            .join(", "));
    }
}

#[no_mangle]
pub extern "C" fn free_energy(energy: &mut EnergyC) {
    let energy = unsafe { std::ptr::read(energy) };
    if let Some(energy) = energy {
        let energy = unsafe { Box::from_raw(energy) };
        drop(energy);
    }
}
