mod energy_duration;
mod libc;

pub mod msr;
pub mod rapl;
#[cfg(feature = "http")]
pub mod http;

pub use msr::Msr;
pub use rapl::Rapl;
#[cfg(feature = "http")]
pub use http::Http;

pub use energy_duration::EnergyDuration;

use std::time::Duration;

use indexmap::IndexMap;

pub enum Energy {
    Msr(Msr),
    Rapl(Rapl),
    #[cfg(feature = "http")]
    Ureq(Http),
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

    #[cfg(feature = "http")]
    pub fn url(url: String, header: String) -> Self {
        let url = Http::now(url, header);
        Energy::Ureq(url)
    }

    pub fn elapsed(&self) -> IndexMap<String, f64> {
        match self {
            Energy::Msr(msr) => msr.elapsed(),
            Energy::Rapl(rapl) => rapl.elapsed(),
            #[cfg(feature = "http")]
            Energy::Ureq(url) => url.elapsed(),
        }
    }

    pub fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        match self {
            Energy::Msr(msr) => msr.power(duration),
            Energy::Rapl(rapl) => rapl.power(duration),
            #[cfg(feature = "http")]
            Energy::Ureq(url) => url.power(duration),
        }
    }
}
