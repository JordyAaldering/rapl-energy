use std::fs::{File, OpenOptions};
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

#[derive(Clone)]
#[derive(serde::Serialize)]
pub struct PackageEnergy {
    energy_uj: u64,
    subzones: Vec<SubzoneEnergy>,
}

#[derive(Clone)]
#[derive(serde::Serialize)]
struct SubzoneEnergy {
    energy_uj: u64,
}

impl Rapl {
    pub fn now() -> Self {
        let packages = (0..u8::MAX).map_while(Package::now).collect();
        Rapl { packages }
    }

    pub fn elapsed(&self) -> Vec<PackageEnergy> {
        self.packages.iter().map(Package::elapsed).collect()
    }
}

impl Package {
    fn now(package_id: u8) -> Option<Self> {
        let path = format!("/sys/class/powercap/intel-rapl:{}/energy_uj", package_id);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;
        let energy_uj = read_raw(&mut file);
        let subzones = (0..u8::MAX).map_while(|subzone_id| Subzone::now(package_id, subzone_id)).collect();
        Some(Package { package_id, energy_uj, subzones })
    }

    fn elapsed(&self) -> PackageEnergy {
        let path = format!("/sys/class/powercap/intel-rapl:{}/energy_uj", self.package_id);
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let energy_uj = read_raw(&mut file) - self.energy_uj;
        let subzones = self.subzones.iter().map(Subzone::elapsed).collect();
        PackageEnergy { energy_uj, subzones }
    }
}

impl Subzone {
    fn now(package_id: u8, subzone_id: u8) -> Option<Self> {
        let path = format!("/sys/class/powercap/intel-rapl:{}:{}/energy_uj", package_id, subzone_id);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;
        let energy_uj = read_raw(&mut file);
        Some(Subzone { package_id, subzone_id, energy_uj })
    }

    fn elapsed(&self) -> SubzoneEnergy {
        let path = format!("/sys/class/powercap/intel-rapl:{}:{}/energy_uj", self.package_id, self.subzone_id);
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let energy_uj = read_raw(&mut file) - self.energy_uj;
        SubzoneEnergy { energy_uj }
    }
}

fn read_raw(file: &mut File) -> u64 {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.trim().parse::<u64>().unwrap()
}

impl std::ops::Add for PackageEnergy {
    type Output = PackageEnergy;

    fn add(self, rhs: Self) -> Self::Output {
        let energy_uj = self.energy_uj - rhs.energy_uj;
        let subzones = self.subzones.into_iter()
            .zip(rhs.subzones)
            .map(|(x, y)| x + y)
            .collect();
        PackageEnergy { energy_uj, subzones }
    }
}

impl std::ops::Sub for PackageEnergy {
    type Output = PackageEnergy;

    fn sub(self, rhs: Self) -> Self::Output {
        let energy_uj = self.energy_uj - rhs.energy_uj;
        let subzones = self.subzones.into_iter()
            .zip(rhs.subzones)
            .map(|(x, y)| x - y)
            .collect();
        PackageEnergy { energy_uj, subzones }
    }
}

impl std::ops::Add for SubzoneEnergy {
    type Output = SubzoneEnergy;

    fn add(self, rhs: Self) -> Self::Output {
        let energy_uj = self.energy_uj + rhs.energy_uj;
        SubzoneEnergy { energy_uj }
    }
}

impl std::ops::Sub for SubzoneEnergy {
    type Output = SubzoneEnergy;

    fn sub(self, rhs: Self) -> Self::Output {
        let energy_uj = self.energy_uj - rhs.energy_uj;
        SubzoneEnergy { energy_uj }
    }
}
