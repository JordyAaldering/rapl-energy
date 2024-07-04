mod msr;
mod rapl;

pub use msr::Msr;
pub use rapl::Rapl;

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
}
