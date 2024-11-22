use once_cell::sync::Lazy;

use crate::{EnergyProbe, Energy};

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
    pub fn now() -> Option<Box<dyn EnergyProbe>> {
        let nvml = NVML.as_ref()?;
        let count = nvml.device_count().ok()?;
        let devices = (0..count).filter_map(NvmlDevice::new).collect();
        Some(Box::new(Nvml { devices }))
    }
}

impl<'a> EnergyProbe for Nvml<'a> {
    fn elapsed(&self) -> Energy {
        self.devices.iter().map(|device| {
            let name = device.name.clone();
            let energy = device.elapsed();
            (name, energy)
        }).collect()
    }

    fn reset(&mut self) {
        self.devices.iter_mut().for_each(NvmlDevice::reset);
    }
}

impl<'a> NvmlDevice<'a> {
    fn new(index: u32) -> Option<Self> {
        let nvml = NVML.as_ref()?;
        let device = nvml.device_by_index(index).ok()?;
        let name = format!("GPU({}) {}", index, device.name().ok()?);
        let energy = read(&device)?;
        Some(Self { device, name, energy })
    }

    fn elapsed(&self) -> f32 {
        let prev = self.energy;
        let next = read(&self.device).unwrap();
        (next - prev) as f32 / 1000.0
    }

    fn reset(&mut self) {
        self.energy = read(&self.device).unwrap();
    }
}

fn read<'a>(device: &nvml_wrapper::Device<'a>) -> Option<u64> {
    device.total_energy_consumption().ok()
}
