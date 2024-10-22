use std::iter::once;

use indexmap::{indexmap, IndexMap};

use crate::file_handle::FileHandle;
use crate::Energy;

pub struct Msr {
    cores: Vec<Core>,
}

struct Core {
    package_id: u8,
    handle: FileHandle,
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
        let handle = FileHandle::new(&path).ok()?;
        Some(Core {
            package_id,
            unit: Unit::new(&handle),
            package_unit: handle.from_le_bytes(Offset::PackageEnergy as u64),
            core_unit: handle.from_le_bytes(Offset::CoreEnergy as u64),
            handle,
        })
    }

    fn elapsed(&self) -> IndexMap<String, f32> {
        let package = self.handle.from_le_bytes(Offset::PackageEnergy as u64);
        let core = self.handle.from_le_bytes(Offset::CoreEnergy as u64);
        indexmap!{
            format!("cpu{}:package", self.package_id) => self.unit.joules(package - self.package_unit),
            format!("cpu{}:core", self.package_id) => self.unit.joules(core - self.core_unit),
        }
    }

    fn reset(&mut self) {
        self.package_unit = self.handle.from_le_bytes(Offset::PackageEnergy as u64);
        self.core_unit = self.handle.from_le_bytes(Offset::CoreEnergy as u64);
    }
}

#[allow(unused)]
struct Unit {
    time: f32,
    energy: f32,
    power: f32,
}

impl Unit {
    pub fn new(handle: &FileHandle) -> Self {
        let units = handle.from_le_bytes(Offset::PowerUnit as u64);
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
