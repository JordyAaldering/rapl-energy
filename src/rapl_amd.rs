use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::sync::Mutex;

use crate::RaplReader;

pub struct RaplAMD {
    handle: Mutex<File>,
    package_id: usize,
    energy_uj: u64,
}

impl RaplReader for RaplAMD {
    fn now(package_id: usize) -> Option<Self> {
        let path = format!("/dev/cpu/{}/msr", package_id);
        let mut file = OpenOptions::new().read(true).write(true).open(&path).ok()?;
        let energy_uj = read_raw(&mut file);
        let handle = Mutex::new(file);
        Some(RaplAMD { handle, package_id, energy_uj })
    }

    fn elapsed(&self) -> u64 {
        let mut file = self.handle.lock().unwrap();
        let energy_uj = read_raw(&mut file);
        self.energy_uj - energy_uj
    }

    fn label(&self) -> String {
        format!("cpu/{}", self.package_id)
    }
}

fn read_raw(file: &mut File) -> u64 {
    const MSR_PACKAGE_ENERGY: u64 = 0xC001029B;
    file.seek(SeekFrom::Start(MSR_PACKAGE_ENERGY)).unwrap();
    let mut buf = [0; size_of::<u64>()];
    file.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}
