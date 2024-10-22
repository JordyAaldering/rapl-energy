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
    /// Gets the total amount of energy consumed in Joules since the last time
    /// this energy probe was created/reset.
    fn elapsed(&self) -> IndexMap<String, f32>;

    /// Resets this energy probe, such that the next time `elapsed` is called,
    /// the total amount of energy since this reset is returned.
    fn reset(&mut self);
}
