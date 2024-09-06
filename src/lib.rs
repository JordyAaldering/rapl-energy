mod libc;

pub mod msr;
pub mod rapl;
#[cfg(feature = "http")]
pub mod http;

pub use msr::Msr;
pub use rapl::Rapl;
#[cfg(feature = "http")]
pub use http::Http;

use std::time::Duration;

use indexmap::IndexMap;

pub trait Energy {
    fn elapsed(&self) -> IndexMap<String, f64>;

    fn power(&mut self, duration: Duration) -> IndexMap<String, f64>;
}
