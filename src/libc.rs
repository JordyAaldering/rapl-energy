use std::{ffi::{c_char, CString}, ptr};

use crate::*;

type RaplEnergy = Option<*mut dyn Energy>;

#[repr(C)]
struct RaplElapsed {
    keys: *const *const c_char,
    energy: *const f64,
    len: usize,
}

impl RaplElapsed {
    fn default() -> Self {
        RaplElapsed {
            keys: ptr::null(),
            energy: ptr::null(),
            len: 0,
        }
    }

    fn from(map: IndexMap<String, f64>) -> Self {
        let len = map.len();

        let (keys, values): (Vec<String>, Vec<f64>) = map.into_iter().unzip();

        let cstr_vec: Vec<CString> = keys.into_iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
        let cptr_vec: Vec<*const c_char> = cstr_vec.iter().map(|s| s.as_ptr()).collect();

        let res = RaplElapsed {
            keys: cptr_vec.as_ptr(),
            energy: values.as_ptr(),
            len,
        };

        std::mem::forget(cptr_vec);
        std::mem::forget(values);
        res
    }
}

#[no_mangle]
extern "C" fn rapl_start() -> *mut RaplEnergy {
    let rapl = Rapl::now().map(Box::into_raw);
    Box::into_raw(Box::new(rapl))
}

#[no_mangle]
extern "C" fn rapl_elapsed(energy: &mut RaplEnergy) -> *mut RaplElapsed {
    let energy = unsafe { std::ptr::read(energy) };

    if let Some(energy) = energy {
        let energy = unsafe { Box::from_raw(energy) };
        let elapsed = energy.elapsed();
        std::mem::forget(energy);

        Box::into_raw(Box::new(RaplElapsed::from(elapsed)))
    } else {
        Box::into_raw(Box::new(RaplElapsed::default()))
    }
}

#[no_mangle]
extern "C" fn elapsed_free(elapsed: &mut RaplElapsed) {
    let energy = unsafe { std::ptr::read(elapsed) };
    let keys: Vec<*mut c_char> = unsafe { Vec::from_raw_parts(energy.keys as *mut *mut c_char, energy.len, energy.len) };
    let vals: Vec<f64> = unsafe { Vec::from_raw_parts(energy.energy as *mut f64, energy.len, energy.len) };
    drop(keys);
    drop(vals);
    drop(energy);
}

#[no_mangle]
extern "C" fn rapl_free(energy: &mut RaplEnergy) {
    let energy = unsafe { std::ptr::read(energy) };
    if let Some(energy) = energy {
        let energy = unsafe { Box::from_raw(energy) };
        drop(energy);
    }
}
