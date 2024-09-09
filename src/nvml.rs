use indexmap::IndexMap;

use crate::Energy;

pub struct Nvml {
    nvml: nvml_wrapper::Nvml,
    devices: IndexMap<u32, Result<u64, nvml_wrapper::error::NvmlError>>
}

impl Nvml {
    pub fn now() -> Option<Box<dyn Energy>> {
        let nvml = nvml_wrapper::Nvml::init().ok()?;
        let device_count = nvml.device_count().ok()?;
        let devices = (0..device_count).map(|i| (i, read_nvml(&nvml, i))).collect();
        Some(Box::new(Nvml { nvml, devices }))
    }
}

impl Energy for Nvml {
    fn elapsed(&self) -> IndexMap<String, f64> {
        self.devices.iter().map(|(&i, prev_energy)| {
            if let Ok(prev_energy) = prev_energy {
                let next_energy = read_nvml(&self.nvml, i).unwrap();
                (i.to_string(), (next_energy - prev_energy) as f64)
            } else {
                (i.to_string(), f64::NAN)
            }
        }).collect()
    }

    fn elapsed_mut(&mut self) -> IndexMap<String, f64> {
        todo!()
    }
}

fn read_nvml(nvml: &nvml_wrapper::Nvml, index: u32) -> Result<u64, nvml_wrapper::error::NvmlError> {
    let device = nvml.device_by_index(index)?;
    device.total_energy_consumption()
}
