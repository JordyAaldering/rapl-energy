mod file_handle;
mod libc;
mod probes;

use std::iter::once;

use indexmap::IndexMap;

pub use probes::*;

pub trait Energy {
    /// Gets the total amount of energy consumed in Joules since the last time
    /// this energy probe was created/reset.
    fn elapsed(&self) -> IndexMap<String, f32>;

    /// Resets this energy probe, such that the next time `elapsed` is called,
    /// the total amount of energy since this reset is returned.
    fn reset(&mut self);
}

/// Creates a chain of energy probes, requiring that at least one exists.
fn chain<T>(new: fn (u8) -> Option<T>) -> Option<Vec<T>> {
    Some(once(new(0)?).chain((1..u8::MAX).map_while(new)).collect())
}
