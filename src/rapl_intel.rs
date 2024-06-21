use std::fmt::Debug;
use std::fs::{ File, OpenOptions };
use std::io::Read;
use std::mem::size_of;
use std::sync::Mutex;

use crate::RaplReader;

pub struct RaplIntel {
    handle: Mutex<File>,
    energy_uj: u64,
}

impl RaplReader for RaplIntel {
    fn now(package_id: usize) -> Option<Self> {
        let path = format!("/sys/class/powercap/intel-rapl:{}/energy_uj", package_id);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;
        let energy_uj = read_raw(&mut file);
        let handle = Mutex::new(file);
        Some(RaplIntel { handle, energy_uj })
    }

    fn elapsed(&self) -> u64 {
        let mut file = self.handle.lock().unwrap();
        let energy_uj = read_raw(&mut file);
        self.energy_uj - energy_uj
    }
}

impl Debug for RaplIntel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}μJ", self.energy_uj))
    }
}

fn read_raw(file: &mut File) -> u64 {
    let mut buf = [0; size_of::<u64>()];
    file.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}
