mod file_handle;
mod libc;
mod probes;

pub use probes::*;

use indexmap::IndexMap;

pub trait Energy {
    /// Gets the total amount of energy consumed in Joules since the last time
    /// this energy probe was created/reset.
    fn elapsed(&self) -> IndexMap<String, f32>;

    /// Resets this energy probe, such that the next time `elapsed` is called,
    /// the total amount of energy since this reset is returned.
    fn reset(&mut self);
}
