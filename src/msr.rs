use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::sync::Mutex;
use std::time::Duration;

use indexmap::{indexmap, IndexMap};

pub struct Msr {
    #[allow(dead_code)]
    time_unit: f64,
    energy_unit: f64,
    #[allow(dead_code)]
    power_unit: f64,
    cores: Vec<MsrCore>,
}

struct MsrCore {
    package_id: u8,
    handle: Mutex<File>,
    package_energy_j: u64,
    core_energy_j: u64,
}

#[repr(u64)]
enum MsrOffset {
    PowerUnit     = 0xC0010299,
    CoreEnergy    = 0xC001029A,
    PackageEnergy = 0xC001029B,
}

impl Msr {
    pub fn now() -> Self {
        let path = format!("/dev/cpu/0/msr");
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let units = read(&mut file, MsrOffset::PowerUnit);

        const TIME_UNIT_MASK: u64   = 0xF0000;
        const ENERGY_UNIT_MASK: u64 = 0x01F00;
        const POWER_UNIT_MASK: u64  = 0x0000F;

        let time_unit   = (units & TIME_UNIT_MASK)   >> 16;
        let energy_unit = (units & ENERGY_UNIT_MASK) >> 8;
        let power_unit  = (units & POWER_UNIT_MASK)  >> 0;

        let time_unit   = 0.5f64.powi(time_unit   as i32);
        let energy_unit = 0.5f64.powi(energy_unit as i32);
        let power_unit  = 0.5f64.powi(power_unit  as i32);

        let cores = (0..u8::MAX).map_while(MsrCore::now).collect();
        Msr { time_unit, energy_unit, power_unit, cores }
    }

    pub fn elapsed(&self) -> IndexMap<String, f64> {
        self.cores.iter().flat_map(|core| core.elapsed(self.energy_unit)).collect()
    }

    pub fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        self.cores.iter_mut().flat_map(|core| core.power(self.energy_unit, duration)).collect()
    }
}

impl MsrCore {
    fn now(package_id: u8) -> Option<Self> {
        let path = format!("/dev/cpu/{}/msr", package_id);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;
        Some(MsrCore {
            package_id,
            package_energy_j: read(&mut file, MsrOffset::PackageEnergy),
            core_energy_j: read(&mut file, MsrOffset::CoreEnergy),
            handle: Mutex::new(file),
        })
    }

    fn elapsed(&self, energy_unit: f64) -> IndexMap<String, f64> {
        let mut file = self.handle.lock().unwrap();
        let package_energy_j = (read(&mut file, MsrOffset::PackageEnergy) - self.package_energy_j) as f64 * energy_unit;
        let core_energy_j = (read(&mut file, MsrOffset::CoreEnergy) - self.core_energy_j) as f64 * energy_unit;

        indexmap!{
            format!("cpu{}:package", self.package_id) => package_energy_j,
            format!("cpu{}:core", self.package_id) => core_energy_j,
        }
    }

    fn power(&mut self, energy_unit: f64, duration: Duration) -> IndexMap<String, f64> {
        let package_prev = self.package_energy_j;
        let core_prev = self.core_energy_j;

        let mut file = self.handle.lock().unwrap();
        self.package_energy_j = read(&mut file, MsrOffset::PackageEnergy);
        self.core_energy_j = read(&mut file, MsrOffset::CoreEnergy);

        let package_j = (self.package_energy_j - package_prev) as f64 * energy_unit;
        let core_j = (self.core_energy_j - core_prev) as f64 * energy_unit;
        let package_power_w = package_j / duration.as_secs_f64();
        let core_power_w = core_j / duration.as_secs_f64();

        indexmap!{
            format!("cpu{}:package", self.package_id) => package_power_w,
            format!("cpu{}:core", self.package_id) => core_power_w,
        }
    }
}

fn read(file: &mut File, offset: MsrOffset) -> u64 {
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    let mut buf = [0; size_of::<u64>()];
    file.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}