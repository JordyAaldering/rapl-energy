use std::str::FromStr;

use indexmap::IndexMap;

use crate::file_handle::FileHandle;
use crate::Energy;

pub struct Rapl {
    packages: Vec<Package>,
}

struct Package {
    handle: FileHandle,
    name: String,
    max_energy_range_uj: u64,
    package_energy_uj: u64,
    subzones: Vec<Subzone>,
}

struct Subzone {
    handle: FileHandle,
    name: String,
    max_energy_range_uj: u64,
    energy_uj: u64,
}

impl Rapl {
    pub fn now() -> Option<Box<dyn Energy>> {
        let packages = crate::chain(Package::now)?;
        Some(Box::new(Self { packages }))
    }
}

impl Energy for Rapl {
    fn elapsed(&self) -> IndexMap<String, f32> {
        self.packages.iter().flat_map(Package::elapsed).collect()
    }

    fn reset(&mut self) {
        self.packages.iter_mut().for_each(Package::reset);
    }
}

impl Package {
    fn now(package_id: u8) -> Option<Self> {
        let path = format!("/sys/class/powercap/intel-rapl:{}", package_id);
        let handle = FileHandle::new(&format!("{}/energy_uj", path)).ok()?;

        let name = require(&path, "name");
        let max_energy_range_uj = require(&path, "max_energy_range_uj");

        let package_energy_uj = handle.read();
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();

        Some(Self { handle, name, max_energy_range_uj, package_energy_uj, subzones })
    }

    fn elapsed(&self) -> IndexMap<String, f32> {
        let mut res = IndexMap::with_capacity(1 + self.subzones.len());

        let package_energy_next = self.handle.read();
        let package_energy = diff(self.package_energy_uj, package_energy_next, self.max_energy_range_uj);
        res.insert(self.name.clone(), package_energy);

        let subzone_energy_uj = self.subzones.iter().map(Subzone::elapsed);
        res.extend(subzone_energy_uj);

        res
    }

    fn reset(&mut self) {
        self.package_energy_uj = self.handle.read();
        self.subzones.iter_mut().for_each(Subzone::reset);
    }
}

impl Subzone {
    fn now(package_id: u8, subzone_id: u8) -> Option<Self> {
        let package_path = format!("/sys/class/powercap/intel-rapl:{}", package_id);
        let subzone_path = format!("{}/intel-rapl:{}:{}", package_path, package_id, subzone_id);
        let handle = FileHandle::new(&format!("{}/energy_uj", subzone_path)).ok()?;

        let package_name: String = require(&package_path, "name");
        let subzone_name: String = require(&subzone_path, "name");
        let name = format!("{}-{}", package_name, subzone_name);

        let max_energy_range_uj = require(&subzone_path, "max_energy_range_uj");

        let energy_uj = handle.read();
        Some(Self { handle, name, max_energy_range_uj, energy_uj })
    }

    fn elapsed(&self) -> (String, f32) {
        let energy_next = self.handle.read();
        let energy = diff(self.energy_uj, energy_next, self.max_energy_range_uj);
        (self.name.clone(), energy)
    }

    fn reset(&mut self) {
        self.energy_uj = self.handle.read();
    }
}

fn diff(prev_uj: u64, next_uj: u64, max_energy_range_uj: u64) -> f32 {
    let energy_uj = if next_uj >= prev_uj {
        next_uj - prev_uj
    } else {
        // The accumulator overflowed
        next_uj + (max_energy_range_uj - prev_uj)
    };
    energy_uj as f32 / 1e6
}

fn require<T: FromStr>(path: &str, file: &str) -> T where T::Err: std::fmt::Debug {
    let path = format!("{}/{}", path, file);
    let handle = FileHandle::new(&path).unwrap();
    handle.read::<T>()
}
