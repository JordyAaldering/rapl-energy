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

#[derive(serde::Serialize)]
pub struct MsrCoreEnergy {
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
        let cores = (0..256).map_while(MsrCore::now).collect();
        Msr { cores }
    }

    pub fn elapsed(&self) -> Vec<MsrCoreEnergy> {
        self.cores.iter().map(MsrCore::elapsed).collect()
    }
}

impl MsrCore {
    fn now(package_id: usize) -> Option<Self> {
        let path = format!("/dev/cpu/{}/msr", package_id);
        let mut file = OpenOptions::new().read(true).write(true).open(&path).ok()?;

        let package_energy_uj = read(&mut file, MsrOffset::PackageEnergy);
        let core_energy_uj = read(&mut file, MsrOffset::CoreEnergy);

        let handle = Mutex::new(file);
        Some(MsrCore { handle, package_energy_uj, core_energy_uj })
    }

    fn elapsed(&self) -> MsrCoreEnergy {
        let mut file = self.handle.lock().unwrap();

        let package_energy_uj = read(&mut file, MsrOffset::PackageEnergy) - self.package_energy_uj;
        let core_energy_uj = read(&mut file, MsrOffset::CoreEnergy) - self.core_energy_uj;

        MsrCoreEnergy { package_energy_uj, core_energy_uj }
    }
}

fn read(file: &mut File, offset: MsrOffset) -> u64 {
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    let mut buf = [0; size_of::<u64>()];
    file.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}

impl std::ops::Add for MsrCoreEnergy {
    type Output = MsrCoreEnergy;

    fn add(self, rhs: Self) -> Self::Output {
        let package_energy_uj = self.package_energy_uj + rhs.package_energy_uj;
        let core_energy_uj = self.core_energy_uj + rhs.core_energy_uj;
        MsrCoreEnergy { package_energy_uj, core_energy_uj }
    }
}

impl std::ops::Sub for MsrCoreEnergy {
    type Output = MsrCoreEnergy;

    fn sub(self, rhs: Self) -> Self::Output {
        let package_energy_uj = self.package_energy_uj - rhs.package_energy_uj;
        let core_energy_uj = self.core_energy_uj - rhs.core_energy_uj;
        MsrCoreEnergy { package_energy_uj, core_energy_uj }
    }
}
