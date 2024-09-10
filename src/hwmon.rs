use std::mem::swap;

use indexmap::IndexMap;
use libmedium::sensors::{sync_sensors::SyncSensor, SensorSubFunctionType};

use crate::Energy;

pub struct Hwmon {
    name: String,
    device: libmedium::hwmon::sync_hwmon::Hwmon,
    energy: IndexMap<u16, u64>
}

impl Hwmon {
    pub fn now(device: libmedium::hwmon::sync_hwmon::Hwmon) -> Option<Box<dyn Energy>> {
        let name = device.name().to_string();
        let energy = device.energies()
            .iter()
            .filter_map(|(&key, sensor)| {
                let s = sensor.read_raw(SensorSubFunctionType::Input).ok()?;
                let v = s.trim().parse::<u64>().ok()?;
                Some((key, v))
            }).collect();
        Some(Box::new(Hwmon { name, device, energy }))
    }

    pub fn all_with_energy() -> Vec<Box<dyn Energy>> {
        let devices = libmedium::parse_hwmons().unwrap();
        devices.into_iter()
            .filter_map(|device| Hwmon::now(device.clone()))
            .collect()
    }
}

impl Energy for Hwmon {
    fn elapsed(&self) -> IndexMap<String, f64> {
        let prev = &self.energy;
        let next = hwmon_energy(&self.device);
        next.into_iter().map(|(key, next)| {
            let name = self.name.clone();
            let energy = next - prev[&key];
            (name, energy as f64)
        }).collect()
    }

    fn elapsed_mut(&mut self) -> IndexMap<String, f64> {
        let mut prev = hwmon_energy(&self.device);
        swap(&mut self.energy, &mut prev);

        self.energy.iter().map(|(&key, &next)| {
            let name = self.name.clone();
            let energy = next - prev[&key];
            (name, energy as f64)
        }).collect()
    }
}

fn hwmon_energy(hwmon: &libmedium::hwmon::sync_hwmon::Hwmon) -> IndexMap<u16, u64> {
    hwmon.energies()
        .iter()
        .filter_map(|(&key, sensor)| {
            let str = sensor.read_raw(libmedium::sensors::SensorSubFunctionType::Input).ok()?;
            let energy = str.trim().parse::<u64>().ok()?;
            Some((key, energy))
        }).collect()
}
