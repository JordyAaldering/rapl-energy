mod libc;
mod file_handle;
mod probes;

pub use probes::*;

pub type Probes = Vec<Box<dyn Probe>>;

pub type Elapsed = indexmap::IndexMap<String, f32>;

pub trait Probe {
    /// Gets the difference in value since the last
    /// time this energy probe was created/reset.
    fn elapsed(&self) -> Elapsed;

    /// Resets this probe, such that the next time `elapsed` is called,
    /// the difference compared to the value at this reset is returned.
    fn reset(&mut self);
}

impl Probe for Probes {
    /// Get values of all probes.
    fn elapsed(&self) -> Elapsed {
        self.iter()
            .rev()
            .map(|probe| probe.elapsed())
            .flatten()
            .collect()
    }

    /// Reset all probes.
    fn reset(&mut self) {
        self.iter_mut().for_each(|probe| probe.reset())
    }
}
