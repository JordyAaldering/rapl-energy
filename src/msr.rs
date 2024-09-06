use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::mem;
use std::sync::Mutex;
use std::time::Duration;

use indexmap::{indexmap, IndexMap};

use crate::energy_duration::EnergyDuration;

pub struct Msr {
    cores: Vec<Core>,
}

struct Core {
    package_id: u8,
    handle: Mutex<File>,
    package_unit: u64,
    core_unit: u64,
    unit: Unit,
}

#[repr(u64)]
enum Offset {
    PowerUnit     = 0xC0010299,
    CoreEnergy    = 0xC001029A,
    PackageEnergy = 0xC001029B,
}

#[allow(unused)]
struct Unit {
    time: f64,
    energy: f64,
    power: f64,
}

#[repr(u64)]
enum Mask {
    TimeUnit   = 0b11110000000000000000,
    EnergyUnit = 0b00000001111100000000,
    PowerUnit  = 0b00000000000000001111,
}

impl Msr {
    pub fn now() -> Self {
        let cores = (0..u8::MAX).map_while(Core::now).collect();
        Msr { cores }
    }

    pub fn elapsed(&self) -> IndexMap<String, f64> {
        self.cores
            .iter()
            .flat_map(Core::elapsed)
            .collect()
    }

    pub fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        self.cores
            .iter_mut()
            .flat_map(|core| core.power(duration))
            .collect()
    }
}

impl EnergyDuration for Msr {
    fn elapsed(&self) -> IndexMap<String, f64> {
        self.cores.iter().flat_map(Core::elapsed).collect()
    }

    fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        self.cores
            .iter_mut()
            .flat_map(|core| core.power(duration))
            .collect()
    }
}

impl Core {
    fn now(package_id: u8) -> Option<Self> {
        let path = format!("/dev/cpu/{}/msr", package_id);
        let mut file = OpenOptions::new().read(true).open(&path).ok()?;
        Some(Core {
            package_id,
            unit: Unit::new(&mut file),
            package_unit: read(&mut file, Offset::PackageEnergy),
            core_unit: read(&mut file, Offset::CoreEnergy),
            handle: Mutex::new(file),
        })
    }

    fn elapsed(&self) -> IndexMap<String, f64> {
        let (package_next, core_next) = self.read();
        let package = package_next - self.package_unit;
        let core = core_next - self.core_unit;

        indexmap!{
            format!("cpu{}:package", self.package_id) => self.unit.joules(package),
            format!("cpu{}:core", self.package_id) => self.unit.joules(core),
        }
    }

    fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        let (package_next, core_next) = self.read();
        let package = package_next - self.package_unit;
        let core = core_next - self.core_unit;
        self.package_unit = package_next;
        self.core_unit = core_next;

        indexmap!{
            format!("cpu{}:package", self.package_id) => self.unit.watts(package, duration),
            format!("cpu{}:core", self.package_id) => self.unit.watts(core, duration),
        }
    }

    fn read(&self) -> (u64, u64) {
        let mut file = self.handle.lock().unwrap();
        let package = read(&mut file, Offset::PackageEnergy);
        let core = read(&mut file, Offset::CoreEnergy);
        (package, core)
    }
}

impl Unit {
    pub fn new(file: &mut File) -> Self {
        let units = read(file, Offset::PowerUnit);
        Unit {
            time: Mask::TimeUnit.mask(units),
            energy: Mask::EnergyUnit.mask(units),
            power: Mask::PowerUnit.mask(units),
        }
    }

    pub fn joules(&self, unit: u64) -> f64 {
        unit as f64 * self.energy
    }

    pub fn watts(&self, unit: u64, duration: Duration) -> f64 {
        (unit as f64 * self.energy) / duration.as_secs_f64()
    }
}

impl Mask {
    pub fn mask(self, units: u64) -> f64 {
        let mask = self as u64;
        let unit = (units & mask) >> mask.trailing_zeros();
        0.5f64.powi(unit as i32)
    }
}

fn read(file: &mut File, offset: Offset) -> u64 {
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    let mut buf = [0; mem::size_of::<u64>()];
    file.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}
