use rapl_energy::*;

const RAPL_NULLPTR: u32 = 1;
const RAPL_SUCCESS: u32 = 0;

#[no_mangle]
pub extern "C" fn rapl_intel_start(rapl_ptr: *mut *mut RaplVec<RaplIntel>) -> u32 {
    unsafe {
        *rapl_ptr = std::ptr::null_mut();
    }

    let packages = get_packages::<RaplIntel>();

    unsafe {
        *rapl_ptr = Box::into_raw(Box::new(packages));
    }

    RAPL_SUCCESS
}

#[no_mangle]
pub extern "C" fn rapl_intel_stop(rapl_ptr: *mut RaplVec<RaplIntel>, elapsed_ptr: *mut *mut RaplVec<u64>) -> u32 {
    if rapl_ptr.is_null() {
        return RAPL_NULLPTR;
    }

    let rapl = unsafe {
        Box::from_raw(rapl_ptr)
    };

    let elapsed = get_elapsed(&rapl);

    unsafe {
        *elapsed_ptr = Box::into_raw(Box::new(elapsed));
    }

    RAPL_SUCCESS
}

#[no_mangle]
pub extern "C" fn rapl_intel_free(rapl_ptr: *mut RaplVec<RaplIntel>, elapsed_ptr: *mut RaplVec<u64>) -> u32 {
    if rapl_ptr.is_null() || elapsed_ptr.is_null() {
        return RAPL_NULLPTR;
    }

    unsafe {
        drop(Box::from_raw(rapl_ptr));
        drop(Box::from_raw(elapsed_ptr));
    }

    RAPL_SUCCESS
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
