use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::iter::once;
use std::mem;
use std::sync::Mutex;

use indexmap::{indexmap, IndexMap};

use crate::Energy;

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

impl Msr {
    pub fn now() -> Option<Box<dyn Energy>> {
        let core0 = once(Core::now(0)?);
        let cores = core0.chain((1..u8::MAX).map_while(Core::now)).collect();
        Some(Box::new(Msr { cores }))
    }
}

impl Energy for Msr {
    fn elapsed(&self) -> IndexMap<String, f32> {
        self.cores.iter().flat_map(Core::elapsed).collect()
    }

    fn reset(&mut self) {
        self.cores.iter_mut().for_each(Core::reset);
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

    fn elapsed(&self) -> IndexMap<String, f32> {
        let mut file = self.handle.lock().unwrap();
        let package = read(&mut file, Offset::PackageEnergy);
        let core = read(&mut file, Offset::CoreEnergy);

        indexmap!{
            format!("cpu{}:package", self.package_id) => self.unit.joules(package - self.package_unit),
            format!("cpu{}:core", self.package_id) => self.unit.joules(core - self.core_unit),
        }
    }

    fn reset(&mut self) {
        let mut file = self.handle.lock().unwrap();
        self.package_unit = read(&mut file, Offset::PackageEnergy);
        self.core_unit = read(&mut file, Offset::CoreEnergy);
    }
}

#[allow(unused)]
struct Unit {
    time: f32,
    energy: f32,
    power: f32,
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

    pub fn joules(&self, unit: u64) -> f32 {
        unit as f32 * self.energy
    }
}

#[repr(u64)]
enum Mask {
    TimeUnit   = 0b11110000000000000000,
    EnergyUnit = 0b00000001111100000000,
    PowerUnit  = 0b00000000000000001111,
}

impl Mask {
    pub fn mask(self, units: u64) -> f32 {
        let mask = self as u64;
        let unit = (units & mask) >> mask.trailing_zeros();
        0.5f32.powi(unit as i32)
    }
}

fn read(file: &mut File, offset: Offset) -> u64 {
    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    let mut buf = [0; mem::size_of::<u64>()];
    file.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}
