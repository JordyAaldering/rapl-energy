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
pub fn packages<T: RaplReader>() -> impl Iterator<Item = T> {
    (0..u8::MAX).map_while(T::now)
}

/// Returns the energy elapsed of all CPU packages in the system.
pub fn elapsed<T: RaplReader>(packages: impl Iterator<Item = T>) -> impl Iterator<Item = u64> {
    packages.map(|package| package.elapsed())
}
