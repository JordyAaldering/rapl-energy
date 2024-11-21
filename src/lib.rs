mod file_handle;
mod libc;
mod probes;
#[cfg(feature = "statistics")]
mod statistics;

pub use probes::*;
#[cfg(feature = "statistics")]
pub use statistics::*;

pub type ProbeEnergy = indexmap::IndexMap<String, f32>;

pub trait Energy {
    /// Gets the total amount of energy consumed in Joules since the last time
    /// this energy probe was created/reset.
    fn elapsed(&self) -> ProbeEnergy;

    /// Resets this energy probe, such that the next time `elapsed` is called,
    /// the total amount of energy since this reset is returned.
    fn reset(&mut self);
}

/// Given a list of potential probes, filters out any probes that returned None.
pub fn get_available(potential_probes: Vec<Option<Box<dyn Energy>>>) -> Vec<Box<dyn Energy>> {
    potential_probes.into_iter()
        .filter_map(|x| x)
        .collect()
}

/// Gets the total amount of energy consumed in Joules since the last time
/// these energy probes were created/reset.
pub fn elapsed_all(probes: &Vec<Box<dyn Energy>>) -> ProbeEnergy {
    probes.iter()
        .rev()
        .map(|probe| probe.elapsed())
        .flatten()
        .collect()
}

/// Resets these energy probes, such that the next time `elapsed` is called,
/// the total amount of energy since this reset is returned.
pub fn reset_all(probes: &mut Vec<Box<dyn Energy>>) {
    probes.iter_mut()
        .for_each(|probe| probe.reset());
}

/// Creates a chain of energy probes, returning None if no probes are available.
fn chain<T>(new: fn (u8) -> Option<T>) -> Option<Vec<T>> {
    let head = new(0)?;
    let tail = (1..u8::MAX).map_while(new);
    Some(std::iter::once(head).chain(tail).collect())
}
