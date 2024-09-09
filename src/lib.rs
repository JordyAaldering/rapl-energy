mod libc;

pub mod msr;
pub mod rapl;
#[cfg(feature = "nvml")]
pub mod nvml;
#[cfg(feature = "http")]
pub mod http;

pub use msr::Msr;
pub use rapl::Rapl;
#[cfg(feature = "nvml")]
pub use nvml::Nvml;
#[cfg(feature = "http")]
pub use http::Http;

use indexmap::IndexMap;

pub trait Energy {
    fn elapsed(&self) -> IndexMap<String, f64>;

    fn elapsed_mut(&mut self) -> IndexMap<String, f64>;
}
