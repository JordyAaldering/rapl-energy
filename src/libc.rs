use std::ffi;

use crate::*;

#[repr(C)]
struct EnergyElapsed {
    keys: *const *mut ffi::c_char,
    energy: *const f32,
    len: usize,
}

impl EnergyElapsed {
    fn from(energy: ProbeEnergy) -> Self {
        let len = energy.len();

        let (keys, mut values): (Vec<String>, Vec<f32>) = energy.into_iter().unzip();
        values.shrink_to_fit();

        let mut cchar_vec: Vec<*mut ffi::c_char> = keys
            .into_iter()
            .map(|s| ffi::CString::new(s).unwrap().into_raw())
            .collect();
        cchar_vec.shrink_to_fit();

        let res = EnergyElapsed {
            keys: cchar_vec.as_ptr(),
            energy: values.as_ptr(),
            len,
        };

        std::mem::forget(cchar_vec);
        std::mem::forget(values);
        res
    }

    fn free(&self) {
        let keys = unsafe { Vec::from_raw_parts(self.keys as *mut *mut ffi::c_char, self.len, self.len) };
        for key in keys {
            let cstr = unsafe { ffi::CString::from_raw(key) };
            drop(cstr);
        }

        let energy = unsafe { Vec::from_raw_parts(self.energy as *mut f32, self.len, self.len) };
        drop(energy);
    }
}

#[no_mangle]
extern "C" fn rapl_start() -> Box<Rapl> {
    let rapl = Rapl::now().unwrap();
    Box::new(rapl)
}

#[no_mangle]
extern "C" fn rapl_elapsed(rapl: &mut Box<Rapl>) -> *mut EnergyElapsed {
    let elapsed = rapl.elapsed();
    let elapsed = EnergyElapsed::from(elapsed);
    Box::into_raw(Box::new(elapsed))
}

#[no_mangle]
extern "C" fn elapsed_free(elapsed: &mut EnergyElapsed) {
    let elapsed = unsafe { Box::from_raw(elapsed) };
    elapsed.free();
    drop(elapsed);
}

#[no_mangle]
extern "C" fn rapl_free(rapl: Box<Rapl>) {
    drop(rapl);
}
