use rapl_energy::Energy;

#[no_mangle]
pub extern "C" fn start_msr(msr_out: *mut *mut Energy) {
    unsafe {
        *msr_out = Box::into_raw(Box::new(Energy::msr()));
    }
}

#[no_mangle]
pub extern "C" fn start_rapl(rapl_out: *mut *mut Energy) {
    unsafe {
        *rapl_out = Box::into_raw(Box::new(Energy::rapl()));
    }
}

#[no_mangle]
pub extern "C" fn print_energy(energy_in: *mut Energy) {
    if energy_in.is_null() {
        println!("nullptr");
        return;
    }

    let energy = unsafe { Box::from_raw(energy_in) };
    let elapsed = energy.elapsed();
    println!("{}", serde_json::to_string_pretty(&elapsed).unwrap());
}

#[no_mangle]
pub extern "C" fn free_energy(energy_in: *mut Energy) {
    if energy_in.is_null() {
        println!("nullptr");
        return;
    }

    let energy = unsafe { Box::from_raw(energy_in) };
    drop(energy);
}
