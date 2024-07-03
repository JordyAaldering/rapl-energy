mod msr;
mod rapl;

pub enum Energy {
    MSR(msr::Msr),
    Rapl(rapl::Rapl),
}

impl Energy {
    pub fn msr() -> Self {
        let msr = msr::Msr::now();
        Energy::MSR(msr)
    }

    pub fn rapl() -> Self {
        let rapl = rapl::Rapl::now();
        Energy::Rapl(rapl)
    }

    pub fn elapsed(&self) -> impl serde::Serialize {
        match self {
            Energy::MSR(msr) => msr.elapsed(),
            Energy::Rapl(rapl) => rapl.elapsed(),
        }
    }
}
