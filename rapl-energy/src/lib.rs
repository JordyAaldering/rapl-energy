pub mod msr;
pub mod rapl;

use std::time::Duration;

pub use msr::{Msr, MsrEnergy};
pub use rapl::{Rapl, RaplEnergy};

pub type Serializable = dyn erased_serde::Serialize;

pub enum Energy {
    Msr(Msr),
    Rapl(Rapl),
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

    pub fn elapsed(&self) -> Box<Serializable> {
        match self {
            Energy::Msr(msr) => Box::new(msr.elapsed()),
            Energy::Rapl(rapl) => Box::new(rapl.elapsed()),
        }
    }

    pub fn power(&mut self, duration: Duration) -> Box<Serializable> {
        match self {
            Energy::Msr(msr) => Box::new(msr.power(duration)),
            Energy::Rapl(rapl) => Box::new(rapl.power(duration)),
        }
    }
}
