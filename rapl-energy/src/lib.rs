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
