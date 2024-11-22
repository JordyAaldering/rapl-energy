use indexmap::indexmap;

use crate::file_handle::FileHandle;
use crate::{EnergyProbe, Energy};

pub struct Msr {
    cores: Vec<Core>,
}

struct Core {
    handle: FileHandle,
    package_id: u8,
    package: u64,
    core: u64,
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
    time: f32,
    energy: f32,
    power: f32,
}

#[repr(u64)]
enum Mask {
    TimeUnit   = 0b11110000000000000000,
    EnergyUnit = 0b00000001111100000000,
    PowerUnit  = 0b00000000000000001111,
}

impl Msr {
    pub fn now() -> Option<Self> {
        let head = Core::now(0)?;
        let tail = (1..u8::MAX).map_while(Core::now);
        let cores = std::iter::once(head).chain(tail).collect();
        Some(Self { cores })
    }

    pub fn as_energy(self) -> Box<dyn EnergyProbe> {
        Box::new(self)
    }
}

impl EnergyProbe for Msr {
    fn elapsed(&self) -> Energy {
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
            package: Offset::PackageEnergy.read(&handle),
            core: Offset::CoreEnergy.read(&handle),
            unit: Unit::new(&handle),
            handle,
        })
    }

    fn elapsed(&self) -> Energy {
        let package = Offset::PackageEnergy.read(&self.handle);
        let core = Offset::CoreEnergy.read(&self.handle);
        indexmap!{
            format!("msr-{}-package", self.package_id) => self.unit.joules(package - self.package),
            format!("msr-{}-core", self.package_id) => self.unit.joules(core - self.core),
        }
    }

    fn reset(&mut self) {
        self.package = Offset::PackageEnergy.read(&self.handle);
        self.core = Offset::CoreEnergy.read(&self.handle);
    }
}

impl Offset {
    pub fn read(self, handle: &FileHandle) -> u64 {
        handle.from_le_bytes(self as u64)
    }
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

impl Mask {
    pub fn mask(self, units: u64) -> f32 {
        let mask = self as u64;
        let unit = (units & mask) >> mask.trailing_zeros();
        0.5f32.powi(unit as i32)
    }
}
