use std::path::Path;
use std::str::FromStr;

use indexmap::IndexMap;
use once_cell::sync::Lazy;

use crate::file_handle::FileHandle;
use crate::{Probe, Elapsed};

/// RAPL can exist in either `/sys/class` or `/sys/devices`. Determine the
/// correct location, with a preference for `/sys/class` if both exist.
/// Note the even on AMD processors, the path contains `intel-rapl`.
static PREFIX: Lazy<&'static str> = Lazy::new(|| {
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
    pub dram: Option<Subzone>,
}

#[derive(Debug)]
pub struct Subzone {
    handle: FileHandle,
    name: String,
    energy_uj: u64,
    pub max_energy_range_uj: u64,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug)]
pub struct Constraint {
    /// constraint_X_name (ro) (optional)
    /// An optional name of the constraint
    pub name: Option<String>,
    /// constraint_X_power_limit_uw (rw) (required)
    /// Power limit in micro watts, which should be applicable for the time window specified by “constraint_X_time_window_us”.
    pub power_limit_uw: u64,
    power_limit_handle: FileHandle,
    /// constraint_X_time_window_us (rw) (required)
    /// Time window in micro seconds.
    pub time_window_us: u64,
    time_window_handle: FileHandle,
    /// constraint_X_min_power_uw (ro) (optional)
    /// Minimum allowed power in micro watts.
    pub min_power_uw: Option<u64>,
    /// constraint_X_max_power_uw (ro) (optional)
    /// Maximum allowed power in micro watts.
    pub max_power_uw: Option<u64>,
    /// constraint_X_min_time_window_us (ro) (optional)
    /// Minimum allowed time window in micro seconds.
    pub min_time_window_us: Option<u64>,
    /// constraint_X_max_time_window_us (ro) (optional)
    /// Maximum allowed time window in micro seconds.
    pub max_time_window_us: Option<u64>,
}

impl Rapl {
    pub fn now(with_subzones: bool) -> Option<Self> {
        let head = Package::now(0, with_subzones)?;
        let tail = (1..u8::MAX).map_while(|package_id| Package::now(package_id, with_subzones));
        let packages = std::iter::once(head).chain(tail).collect();
        Some(Self { packages })
    }
}

impl Probe for Rapl {
    fn elapsed(&self) -> Elapsed {
        self.packages.iter().flat_map(Package::elapsed).collect()
    }

    fn reset(&mut self) {
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

        let constraints = (0..u8::MAX).map_while(|constraint_id| Constraint::now(constraint_id, package_id, None)).collect();

        let subzones = if with_subzones {
            (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect()
        } else {
            Vec::with_capacity(0)
        };

        let dram = Subzone::mmio_now(package_id);

        Some(Self { handle, name, max_energy_range_uj, package_energy_uj, constraints, subzones, dram })
    }

    pub fn elapsed(&self) -> Elapsed {
        let mut res = IndexMap::with_capacity(1 + self.subzones.len());

        let package_energy_next = self.handle.read();
        let package_energy = diff(self.package_energy_uj, package_energy_next, self.max_energy_range_uj);
        res.insert(self.name.clone(), package_energy);

        let subzone_energy_uj = self.subzones.iter().map(Subzone::elapsed);
        res.extend(subzone_energy_uj);

        if let Some(dram) = &self.dram {
            let (k, v) = dram.elapsed();
            res.insert(k, v);
        }

        res
    }

    pub fn reset(&mut self) {
        self.package_energy_uj = self.handle.read();
        self.subzones.iter_mut().for_each(Subzone::reset);
        if let Some(dram) = &mut self.dram {
            dram.reset();
        }
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

        let constraints = (0..u8::MAX).map_while(|constraint_id| Constraint::now(constraint_id, package_id, Some(subzone_id))).collect();

        Some(Self { handle, name, max_energy_range_uj, energy_uj, constraints })
    }

    pub fn mmio_now(package_id: u8) -> Option<Self> {
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

impl Constraint {
    pub fn now(constraint_id: u8, package_id: u8, subzone_id: Option<u8>) -> Option<Self> {
        let path = if let Some(subzone_id) = subzone_id {
            format!("{}/intel-rapl:{}:{}", *PREFIX, package_id, subzone_id)
        } else {
            format!("{}/intel-rapl:{}", *PREFIX, package_id)
        };

        // Power limit is required; if it does not exist then this constraint does not exist
        let power_limit_handle = FileHandle::new(&format!("{}/constraint_{}_{}", path, constraint_id, "power_limit_uw"), true).ok()?;
        let time_window_handle = FileHandle::new(&format!("{}/constraint_{}_{}", path, constraint_id, "time_window_us"), true).ok()?;

        Some(Self {
            name:               optional(&path, &format!("constraint_{}_{}", constraint_id, "name")),
            min_power_uw:       optional(&path, &format!("constraint_{}_{}", constraint_id, "min_power_uw")),
            max_power_uw:       optional(&path, &format!("constraint_{}_{}", constraint_id, "max_power_uw")),
            min_time_window_us: optional(&path, &format!("constraint_{}_{}", constraint_id, "min_time_window_us")),
            max_time_window_us: optional(&path, &format!("constraint_{}_{}", constraint_id, "max_time_window_us")),
            power_limit_uw: power_limit_handle.read(),
            time_window_us: time_window_handle.read(),
            power_limit_handle,
            time_window_handle,
        })
    }

    pub fn set_power_limit_uw(&mut self, value: u64) {
        assert!(value > 0);
        assert!(self.max_power_uw.is_none_or(|max| value <= max));
        self.power_limit_handle.write(value);
    }

    pub fn set_time_window_us(&mut self, value: u64) {
        assert!(value > 0);
        assert!(self.max_time_window_us.is_none_or(|max| value <= max));
        self.time_window_handle.write(value);
    }
}

fn required<T: FromStr>(path: &str, file: &str) -> T where T::Err: std::fmt::Debug {
    let path = format!("{}/{}", path, file);
    let handle = FileHandle::new(&path, false).unwrap();
    handle.read::<T>()
}

fn optional<T: FromStr>(path: &str, file: &str) -> Option<T> where T::Err: std::fmt::Debug {
    let path = format!("{}/{}", path, file);
    let handle = FileHandle::new(&path, false).ok()?;
    Some(handle.read::<T>())
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
