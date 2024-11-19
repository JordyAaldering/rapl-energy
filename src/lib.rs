mod file_handle;
mod libc;
mod probes;
#[cfg(feature = "statistics")]
mod statistics;

use indexmap::IndexMap;

pub use probes::*;
#[cfg(feature = "statistics")]
pub use statistics::*;

pub type ProbeEnergy = IndexMap<String, f32>;

pub trait Energy {
    /// Gets the total amount of energy consumed in Joules since the last time
    /// this energy probe was created/reset.
    fn elapsed(&self) -> ProbeEnergy;

    /// Resets this energy probe, such that the next time `elapsed` is called,
    /// the total amount of energy since this reset is returned.
    fn reset(&mut self);
}

/// Creates a chain of energy probes, returning None if no probes are available.
fn chain<T>(new: fn (u8) -> Option<T>) -> Option<Vec<T>> {
    let head = new(0)?;
    let tail = (1..u8::MAX).map_while(new);
    Some(std::iter::once(head).chain(tail).collect())
}
