use std::fs::OpenOptions;
use std::io::Read;

pub struct Rapl {
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

#[derive(Clone, Default)]
#[derive(serde::Serialize)]
pub struct RaplEnergy {
    package_energy_uj: u64,
    subzones: Vec<u64>,
}

impl Rapl {
    pub fn now() -> Self {
        let packages = (0..u8::MAX).map_while(Package::now).collect();
        Rapl { packages }
    }

    pub fn elapsed(&self) -> Vec<RaplEnergy> {
        self.packages.iter().map(Package::elapsed).collect()
    }

    pub fn elapsed_mut(&mut self) -> Vec<RaplEnergy> {
        self.packages.iter_mut().map(Package::elapsed_mut).collect()
    }
}

impl Package {
    fn now(package_id: u8) -> Option<Self> {
        let energy_uj = read_package(package_id)?;
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();
        Some(Package { package_id, package_energy_uj: energy_uj, subzones })
    }

    fn elapsed(&self) -> RaplEnergy {
        let prev = self.package_energy_uj;
        let next = read_package(self.package_id).unwrap();

        let subzones = self.subzones.iter().map(Subzone::elapsed).collect();

        RaplEnergy { package_energy_uj: next - prev, subzones }
    }

    fn elapsed_mut(&mut self) -> RaplEnergy {
        let prev = self.package_energy_uj;
        let next = read_package(self.package_id).unwrap();
        self.package_energy_uj = next;

        let subzones = self.subzones.iter_mut().map(Subzone::elapsed_mut).collect();

        RaplEnergy { package_energy_uj: next - prev, subzones }
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

    fn elapsed_mut(&mut self) -> u64 {
        let prev = self.energy_uj;
        let next = read_subzone(self.package_id, self.subzone_id).unwrap();
        self.energy_uj = next;
        next - prev
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
