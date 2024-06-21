mod rapl_amd;
mod rapl_intel;


pub use rapl_amd::RaplAMD;
pub use rapl_intel::RaplIntel;

use arrayvec::ArrayVec;

pub const MAX: usize = u8::MAX as usize;

pub trait RaplReader : std::fmt::Debug {
    /// Creates a new RAPL reader for the given CPU package id.
    fn now(package_id: usize) -> Option<Self> where Self: Sized;

    /// Returns the energy elapsed in micro-Joules since this RAPL reader was created.
    fn elapsed(&self) -> u64;
}

/// Returns a RAPL reader for all CPU packages in the system.
pub fn packages<T: RaplReader>() -> ArrayVec<T, MAX> {
    (0..MAX).map_while(T::now).collect()
}

/// Returns the energy elapsed of all CPU packages in the system.
pub fn elapsed<T: RaplReader>(packages: &ArrayVec<T, MAX>) -> ArrayVec<u64, MAX> {
    packages.iter().map(|package| package.elapsed()).collect()
}
