use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::sync::Mutex;

pub struct Msr {
    cores: Vec<MsrCore>,
}

struct MsrCore {
    handle: Mutex<File>,
    core_energy_uj: u64,
}

impl Msr {
    pub fn now() -> Self {
        let cores = (0..256).map_while(MsrCore::now).collect();
        Msr { cores }
    }

    pub fn elapsed(&self) -> Vec<u64> {
        self.cores.iter().map(|core| core.elapsed()).collect()
    }
}

impl MsrCore {
    fn now(package_id: usize) -> Option<Self> {
        let path = format!("/dev/cpu/{}/msr", package_id);
        let mut file = OpenOptions::new().read(true).write(true).open(&path).ok()?;
        let core_energy_uj = read_raw(&mut file);
        let handle = Mutex::new(file);
        Some(MsrCore { handle, core_energy_uj })
    }

    fn elapsed(&self) -> u64 {
        let mut file = self.handle.lock().unwrap();
        let core_energy_uj = read_raw(&mut file);
        core_energy_uj - self.core_energy_uj
    }
}

fn read_raw(file: &mut File) -> u64 {
    // const MSR_PACKAGE_ENERGY: u64 = 0xC001029B;
    const MSR_CORE_ENERGY: u64 = 0xC001029A;
    file.seek(SeekFrom::Start(MSR_CORE_ENERGY)).unwrap();
    let mut buf = [0; size_of::<u64>()];
    file.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}
