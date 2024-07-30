use indexmap::IndexMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::time::Duration;

pub struct Rapl {
    packages: Vec<Package>,
}

struct Package {
    package_id: u8,
    max_energy_range_uj: u64,
    package_energy_uj: u64,
    subzones: Vec<Subzone>,
}

struct Subzone {
    package_id: u8,
    subzone_id: u8,
    max_energy_range_uj: u64,
    energy_uj: u64,
}

impl Rapl {
    pub fn now() -> Self {
        let packages = (0..u8::MAX).map_while(Package::now).collect();
        Rapl { packages }
    }

    pub fn elapsed(&self) -> IndexMap<String, f64> {
        self.packages.iter().flat_map(Package::elapsed).collect()
    }

    pub fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        self.packages.iter_mut().flat_map(|p| p.power(duration)).collect()
    }
}

impl Package {
    fn now(package_id: u8) -> Option<Self> {
        let package_energy_uj = read_package(package_id)?;
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();
        let max_energy_range_uj = read_package_range(package_id);
        Some(Package { package_id, max_energy_range_uj, package_energy_uj, subzones })
    }

    fn elapsed(&self) -> IndexMap<String, f64> {
        let mut res = IndexMap::with_capacity(1 + self.subzones.len());

        let package_energy_next = read_package(self.package_id).unwrap();
        let package_energy_uj = rapl_diff(self.package_energy_uj, package_energy_next, self.max_energy_range_uj);
        res.insert(format!("intel-rapl:{}", self.package_id), package_energy_uj as f64);

        let subzone_energy_uj = self.subzones.iter().map(Subzone::elapsed);
        res.extend(subzone_energy_uj);

        res
    }

    fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        let package_uj_prev = self.package_energy_uj;
        self.package_energy_uj = read_package(self.package_id).unwrap();
        let package_uj = rapl_diff(package_uj_prev, self.package_energy_uj, self.max_energy_range_uj);
        let package_power_w = (package_uj as f64 / 1e6) / duration.as_secs_f64();

        let mut res = IndexMap::with_capacity(1 + self.subzones.len());
        res.insert(format!("intel-rapl:{}", self.package_id), package_power_w);
        let subzone_energy_uj = self.subzones.iter_mut().map(|s| s.power(duration));
        res.extend(subzone_energy_uj);
        res
    }
}

impl Subzone {
    fn now(package_id: u8, subzone_id: u8) -> Option<Self> {
        let energy_uj = read_subzone(package_id, subzone_id)?;
        let max_energy_range_uj = read_subzone_range(package_id, subzone_id);
        Some(Subzone { package_id, subzone_id, max_energy_range_uj, energy_uj })
    }

    fn elapsed(&self) -> (String, f64) {
        let prev = self.energy_uj;
        let next = read_subzone(self.package_id, self.subzone_id).unwrap();
        let energy_uj = rapl_diff(prev, next, self.max_energy_range_uj);
        (format!("intel-rapl:{}:{}", self.package_id, self.subzone_id), energy_uj as f64)
    }

    fn power(&mut self, duration: Duration) -> (String, f64) {
        let prev_energy_uj = self.energy_uj;
        self.energy_uj = read_subzone(self.package_id, self.subzone_id).unwrap();
        let energy_uj = rapl_diff(prev_energy_uj, self.energy_uj, self.max_energy_range_uj);
        let energy_uj = (energy_uj as f64 / 1e6) / duration.as_secs_f64();
        (format!("intel-rapl:{}:{}", self.package_id, self.subzone_id), energy_uj)
    }
}

fn rapl_diff(prev_uj: u64, next_uj: u64, max_energy_range_uj: u64) -> u64 {
    if next_uj >= prev_uj {
        next_uj - prev_uj
    } else {
        // The accumulator overflowed
        next_uj + (max_energy_range_uj - prev_uj)
    }
}

fn read_package(package_id: u8) -> Option<u64> {
    read(&format!("/sys/class/powercap/intel-rapl:{}/energy_uj", package_id))
}

fn read_subzone(package_id: u8, subzone_id: u8) -> Option<u64> {
    read(&format!("/sys/class/powercap/intel-rapl:{}/intel-rapl:{}:{}/energy_uj", package_id, package_id, subzone_id))
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
