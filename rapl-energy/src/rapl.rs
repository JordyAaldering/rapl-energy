use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;

pub struct Rapl {
    packages: Vec<Package>,
}

struct Package {
    package_id: u8,
    energy_uj: u64,
    subzones: Vec<Subzone>,
}

struct Subzone {
    package_id: u8,
    subzone_id: u8,
    energy_uj: u64,
}

#[derive(Clone, Default)]
#[derive(serde::Serialize)]
pub struct RaplEnergy {
    energy_uj: u64,
    subzones: Vec<u64>,
}

impl Rapl {
    pub fn now() -> Self {
        let packages = (0..u8::MAX).map_while(Package::now).collect();
        Rapl { packages }
    }

    pub fn elapsed(&self) -> HashMap<u8, RaplEnergy> {
        self.packages.iter().map(|package| (package.package_id, package.elapsed())).collect()
    }

    pub fn elapsed_mut(&mut self) -> HashMap<u8, RaplEnergy> {
        self.packages.iter_mut().map(|package| (package.package_id, package.elapsed_mut())).collect()
    }
}

impl Package {
    fn now(package_id: u8) -> Option<Self> {
        let energy_uj = package_raw(package_id)?;
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();
        Some(Package { package_id, energy_uj, subzones })
    }

    fn elapsed(&self) -> RaplEnergy {
        let energy_uj = package_raw(self.package_id).unwrap() - self.energy_uj;
        let subzones = self.subzones.iter().map(Subzone::elapsed).collect();
        RaplEnergy { energy_uj, subzones }
    }

    fn elapsed_mut(&mut self) -> RaplEnergy {
        let prev_energy_uj = self.energy_uj;
        let energy_uj = package_raw(self.package_id).unwrap() - self.energy_uj;
        self.energy_uj = prev_energy_uj;

        let subzones = self.subzones.iter_mut().map(Subzone::elapsed_mut).collect();
        RaplEnergy { energy_uj, subzones }
    }
}

impl Subzone {
    fn now(package_id: u8, subzone_id: u8) -> Option<Self> {
        let energy_uj = subzone_raw(package_id, subzone_id)?;
        Some(Subzone { package_id, subzone_id, energy_uj })
    }

    fn elapsed(&self) -> u64 {
        subzone_raw(self.package_id, self.subzone_id).unwrap() - self.energy_uj
    }

    fn elapsed_mut(&mut self) -> u64 {
        let prev_energy_uj = self.energy_uj;
        let elapsed = self.elapsed();
        self.energy_uj = prev_energy_uj;
        elapsed
    }
}

fn package_raw(package_id: u8) -> Option<u64> {
    let path = format!("/sys/class/powercap/intel-rapl:{}/energy_uj", package_id);
    let mut file = OpenOptions::new().read(true).open(&path).ok()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).ok()?;
    buf.trim().parse::<u64>().ok()
}

fn subzone_raw(package_id: u8, subzone_id: u8) -> Option<u64> {
    let path = format!("/sys/class/powercap/intel-rapl:{}:{}/energy_uj", package_id, subzone_id);
    let mut file = OpenOptions::new().read(true).open(&path).ok()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).ok()?;
    buf.trim().parse::<u64>().ok()
}
