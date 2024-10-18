use std::{ffi, ptr};

use crate::*;

type RaplEnergy = Option<*mut dyn Energy>;

#[repr(C)]
struct RaplElapsed {
    keys: *const *mut ffi::c_char,
    energy: *const f32,
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

    fn from(map: IndexMap<String, f32>) -> Self {
        let len = map.len();

        let (keys, mut values): (Vec<String>, Vec<f32>) = map.into_iter().unzip();
        values.shrink_to_fit();

        let mut cchar_vec: Vec<*mut ffi::c_char> = keys
            .into_iter()
            .map(|s| ffi::CString::new(s).unwrap().into_raw())
            .collect();
        cchar_vec.shrink_to_fit();

        let res = RaplElapsed {
            keys: cchar_vec.as_ptr(),
            energy: values.as_ptr(),
            len,
        };

        std::mem::forget(cchar_vec);
        std::mem::forget(values);
        res
    }

    fn keys(&self) -> Vec<*mut ffi::c_char> {
        unsafe { Vec::from_raw_parts(self.keys as *mut *mut ffi::c_char, self.len, self.len) }
    }

    fn energy(&self) -> Vec<f32> {
        unsafe { Vec::from_raw_parts(self.energy as *mut f32, self.len, self.len) }
    }

    fn free(&self) {
        for key in self.keys() {
            let cstr = unsafe { ffi::CString::from_raw(key) };
            drop(cstr);
        }

        drop(self.energy());
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
    let elapsed = unsafe { Box::from_raw(elapsed) };
    elapsed.free();
    drop(elapsed);
}

#[no_mangle]
extern "C" fn rapl_free(energy: &mut RaplEnergy) {
    let energy = unsafe { Box::from_raw(energy) };
    if let Some(energy) = *energy {
        let energy = unsafe { Box::from_raw(energy) };
        drop(energy);
    }
}
