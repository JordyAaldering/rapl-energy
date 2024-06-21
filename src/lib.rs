mod rapl_amd;
mod rapl_intel;

pub use rapl_amd::RaplAMD;
pub use rapl_intel::RaplIntel;

pub trait RaplReader {
    /// Creates a new RAPL reader for the given CPU package id.
    fn now(package_id: u8) -> Option<Self> where Self: Sized;

    /// Returns the energy elapsed in nano-Joules since this RAPL reader was created.
    fn elapsed(&self) -> u64;
}

/// Returns a RAPL reader for all CPU packages in the system.
pub fn packages<T: RaplReader>() -> Vec<T> {
    (0..u8::MAX).map_while(T::now).collect()
}

/// Returns the energy elapsed of all CPU packages in the system.
pub fn elapsed<T: RaplReader>(packages: &Vec<T>) -> Vec<u64> {
    packages.iter().map(|package| package.elapsed()).collect()
}
