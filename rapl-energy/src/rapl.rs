use std::fs::OpenOptions;
use std::io::Read;
use std::time::Instant;

pub struct Rapl {
    instant: Instant,
    packages: Vec<Package>,
}

struct Package {
    package_id: u8,
    package_energy_uj: u64,
    subzones: Vec<Subzone>,
}

struct Subzone {
    package_id: u8,
    subzone_id: u8,
    energy_uj: u64,
}

#[derive(Clone, Debug, Default)]
#[derive(serde::Serialize)]
pub struct RaplEnergy {
    package_energy_uj: u64,
    subzone_energy_uj: Vec<u64>,
}

#[derive(Clone, Debug, Default)]
#[derive(serde::Serialize)]
pub struct RaplPower {
    package_power_w: u64,
    subzone_power_w: Vec<u64>,
}

impl Rapl {
    pub fn now() -> Self {
        Rapl {
            instant: Instant:: now(),
            packages: (0..u8::MAX).map_while(Package::now).collect(),
        }
    }

    pub fn elapsed(&self) -> Vec<RaplEnergy> {
        self.packages.iter().map(Package::elapsed).collect()
    }

    pub fn power(&mut self) -> Vec<RaplPower> {
        let ms = self.instant.elapsed().as_micros() as u64;
        self.instant = Instant::now();
        self.packages.iter_mut().map(|package| package.power(ms)).collect()
    }

    pub fn headers(&self) -> Vec<String> {
        self.packages.iter().flat_map(Package::headers).collect()
    }
}

impl Package {
    fn now(package_id: u8) -> Option<Self> {
        let energy_uj = read_package(package_id)?;
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();
        Some(Package { package_id, package_energy_uj: energy_uj, subzones })
    }

    fn elapsed(&self) -> RaplEnergy {
        RaplEnergy {
            package_energy_uj: read_package(self.package_id).unwrap() - self.package_energy_uj,
            subzone_energy_uj: self.subzones.iter().map(Subzone::elapsed).collect(),
        }
    }

    fn power(&mut self, ms: u64) -> RaplPower {
        let prev_package_energy_uj = self.package_energy_uj;
        self.package_energy_uj = read_package(self.package_id).unwrap();
        RaplPower {
            package_power_w: (self.package_energy_uj - prev_package_energy_uj) / ms,
            subzone_power_w: self.subzones.iter_mut().map(|subzone| subzone.power(ms)).collect()
        }
    }

    fn headers(&self) -> Vec<String> {
        let mut res = self.subzones.iter().map(Subzone::headers).collect::<Vec<String>>();
        res.insert(0, format!("intel-rapl:{}", self.package_id));
        res
    }
}

impl Subzone {
    fn now(package_id: u8, subzone_id: u8) -> Option<Self> {
        let energy_uj = read_subzone(package_id, subzone_id)?;
        Some(Subzone { package_id, subzone_id, energy_uj })
    }

    fn elapsed(&self) -> u64 {
        let prev = self.energy_uj;
        let next = read_subzone(self.package_id, self.subzone_id).unwrap();
        next - prev
    }

    fn power(&mut self, ms: u64) -> u64 {
        let prev_energy_uj = self.energy_uj;
        self.energy_uj = read_subzone(self.package_id, self.subzone_id).unwrap();
        (self.energy_uj - prev_energy_uj) / ms
    }

    fn headers(&self) -> String {
        format!("intel-rapl:{}:{}", self.package_id, self.subzone_id)
    }
}

fn read_package(package_id: u8) -> Option<u64> {
    read(&format!("/sys/class/powercap/intel-rapl:{}/energy_uj", package_id))
}

fn read_subzone(package_id: u8, subzone_id: u8) -> Option<u64> {
    read(&format!("/sys/class/powercap/intel-rapl:{}:{}/energy_uj", package_id, subzone_id))
}

fn read(path: &String) -> Option<u64> {
    let mut file = OpenOptions::new().read(true).open(path).ok()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).ok()?;
    buf.trim().parse::<u64>().ok()
}
