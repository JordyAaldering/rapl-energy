mod libc;

mod msr;
mod rapl;
#[cfg(feature = "http")] mod http;
#[cfg(feature = "hwmon")] mod hwmon;
#[cfg(feature = "nvml")] mod nvml;

pub use msr::Msr;
pub use rapl::Rapl;
#[cfg(feature = "http")] pub use http::Http;
#[cfg(feature = "hwmon")] pub use hwmon::Hwmon;
#[cfg(feature = "nvml")] pub use nvml::Nvml;

use indexmap::IndexMap;

pub trait Energy {
    fn elapsed(&self) -> IndexMap<String, f64>;

    fn elapsed_mut(&mut self) -> IndexMap<String, f64>;
}
