use std::fs::{File, OpenOptions};
use std::io::Read;

use crate::RaplReader;

pub struct RaplIntel {
    path: String,
    package_id: usize,
    subzone_id: Option<usize>,
    energy_uj: u64,
}

impl RaplReader for RaplIntel {
    fn now(package_id: usize) -> Option<Self> {
        let path = format!("/sys/class/powercap/intel-rapl:{}/energy_uj", package_id);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;
        let energy_uj = read_raw(&mut file);
        Some(RaplIntel { path, package_id, subzone_id: None, energy_uj })
    }

    fn elapsed(&self) -> u64 {
        let mut file = OpenOptions::new().read(true).open(&self.path).unwrap();
        let energy_uj = read_raw(&mut file);
        energy_uj - self.energy_uj
    }

    fn label(&self) -> String {
        if let Some(subzone_id) = self.subzone_id {
            format!("intel-rapl:{}:{}", self.package_id, subzone_id)
        } else {
            format!("intel-rapl:{}", self.package_id)
        }
    }
}

impl RaplIntel {
    /// Currently unused, but we might want this in the future.
    /// If RaplAMD has a similar field this is easy, otherwise we need to find
    /// a nice generic solution
    #[allow(unused)]
    fn subzone_now(package_id: usize, subzone_id: usize) -> Option<Self> {
        let path = format!("/sys/class/powercap/intel-rapl:{}:{}/energy_uj", package_id, subzone_id);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;
        let energy_uj = read_raw(&mut file);
        Some(RaplIntel { path, package_id, subzone_id: Some(subzone_id), energy_uj })
    }
}

fn read_raw(file: &mut File) -> u64 {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf.trim().parse::<u64>().unwrap()
}
