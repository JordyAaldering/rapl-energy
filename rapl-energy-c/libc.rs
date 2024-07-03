use rapl_energy::*;

const RAPL_NULLPTR: u32 = 1;
const RAPL_SUCCESS: u32 = 0;

fn rapl_start<T: RaplReader>(rapl_ptr: *mut *mut RaplVec<T>) -> u32 {
    unsafe {
        *rapl_ptr = std::ptr::null_mut();
    }

    let packages = get_packages::<T>();

    unsafe {
        *rapl_ptr = Box::into_raw(Box::new(packages));
    }

    RAPL_SUCCESS
}

#[no_mangle]
pub extern "C" fn rapl_intel_start(rapl_ptr: *mut *mut RaplVec<RaplIntel>) -> u32 {
    rapl_start::<RaplIntel>(rapl_ptr)
}

#[no_mangle]
pub extern "C" fn rapl_amd_start(rapl_ptr: *mut *mut RaplVec<Msr>) -> u32 {
    rapl_start::<Msr>(rapl_ptr)
}

fn rapl_stop<T: RaplReader>(rapl_ptr: *mut RaplVec<T>, elapsed_ptr: *mut *mut RaplVec<u64>) -> u32 {
    if rapl_ptr.is_null() {
        return RAPL_NULLPTR;
    }

    let rapl = unsafe {
        Box::from_raw(rapl_ptr)
    };

    let elapsed = get_elapsed::<T>(&rapl);

    unsafe {
        *elapsed_ptr = Box::into_raw(Box::new(elapsed));
    }

    RAPL_SUCCESS
}

#[no_mangle]
pub extern "C" fn rapl_intel_stop(rapl_ptr: *mut RaplVec<RaplIntel>, elapsed_ptr: *mut *mut RaplVec<u64>) -> u32 {
    rapl_stop::<RaplIntel>(rapl_ptr, elapsed_ptr)
}

#[no_mangle]
pub extern "C" fn rapl_amd_stop(rapl_ptr: *mut RaplVec<Msr>, elapsed_ptr: *mut *mut RaplVec<u64>) -> u32 {
    rapl_stop::<Msr>(rapl_ptr, elapsed_ptr)
}

fn rapl_free<T: RaplReader>(rapl_ptr: *mut RaplVec<T>, elapsed_ptr: *mut RaplVec<u64>) {
    if !rapl_ptr.is_null() {
        drop(unsafe { Box::from_raw(rapl_ptr) });
    }

    if !elapsed_ptr.is_null() {
        drop(unsafe { Box::from_raw(elapsed_ptr) });
    }
}

#[no_mangle]
pub extern "C" fn rapl_intel_free(rapl_ptr: *mut RaplVec<RaplIntel>, elapsed_ptr: *mut RaplVec<u64>) {
    rapl_free::<RaplIntel>(rapl_ptr, elapsed_ptr)
}

#[no_mangle]
pub extern "C" fn rapl_amd_free(rapl_ptr: *mut RaplVec<Msr>, elapsed_ptr: *mut RaplVec<u64>) {
    rapl_free::<Msr>(rapl_ptr, elapsed_ptr)
}

#[no_mangle]
pub extern "C" fn rapl_print(elapsed_ptr: *mut RaplVec<u64>) -> u32 {
    if elapsed_ptr.is_null() {
        return RAPL_NULLPTR;
    }

    let elapsed = unsafe {
        Box::from_raw(elapsed_ptr)
    };

    println!("{:?}", elapsed);

    RAPL_SUCCESS
}
