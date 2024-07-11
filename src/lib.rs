pub mod msr;
pub mod rapl;
#[cfg(feature = "url")]
pub mod url;

use std::time::Duration;

pub use msr::{Msr, MsrEnergy};
pub use rapl::{Rapl, RaplEnergy};
#[cfg(feature = "url")]
pub use url::Url;

pub type Serializable = dyn erased_serde::Serialize;

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

    pub fn elapsed(&self) -> Box<Serializable> {
        match self {
            Energy::Msr(msr) => Box::new(msr.elapsed()),
            Energy::Rapl(rapl) => Box::new(rapl.elapsed()),
            #[cfg(feature = "url")]
            Energy::Url(url) => Box::new(url.elapsed()),
        }
    }

    pub fn power(&mut self, duration: Duration) -> Box<Serializable> {
        match self {
            Energy::Msr(msr) => Box::new(msr.power(duration)),
            Energy::Rapl(rapl) => Box::new(rapl.power(duration)),
            #[cfg(feature = "url")]
            Energy::Url(url) => Box::new(url.power(duration)),
        }
    }
}

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

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .terminator(csv::Terminator::Any(',' as u8))
        .from_writer(std::io::stdout());
    wtr.serialize(elapsed).unwrap();
    wtr.flush().unwrap();
}
