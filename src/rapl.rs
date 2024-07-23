use indexmap::IndexMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::time::Duration;

pub struct Rapl {
    packages: Vec<Package>,
}

struct Package {
    package_id: u8,
    package_energy_uj: f64,
    subzones: Vec<Subzone>,
}

struct Subzone {
    package_id: u8,
    subzone_id: u8,
    energy_uj: f64,
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
        let energy_uj = read_package(package_id)?;
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();
        Some(Package { package_id, package_energy_uj: energy_uj, subzones })
    }

    fn elapsed(&self) -> IndexMap<String, f64> {
        let mut res = IndexMap::with_capacity(1 + self.subzones.len());
        let package_energy_uj = read_package(self.package_id).unwrap() - self.package_energy_uj;
        res.insert(format!("intel-rapl:{}", self.package_id), package_energy_uj);
        let subzone_energy_uj = self.subzones.iter().map(Subzone::elapsed);
        res.extend(subzone_energy_uj);
        res
    }

    fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        let prev_package_energy_uj = self.package_energy_uj;
        self.package_energy_uj = read_package(self.package_id).unwrap();
        let package_j = ((self.package_energy_uj - prev_package_energy_uj) as f64) / 1e6;
        let package_power_w = package_j / duration.as_secs_f64();

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
        Some(Subzone { package_id, subzone_id, energy_uj })
    }

    fn elapsed(&self) -> (String, f64) {
        let prev = self.energy_uj;
        let next = read_subzone(self.package_id, self.subzone_id).unwrap();
        (format!("intel-rapl:{}:{}", self.package_id, self.subzone_id), next - prev)
    }

    fn power(&mut self, duration: Duration) -> (String, f64) {
        let prev_energy_uj = self.energy_uj;
        self.energy_uj = read_subzone(self.package_id, self.subzone_id).unwrap();
        let energy_j = ((self.energy_uj - prev_energy_uj) as f64) / 1e6;
        let energy_uj = energy_j / duration.as_secs_f64();
        (format!("intel-rapl:{}:{}", self.package_id, self.subzone_id), energy_uj)
    }
}

fn read_package(package_id: u8) -> Option<f64> {
    read(&format!("/sys/class/powercap/intel-rapl:{}/energy_uj", package_id))
}

fn read_subzone(package_id: u8, subzone_id: u8) -> Option<f64> {
    read(&format!("/sys/class/powercap/intel-rapl:{}:{}/energy_uj", package_id, subzone_id))
}

fn read(path: &String) -> Option<f64> {
    let mut file = OpenOptions::new().read(true).open(path).ok()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).ok()?;
    buf.trim().parse::<u64>().map(|x| x as f64).ok()
}
