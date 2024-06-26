mod rapl_amd;
mod rapl_intel;

pub use rapl_amd::RaplAMD;
pub use rapl_intel::RaplIntel;

use arrayvec::ArrayVec;

const MAX: usize = u8::MAX as usize;

pub type RaplVec<T> = ArrayVec<T, MAX>;

pub trait RaplReader {
    /// Creates a new RAPL reader for the given CPU package id.
    fn now(package_id: usize) -> Option<Self> where Self: Sized;

    /// Returns the energy elapsed in micro-Joules since this RAPL reader was created.
    fn elapsed(&self) -> u64;

    fn label(&self) -> String;
}

/// Returns a RAPL reader for all CPU packages in the system.
pub fn get_packages<T: RaplReader>() -> RaplVec<T> {
    (0..MAX).map_while(T::now).collect()
}

/// Returns the energy elapsed of all CPU packages in the system.
pub fn get_elapsed<T: RaplReader>(packages: &RaplVec<T>) -> RaplVec<u64> {
    packages.iter().map(|package| package.elapsed()).collect()
}
