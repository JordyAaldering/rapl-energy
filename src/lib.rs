pub mod msr;
pub mod rapl;
#[cfg(feature = "url")]
pub mod url;

pub use msr::Msr;
pub use rapl::Rapl;
#[cfg(feature = "url")]
pub use url::Url;

use indexmap::IndexMap;
use std::time::Duration;

pub enum Energy {
    Msr(Msr),
    Rapl(Rapl),
    #[cfg(feature = "url")]
    Url(Url),
}

impl Energy {
    pub fn msr() -> Self {
        let msr = Msr::now();
        Energy::Msr(msr)
    }

    pub fn rapl() -> Self {
        let rapl = Rapl::now();
        Energy::Rapl(rapl)
    }

    #[cfg(feature = "url")]
    pub fn url(url: String, header: String) -> Self {
        let url = Url::now(url, header);
        Energy::Url(url)
    }

    pub fn elapsed(&self) -> IndexMap<String, f64> {
        match self {
            Energy::Msr(msr) => msr.elapsed(),
            Energy::Rapl(rapl) => rapl.elapsed(),
            #[cfg(feature = "url")]
            Energy::Url(url) => url.elapsed(),
        }
    }

    pub fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        match self {
            Energy::Msr(msr) => msr.power(duration),
            Energy::Rapl(rapl) => rapl.power(duration),
            #[cfg(feature = "url")]
            Energy::Url(url) => url.power(duration),
        }
    }
}

#[no_mangle]
pub extern "C" fn msr(msr_out: *mut *mut Energy) {
    let msr = Box::into_raw(Box::new(Energy::msr()));
    unsafe {
        *msr_out = msr;
    }
}

#[no_mangle]
pub extern "C" fn rapl(rapl_out: *mut *mut Energy) {
    let rapl = Box::into_raw(Box::new(Energy::rapl()));
    unsafe {
        *rapl_out = rapl;
    }
}

#[cfg(feature = "url")]
#[no_mangle]
pub extern "C" fn ina(ina_out: *mut *mut Energy) {
    let url = std::env::var("ENERGY_STATS").unwrap();
    let header = "X-Electricity-Consumed-Total".to_string();
    let ina = Box::into_raw(Box::new(Energy::url(url, header)));
    unsafe {
        *ina_out = ina;
    }
}

#[no_mangle]
pub extern "C" fn elapsed(energy: *mut Energy, elapsed_out: *mut *mut f64) -> usize {
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

    size
}

#[no_mangle]
pub extern "C" fn print_energy(energy: *mut Energy) {
    if energy.is_null() {
        eprintln!("nullptr");
        return;
    }

    let energy = unsafe { Box::from_raw(energy) };
    let elapsed = energy.elapsed();

    println!("{}", elapsed.values()
                          .map(f64::to_string)
                          .collect::<Vec<String>>()
                          .join(", "));
}
