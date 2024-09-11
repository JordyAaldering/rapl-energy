use crate::*;

type EnergyC = Option<*mut dyn Energy>;

#[no_mangle]
pub extern "C" fn start_rapl() -> *mut EnergyC {
    let rapl = Rapl::now().map(Box::into_raw);
    Box::into_raw(Box::new(rapl))
}

#[no_mangle]
pub extern "C" fn start_msr() -> *mut EnergyC {
    let msr = Msr::now().map(Box::into_raw);
    Box::into_raw(Box::new(msr))
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

        println!("{:?}", elapsed);
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
