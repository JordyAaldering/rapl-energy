mod file_handle;
mod libc;
mod probes;

pub use probes::*;

pub type Probes = Vec<Box<dyn EnergyProbe>>;

pub type Energy = indexmap::IndexMap<String, f32>;

pub trait EnergyProbe {
    /// Gets the total amount of energy consumed in Joules since the last time
    /// this energy probe was created/reset.
    fn elapsed(&self) -> Energy;

    /// Resets this energy probe, such that the next time `elapsed` is called,
    /// the total amount of energy since this reset is returned.
    fn reset(&mut self);
}

impl EnergyProbe for Probes {
    /// Gets the total amount of energy consumed in Joules since
    /// the last time these energy probes were created/reset.
    fn elapsed(&self) -> Energy {
        self.iter()
            .rev()
            .map(|probe| probe.elapsed())
            .flatten()
            .collect()
    }

    /// Resets these energy probes, such that the next time `elapsed` is
    /// called, the total amount of energy since this reset is returned.
    fn reset(&mut self) {
        self.iter_mut()
            .for_each(|probe| probe.reset())
    }
}

pub fn transpose(measurements: &Vec<Energy>) -> Vec<Vec<f32>> {
    let mut iter_probes: Vec<_> = measurements.iter()
        .map(|probe_energies| probe_energies.values().cloned())
        .collect();

    let n = measurements[0].len();
    (0..n).map(|_| {
        iter_probes.iter_mut()
            .map(|probe_energies| probe_energies.next().unwrap())
            .collect()
    }).collect()
}
