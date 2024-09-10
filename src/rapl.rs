use std::fs::{self, OpenOptions};
use std::io::Read;
use std::iter::once;

use indexmap::IndexMap;

use crate::Energy;

pub struct Rapl {
    packages: Vec<Package>,
}

struct Package {
    package_id: u8,
    name: String,
    max: u64,
    energy: u64,
    subzones: Vec<Subzone>,
}

struct Subzone {
    package_id: u8,
    subzone_id: u8,
    name: String,
    max: u64,
    energy: u64,
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
        let name = package_name(package_id);
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();
        Some(Package { name, package_id, max, energy, subzones })
    }

    fn elapsed(&self) -> IndexMap<String, f64> {
        let prev = self.energy;
        let next = self.next();
        let energy = diff(prev, next, self.max);

        once((self.name.clone(), energy))
            .chain(self.subzones.iter().map(Subzone::elapsed))
            .collect()
    }

    fn elapsed_mut(&mut self) -> IndexMap<String, f64> {
        let prev = self.energy;
        let next = self.next();
        let energy = diff(prev, next, self.max);
        self.energy = next;

        once((self.name.clone(), energy))
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
        let name = subzone_name(package_id, subzone_id);
        Some(Subzone { name, package_id, subzone_id, max, energy })
    }

    fn elapsed(&self) -> (String, f64) {
        let prev = self.energy;
        let next = self.next();

        let energy = diff(prev, next, self.max);
        (self.name.clone(), energy)
    }

    fn elapsed_mut(&mut self) -> (String, f64) {
        let prev = self.energy;
        let next = self.next();
        self.energy = next;

        let energy = diff(prev, next, self.max);
        (self.name.clone(), energy)
    }

    fn next(&self) -> u64 {
        read_subzone(self.package_id, self.subzone_id).unwrap()
    }
}

fn diff(prev: u64, next: u64, max: u64) -> f64 {
    let energy_uj = if next >= prev {
        next - prev
    } else {
        // The accumulator overflowed
        next + (max - prev)
    };
    energy_uj as f64 / 1000_000.0
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
