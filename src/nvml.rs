use indexmap::IndexMap;
use once_cell::sync::Lazy;

use crate::Energy;

static NVML: Lazy<Option<nvml_wrapper::Nvml>> = Lazy::new(|| nvml_wrapper::Nvml::init().ok());

pub struct Nvml<'a> {
    devices: Vec<NvmlDevice<'a>>,
}

pub struct NvmlDevice<'a> {
    device: nvml_wrapper::Device<'a>,
    name: String,
    energy: u64,
}

impl<'a> Nvml<'a> {
    pub fn now() -> Option<Box<dyn Energy>> {
        let nvml = NVML.as_ref()?;
        let count = nvml.device_count().ok()?;
        let devices = (0..count).filter_map(NvmlDevice::new).collect();
        Some(Box::new(Nvml { devices }))
    }
}

impl<'a> Energy for Nvml<'a> {
    fn elapsed(&self) -> IndexMap<String, f32> {
        self.devices.iter().map(|device| {
            let name = device.name.clone();
            let energy = device.elapsed();
            (name, energy)
        }).collect()
    }

    fn elapsed_mut(&mut self) -> IndexMap<String, f32> {
        self.devices.iter_mut().map(|device| {
            let name = device.name.clone();
            let energy = device.elapsed_mut();
            (name, energy)
        }).collect()
    }
}

impl<'a> NvmlDevice<'a> {
    fn new(index: u32) -> Option<NvmlDevice<'a>> {
        let nvml = NVML.as_ref()?;
        let device = nvml.device_by_index(index).ok()?;
        let name = format!("GPU({}) {}", index, device.name().ok()?);
        let energy = device.total_energy_consumption().ok()?;
        Some(NvmlDevice { device, name, energy })
    }

    fn elapsed(&self) -> f32 {
        let prev = self.energy;
        let next = self.next();
        diff(prev, next)
    }

    fn elapsed_mut(&mut self) -> f32 {
        let prev = self.energy;
        let next = self.next();
        self.energy = next;
        diff(prev, next)
    }

    fn next(&self) -> u64 {
        self.device.total_energy_consumption().unwrap()
    }
}

fn diff(prev: u64, next: u64) -> f32 {
    (next - prev) as f32 / 1000.0
}
