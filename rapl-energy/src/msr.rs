use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::sync::Mutex;
use std::time::Instant;

pub struct Msr {
    instant: Instant,
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
    package_power_w: u64,
    core_power_w: u64,
}

#[repr(u64)]
enum MsrOffset {
    //PowerUnit     = 0xC0010299,
    CoreEnergy    = 0xC001029A,
    PackageEnergy = 0xC001029B,
}

impl Msr {
    pub fn now() -> Self {
        Msr {
            instant: Instant::now(),
            cores: (0..u8::MAX).map_while(MsrCore::now).collect(),
        }
    }

    pub fn elapsed(&self) -> Vec<MsrEnergy> {
        self.cores.iter().map(MsrCore::elapsed).collect()
    }

    pub fn power(&mut self) -> Vec<MsrPower> {
        let ms = self.instant.elapsed().as_micros() as u64;
        self.instant = Instant::now();
        self.cores.iter_mut().map(|core| core.power(ms)).collect()
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

    fn power(&mut self, ms: u64) -> MsrPower {
        let package_prev = self.package_energy_uj;
        let core_prev = self.core_energy_uj;

        let mut file = self.handle.lock().unwrap();
        self.package_energy_uj = read(&mut file, MsrOffset::PackageEnergy);
        self.core_energy_uj = read(&mut file, MsrOffset::CoreEnergy);

        MsrPower {
            package_power_w: ms / (self.package_energy_uj - package_prev),
            core_power_w: ms / (self.core_energy_uj - core_prev),
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
