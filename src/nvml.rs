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
    fn elapsed(&self) -> IndexMap<String, f64> {
        self.devices.iter().map(|device| {
            let name = device.name.clone();
            let energy = device.elapsed();
            (name, energy)
        }).collect()
    }

    fn elapsed_mut(&mut self) -> IndexMap<String, f64> {
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

    fn elapsed(&self) -> f64 {
        let prev_energy = self.energy;
        let next_energy = self.device.total_energy_consumption().unwrap();
        (next_energy - prev_energy) as f64 / 1000.0
    }

    fn elapsed_mut(&mut self) -> f64 {
        let prev_energy = self.energy;
        let next_energy = self.device.total_energy_consumption().unwrap();
        self.energy = next_energy;
        (next_energy - prev_energy) as f64 / 1000.0
    }
}
