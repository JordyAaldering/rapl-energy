use std::fs;
use std::iter::once;

use indexmap::IndexMap;

use crate::Energy;

struct Accumulator<T> where T: Copy + Default + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::cmp::PartialOrd {
    value: T,
    max: T,
}

impl<T> Accumulator<T> where T: Copy + Default + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::cmp::PartialOrd {
    pub fn new(value: T, max: T) -> Accumulator<T> {
        debug_assert!(value < max);
        Accumulator { value, max }
    }

    pub fn diff(&self, next: T) -> T {
        if next >= self.value {
            next - self.value
        } else {
            // The accumulator overflowed
            next + (self.max - self.value)
        }
    }

    pub fn update(&mut self, next: T) -> T {
        let diff = self.diff(next);
        self.value = next;
        diff
    }
}

pub struct Rapl {
    packages: Vec<Package>,
}

struct Package {
    name: String,
    package_id: u8,
    energy: Accumulator<u64>,
    subzones: Vec<Subzone>,
}

struct Subzone {
    name: String,
    package_id: u8,
    subzone_id: u8,
    energy: Accumulator<u64>,
}

impl Rapl {
    pub fn now() -> Option<Box<dyn Energy>> {
        let package0 = once(Package::now(0)?);
        let packages = package0.chain((1..u8::MAX).map_while(Package::now)).collect();
        Some(Box::new(Rapl { packages }))
    }
}

impl Energy for Rapl {
    fn elapsed(&self) -> IndexMap<String, f64> {
        self.packages
            .iter()
            .flat_map(Package::elapsed)
            .collect()
    }

    fn elapsed_mut(&mut self) -> IndexMap<String, f64> {
        self.packages
            .iter_mut()
            .flat_map(Package::elapsed_mut)
            .collect()
    }
}

impl Package {
    fn now(package_id: u8) -> Option<Self> {
        let energy = read_package(package_id)?;
        let max = read_package_range(package_id);
        let energy = Accumulator::new(energy, max);
        let name = package_name(package_id);
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();
        Some(Package { name, package_id, energy, subzones })
    }

    fn elapsed(&self) -> IndexMap<String, f64> {
        let next = self.next();
        let diff = self.energy.diff(next);
        once((self.name.clone(), to_joules(diff)))
            .chain(self.subzones.iter().map(Subzone::elapsed))
            .collect()
    }

    fn elapsed_mut(&mut self) -> IndexMap<String, f64> {
        let next = self.next();
        let diff = self.energy.update(next);
        once((self.name.clone(), to_joules(diff)))
            .chain(self.subzones.iter_mut().map(Subzone::elapsed_mut))
            .collect()
    }

    fn next(&self) -> u64 {
        read_package(self.package_id).unwrap()
    }
}

impl Subzone {
    fn now(package_id: u8, subzone_id: u8) -> Option<Self> {
        let energy = read_subzone(package_id, subzone_id)?;
        let max = read_subzone_range(package_id, subzone_id);
        let energy = Accumulator::new(energy, max);
        let name = subzone_name(package_id, subzone_id);
        Some(Subzone { name, package_id, subzone_id, energy })
    }

    fn elapsed(&self) -> (String, f64) {
        let next = self.next();
        let diff = self.energy.diff(next);
        (self.name.clone(), to_joules(diff))
    }

    fn elapsed_mut(&mut self) -> (String, f64) {
        let next = self.next();
        let diff = self.energy.update(next);
        (self.name.clone(), to_joules(diff))
    }

    fn next(&self) -> u64 {
        read_subzone(self.package_id, self.subzone_id).unwrap()
    }
}

fn to_joules(micro_joules: u64) -> f64 {
    micro_joules as f64 / 1000_000.0
}

fn read_package(package_id: u8) -> Option<u64> {
    read(&format!("/sys/class/powercap/intel-rapl:{}/energy_uj", package_id))
}

fn read_subzone(package_id: u8, subzone_id: u8) -> Option<u64> {
    read(&format!("/sys/class/powercap/intel-rapl:{}/intel-rapl:{}:{}/energy_uj", package_id, package_id, subzone_id))
}

fn package_name(package_id: u8) -> String {
    let path = format!("/sys/class/powercap/intel-rapl:{}/name", package_id);
    fs::read_to_string(path).unwrap().trim().to_string()
}

fn subzone_name(package_id: u8, subzone_id: u8) -> String {
    let package_name = package_name(package_id);
    let path = format!("/sys/class/powercap/intel-rapl:{}:{}/name", package_id, subzone_id);
    let subzone_name = fs::read_to_string(path).unwrap().trim().to_string();
    format!("{}-{}", package_name, subzone_name)
}

fn read_package_range(package_id: u8) -> u64 {
    read(&format!("/sys/class/powercap/intel-rapl:{}/max_energy_range_uj", package_id)).unwrap()
}

fn read_subzone_range(package_id: u8, subzone_id: u8) -> u64 {
    read(&format!("/sys/class/powercap/intel-rapl:{}/intel-rapl:{}:{}/max_energy_range_uj", package_id, package_id, subzone_id)).unwrap()
}

fn read(path: &String) -> Option<u64> {
    let str = fs::read_to_string(path).ok()?;
    let energy = str.trim().parse::<u64>().unwrap();
    Some(energy)
}
