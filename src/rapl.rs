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
    max_energy_range_uj: u64,
    package_energy_uj: u64,
    subzones: Vec<Subzone>,
}

struct Subzone {
    package_id: u8,
    subzone_id: u8,
    name: String,
    max_energy_range_uj: u64,
    energy_uj: u64,
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
        let package_energy_uj = read_package(package_id)?;
        let max_energy_range_uj = read_package_range(package_id);
        let name = package_name(package_id);
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();
        Some(Package { name, package_id, max_energy_range_uj, package_energy_uj, subzones })
    }

    fn elapsed(&self) -> IndexMap<String, f64> {
        let mut res = IndexMap::with_capacity(1 + self.subzones.len());

        let package_energy_next = read_package(self.package_id).unwrap();
        let package_energy = rapl_diff(self.package_energy_uj, package_energy_next, self.max_energy_range_uj);
        res.insert(self.name.clone(), package_energy);

        let subzone_energy_uj = self.subzones.iter().map(Subzone::elapsed);
        res.extend(subzone_energy_uj);

        res
    }

    fn elapsed_mut(&mut self) -> IndexMap<String, f64> {
        let package_uj_prev = self.package_energy_uj;
        self.package_energy_uj = read_package(self.package_id).unwrap();
        let package_energy = rapl_diff(package_uj_prev, self.package_energy_uj, self.max_energy_range_uj);

        let mut res = IndexMap::with_capacity(1 + self.subzones.len());
        res.insert(self.name.clone(), package_energy);
        let subzone_energy_uj = self.subzones.iter_mut().map(Subzone::elapsed_mut);
        res.extend(subzone_energy_uj);

        res
    }
}

impl Subzone {
    fn now(package_id: u8, subzone_id: u8) -> Option<Self> {
        let energy_uj = read_subzone(package_id, subzone_id)?;
        let max_energy_range_uj = read_subzone_range(package_id, subzone_id);
        let name = subzone_name(package_id, subzone_id);
        Some(Subzone { name, package_id, subzone_id, max_energy_range_uj, energy_uj })
    }

    fn elapsed(&self) -> (String, f64) {
        let energy_next = read_subzone(self.package_id, self.subzone_id).unwrap();
        let energy = rapl_diff(self.energy_uj, energy_next, self.max_energy_range_uj);

        (self.name.clone(), energy)
    }

    fn elapsed_mut(&mut self) -> (String, f64) {
        let energy_prev = self.energy_uj;
        self.energy_uj = read_subzone(self.package_id, self.subzone_id).unwrap();
        let energy = rapl_diff(energy_prev, self.energy_uj, self.max_energy_range_uj);

        (self.name.clone(), energy)
    }
}

fn rapl_diff(prev_uj: u64, next_uj: u64, max_energy_range_uj: u64) -> f64 {
    let energy_uj = if next_uj >= prev_uj {
        next_uj - prev_uj
    } else {
        // The accumulator overflowed
        next_uj + (max_energy_range_uj - prev_uj)
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
    let default = format!("intel-rapl:{}", package_id);
    fs::read_to_string(path).map_or(default, |s| s.trim().to_string())
}

fn subzone_name(package_id: u8, subzone_id: u8) -> String {
    let package_name = package_name(package_id);
    let path = format!("/sys/class/powercap/intel-rapl:{}:{}/name", package_id, subzone_id);
    if let Ok(s) = fs::read_to_string(path) {
        format!("{}-{}", package_name, s.trim())
    } else {
        format!("{}:{}", package_name, subzone_id)
    }
}

fn read_package_range(package_id: u8) -> u64 {
    read(&format!("/sys/class/powercap/intel-rapl:{}/max_energy_range_uj", package_id)).unwrap()
}

fn read_subzone_range(package_id: u8, subzone_id: u8) -> u64 {
    read(&format!("/sys/class/powercap/intel-rapl:{}/intel-rapl:{}:{}/max_energy_range_uj", package_id, package_id, subzone_id)).unwrap()
}

fn read(path: &String) -> Option<u64> {
    let mut file = OpenOptions::new().read(true).open(path).ok()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).ok()?;
    buf.trim().parse::<u64>().ok()
}
