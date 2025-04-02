use std::{ffi, mem};

use crate::*;

#[repr(C)]
struct RaplElapsed {
    keys: *const *mut ffi::c_char,
    values: *const f32,
    len: usize,
}

impl RaplElapsed {
    fn from(elapsed: Elapsed) -> Self {
        let len = elapsed.len();

        let (keys, mut values): (Vec<String>, Vec<f32>) = elapsed.into_iter().unzip();
        values.shrink_to_fit();

        let mut cchar_vec: Vec<*mut ffi::c_char> = keys
            .into_iter()
            .map(|s| ffi::CString::new(s).unwrap().into_raw())
            .collect();
        cchar_vec.shrink_to_fit();

        let res = RaplElapsed {
            keys: cchar_vec.as_ptr(),
            values: values.as_ptr(),
            len,
        };

        mem::forget(cchar_vec);
        mem::forget(values);
        res
    }

    fn free(&self) {
        let keys = unsafe { Vec::from_raw_parts(self.keys as *mut *mut ffi::c_char, self.len, self.len) };
        for key in keys {
            let cstr = unsafe { ffi::CString::from_raw(key) };
            drop(cstr);
        }

        let values = unsafe { Vec::from_raw_parts(self.values as *mut f32, self.len, self.len) };
        drop(values);
    }
}

#[unsafe(no_mangle)]
extern "C" fn rapl_start(with_subzones: bool) -> Box<Rapl> {
    let rapl = Rapl::now(with_subzones).unwrap();
    Box::new(rapl)
}

#[unsafe(no_mangle)]
extern "C" fn rapl_elapsed(rapl: &mut Box<Rapl>) -> *mut RaplElapsed {
    let elapsed = rapl.elapsed();
    let elapsed = RaplElapsed::from(elapsed);
    Box::into_raw(Box::new(elapsed))
}

#[unsafe(no_mangle)]
extern "C" fn rapl_elapsed_free(elapsed: &mut RaplElapsed) {
    let elapsed = unsafe { Box::from_raw(elapsed) };
    elapsed.free();
    drop(elapsed);
}

#[unsafe(no_mangle)]
extern "C" fn rapl_free(rapl: Box<Rapl>) {
    drop(rapl);
}
