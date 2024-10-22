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
        let energy = read(&device);
        Some(Box::new(Self { name, device, energy }))
    }

    pub fn all_with_energy() -> Vec<Box<dyn Energy>> {
        let devices = libmedium::parse_hwmons().unwrap();
        devices.into_iter()
            .filter_map(|device| Hwmon::now(device.clone()))
            .collect()
    }
}

impl Energy for Hwmon {
    fn elapsed(&self) -> IndexMap<String, f32> {
        let prev = &self.energy;
        let next = read(&self.device);
        next.into_iter().map(|(key, next)| {
            let name = self.name.clone();
            let energy = next - prev[&key];
            (name, energy as f32)
        }).collect()
    }

    fn reset(&mut self) {
        self.energy = read(&self.device);
    }
}

fn read(device: &libmedium::hwmon::sync_hwmon::Hwmon) -> IndexMap<u16, u64> {
    device.energies()
        .iter()
        .filter_map(|(&key, sensor)| {
            let str = sensor.read_raw(SensorSubFunctionType::Input).ok()?;
            let energy = str.trim().parse::<u64>().expect(&format!("Could not parse {}", str));
            Some((key, energy))
        }).collect()
}
