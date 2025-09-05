mod constraint;
mod file_handle;
mod libc;

use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;

use indexmap::IndexMap;

use crate::constraint::Constraint;
use crate::file_handle::FileHandle;

/// RAPL can exist in either `/sys/class` or `/sys/devices`. Determine the
/// correct location, with a preference for `/sys/class` if both exist.
/// Note the even on AMD processors, the path contains `intel-rapl`.
static PREFIX: LazyLock<&'static str> = LazyLock::new(|| {
    const CLASS_PREFIX: &'static str = "/sys/class/powercap/intel-rapl";
    const DEVICES_PREFIX: &'static str = "/sys/devices/virtual/powercap/intel-rapl";
    if Path::new(CLASS_PREFIX).exists() {
        CLASS_PREFIX
    } else {
        DEVICES_PREFIX
    }
});

/// https://www.kernel.org/doc/html/latest/power/powercap/powercap.html
#[derive(Debug)]
pub struct Rapl {
    pub packages: Vec<Package>,
}

#[derive(Debug)]
pub struct Package {
    handle: FileHandle,
    name: String,
    package_energy_uj: u64,
    pub max_energy_range_uj: u64,
    pub constraints: Vec<Constraint>,
    pub subzones: Vec<Subzone>,
}

#[derive(Debug)]
pub struct Subzone {
    handle: FileHandle,
    name: String,
    energy_uj: u64,
    pub max_energy_range_uj: u64,
    pub constraints: Vec<Constraint>,
}

impl Rapl {
    pub fn now(with_subzones: bool) -> Option<Self> {
        let head = Package::now(0, with_subzones)?;
        let tail = (1..u8::MAX).map_while(|package_id| Package::now(package_id, with_subzones));
        let mmio = (0..u8::MAX).map_while(|package_id| Package::now_mmio(package_id, with_subzones));
        let packages = std::iter::once(head).chain(tail).chain(mmio).collect();
        Some(Self { packages })
    }

    pub fn elapsed(&self) -> IndexMap<String, f32> {
        self.packages.iter().flat_map(Package::elapsed).collect()
    }

    pub fn reset(&mut self) {
        self.packages.iter_mut().for_each(Package::reset);
    }
}

impl Package {
    pub fn now(package_id: u8, with_subzones: bool) -> Option<Self> {
        let path = format!("{}/intel-rapl:{}", *PREFIX, package_id);
        let handle = FileHandle::new(&format!("{}/energy_uj", path), false).ok()?;

        let name = required(&path, "name");
        let max_energy_range_uj = required(&path, "max_energy_range_uj");

        let package_energy_uj = handle.read();

        let constraints = (0..u8::MAX).map_while(|constraint_id| Constraint::now(&path, constraint_id)).collect();

        let subzones = if with_subzones {
            (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect()
        } else {
            Vec::with_capacity(0)
        };

        Some(Self { handle, name, max_energy_range_uj, package_energy_uj, constraints, subzones })
    }

    pub fn now_mmio(package_id: u8, with_subzones: bool) -> Option<Self> {
        let path = format!("{}/intel-rapl-mmio:{}", *PREFIX, package_id);
        let handle = FileHandle::new(&format!("{}/energy_uj", path), false).ok()?;

        let name = required(&path, "name");
        let max_energy_range_uj = required(&path, "max_energy_range_uj");

        let package_energy_uj = handle.read();

        let constraints = (0..u8::MAX).map_while(|constraint_id| Constraint::now(&path, constraint_id)).collect();

        let subzones = if with_subzones {
            (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect()
        } else {
            Vec::with_capacity(0)
        };

        Some(Self { handle, name, max_energy_range_uj, package_energy_uj, constraints, subzones })
    }

    pub fn elapsed(&self) -> IndexMap<String, f32> {
        let mut res = IndexMap::with_capacity(1 + self.subzones.len());

        let package_energy_next = self.handle.read();
        let package_energy = diff(self.package_energy_uj, package_energy_next, self.max_energy_range_uj);
        res.insert(self.name.clone(), package_energy);

        let subzone_energy_uj = self.subzones.iter().map(Subzone::elapsed);
        res.extend(subzone_energy_uj);

        res
    }

    pub fn reset(&mut self) {
        self.package_energy_uj = self.handle.read();
        self.subzones.iter_mut().for_each(Subzone::reset);
    }
}

impl Subzone {
    pub fn now(package_id: u8, subzone_id: u8) -> Option<Self> {
        let package_path = format!("{}/intel-rapl:{}", *PREFIX, package_id);
        let subzone_path = format!("{}/intel-rapl:{}:{}", package_path, package_id, subzone_id);
        let handle = FileHandle::new(&format!("{}/energy_uj", subzone_path), false).ok()?;

        let package_name: String = required(&package_path, "name");
        let subzone_name: String = required(&subzone_path, "name");
        let name = format!("{}-{}", package_name, subzone_name);

        let max_energy_range_uj = required(&subzone_path, "max_energy_range_uj");

        let energy_uj = handle.read();

        let constraints = (0..u8::MAX).map_while(|constraint_id| Constraint::now(&subzone_path, constraint_id)).collect();

        Some(Self { handle, name, max_energy_range_uj, energy_uj, constraints })
    }

    pub fn now_mmio(package_id: u8) -> Option<Self> {
        let path = format!("{}-mmio/intel-rapl-mmio:{}", *PREFIX, package_id);
        let handle = FileHandle::new(&format!("{}/energy_uj", path), false).ok()?;

        let package_name: String = required(&path, "name");
        let name = format!("{}-mmio", package_name);

        let max_energy_range_uj = required(&path, "max_energy_range_uj");

        let energy_uj = handle.read();

        Some(Self { handle, name, max_energy_range_uj, energy_uj, constraints: Vec::new() })
    }

    pub fn elapsed(&self) -> (String, f32) {
        let energy_next = self.handle.read();
        let energy = diff(self.energy_uj, energy_next, self.max_energy_range_uj);
        (self.name.clone(), energy)
    }

    pub fn reset(&mut self) {
        self.energy_uj = self.handle.read();
    }
}

fn required<T: FromStr>(path: &str, file: &str) -> T where T::Err: std::fmt::Debug {
    let path = format!("{}/{}", path, file);
    let handle = FileHandle::new(&path, false).unwrap();
    handle.read::<T>()
}

fn diff(prev_uj: u64, next_uj: u64, max_energy_range_uj: u64) -> f32 {
    let energy_uj = if next_uj >= prev_uj {
        next_uj - prev_uj
    } else {
        // The accumulator overflowed
        next_uj + (max_energy_range_uj - prev_uj)
    };
    energy_uj as f32 / 1e9
}
