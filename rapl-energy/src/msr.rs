use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::sync::Mutex;

pub struct Msr {
    cores: Vec<MsrCore>,
}

struct MsrCore {
    handle: Mutex<File>,
    package_energy_uj: u64,
    core_energy_uj: u64,
}

#[derive(Clone, Default)]
#[derive(serde::Serialize)]
pub struct MsrEnergy {
    package_energy_uj: u64,
    core_energy_uj: u64,
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

    pub fn elapsed_mut(&mut self) -> Vec<MsrEnergy> {
        self.cores.iter_mut().map(MsrCore::elapsed_mut).collect()
    }
}

impl MsrCore {
    fn now(package_id: u8) -> Option<Self> {
        let path = format!("/dev/cpu/{}/msr", package_id);
        let mut file = OpenOptions::new().read(true).write(true).open(&path).ok()?;

        let package_energy_uj = read(&mut file, MsrOffset::PackageEnergy);
        let core_energy_uj = read(&mut file, MsrOffset::CoreEnergy);

        let handle = Mutex::new(file);
        Some(MsrCore { handle, package_energy_uj, core_energy_uj })
    }

    fn elapsed(&self) -> MsrEnergy {
        let mut file = self.handle.lock().unwrap();

        let package_energy_uj = read(&mut file, MsrOffset::PackageEnergy) - self.package_energy_uj;
        let core_energy_uj = read(&mut file, MsrOffset::CoreEnergy) - self.core_energy_uj;

        MsrEnergy { package_energy_uj, core_energy_uj }
    }

    fn elapsed_mut(&mut self) -> MsrEnergy {
        let prev_package_energy_uj = self.package_energy_uj;
        let prev_core_energy_uj = self.core_energy_uj;

        let elapsed = self.elapsed();

        self.package_energy_uj = prev_package_energy_uj;
        self.core_energy_uj = prev_core_energy_uj;

        elapsed
    }
}

fn read(file: &mut File, offset: MsrOffset) -> u64 {
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    let mut buf = [0; size_of::<u64>()];
    file.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}
