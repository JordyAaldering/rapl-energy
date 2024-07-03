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

impl Rapl {
    pub fn now() -> Self {
        let packages = (0..u8::MAX).map_while(Package::now).collect();
        Rapl { packages }
    }

    pub fn elapsed(&self) -> Vec<u64> {
        self.packages.iter().flat_map(|core| core.elapsed()).collect()
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

    fn elapsed(&self) -> Vec<u64> {
        let path = format!("/sys/class/powercap/intel-rapl:{}/energy_uj", self.package_id);
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let energy_uj = read_raw(&mut file) - self.energy_uj;

        let mut subzone_energy = self.subzones.iter().map(|zone| zone.elapsed()).collect::<Vec<u64>>();
        subzone_energy.insert(0, energy_uj);
        subzone_energy
    }
}

impl Subzone {
    fn now(package_id: u8, subzone_id: u8) -> Option<Self> {
        let path = format!("/sys/class/powercap/intel-rapl:{}:{}/energy_uj", package_id, subzone_id);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;
        let energy_uj = read_raw(&mut file);
        Some(Subzone { package_id, subzone_id, energy_uj })
    }

    fn elapsed(&self) -> u64 {
        let path = format!("/sys/class/powercap/intel-rapl:{}:{}/energy_uj", self.package_id, self.subzone_id);
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let energy_uj = read_raw(&mut file);
        energy_uj - self.energy_uj
    }
}

fn read_raw(file: &mut File) -> u64 {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.trim().parse::<u64>().unwrap()
}
