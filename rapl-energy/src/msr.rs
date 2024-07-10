use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::sync::Mutex;
use std::time::Duration;

pub struct Msr {
    cores: Vec<MsrCore>,
}

struct MsrCore {
    package_id: u8,
    handle: Mutex<File>,
    package_energy_uj: u64,
    core_energy_uj: u64,
}

#[derive(Clone, Debug, Default)]
#[derive(serde::Serialize)]
pub struct MsrEnergy {
    package_energy_uj: u64,
    core_energy_uj: u64,
}

#[derive(Clone, Debug, Default)]
#[derive(serde::Serialize)]
pub struct MsrPower {
    package_power_w: f64,
    core_power_w: f64,
}

#[repr(u64)]
enum MsrOffset {
    //PowerUnit     = 0xC0010299,
    CoreEnergy    = 0xC001029A,
    PackageEnergy = 0xC001029B,
}

impl Msr {
    pub fn now() -> Self {
        let cores = (0..u8::MAX).map_while(MsrCore::now).collect();
        Msr { cores }
    }

    pub fn elapsed(&self) -> Vec<MsrEnergy> {
        self.cores.iter().map(MsrCore::elapsed).collect()
    }

    pub fn power(&mut self, duration: Duration) -> Vec<MsrPower> {
        self.cores.iter_mut().map(|core| core.power(duration)).collect()
    }

    pub fn headers(&self) -> Vec<String> {
        self.cores.iter().flat_map(MsrCore::headers).collect()
    }
}

impl MsrCore {
    fn now(package_id: u8) -> Option<Self> {
        let path = format!("/dev/cpu/{}/msr", package_id);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;
        Some(MsrCore {
            package_id,
            package_energy_uj: read(&mut file, MsrOffset::PackageEnergy),
            core_energy_uj: read(&mut file, MsrOffset::CoreEnergy),
            handle: Mutex::new(file),
        })
    }

    fn elapsed(&self) -> MsrEnergy {
        let mut file = self.handle.lock().unwrap();
        MsrEnergy {
            package_energy_uj: read(&mut file, MsrOffset::PackageEnergy) - self.package_energy_uj,
            core_energy_uj: read(&mut file, MsrOffset::CoreEnergy) - self.core_energy_uj,
        }
    }

    fn power(&mut self, duration: Duration) -> MsrPower {
        let package_prev = self.package_energy_uj;
        let core_prev = self.core_energy_uj;

        let mut file = self.handle.lock().unwrap();
        self.package_energy_uj = read(&mut file, MsrOffset::PackageEnergy);
        self.core_energy_uj = read(&mut file, MsrOffset::CoreEnergy);

        let package_j = ((self.package_energy_uj - package_prev) as f64) / 1e6;
        let core_j = ((self.core_energy_uj - core_prev) as f64) / 1e6;
        MsrPower {
            package_power_w: package_j / duration.as_secs_f64(),
            core_power_w: core_j / duration.as_secs_f64(),
        }
    }

    fn headers(&self) -> Vec<String> {
        vec![
            format!("cpu{}:package", self.package_id),
            format!("cpu{}:core", self.package_id),
        ]
    }
}

fn read(file: &mut File, offset: MsrOffset) -> u64 {
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    let mut buf = [0; size_of::<u64>()];
    file.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}
