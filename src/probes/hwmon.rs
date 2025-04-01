use indexmap::IndexMap;
use libmedium::hwmon::sync_hwmon;
use libmedium::sensors::SensorSubFunctionType;
use libmedium::sensors::sync_sensors::SyncSensor;

use crate::{Probe, Elapsed};

pub struct Hwmon {
    name: String,
    device: sync_hwmon::Hwmon,
    energy: IndexMap<u16, u64>
}

impl Hwmon {
    pub fn now(device: sync_hwmon::Hwmon) -> Option<Self> {
        let name = device.name().to_string();
        let energy = read(&device);
        Some(Self { name, device, energy })
    }

    pub fn get_available() -> Vec<Self> {
        let devices = libmedium::parse_hwmons().unwrap();
        devices.into_iter()
            .filter_map(|device| Hwmon::now(device.clone()))
            .collect()
    }
}

impl Probe for Hwmon {
    fn elapsed(&self) -> Elapsed {
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

fn read(device: &sync_hwmon::Hwmon) -> IndexMap<u16, u64> {
    device.energies()
        .iter()
        .filter_map(|(&key, sensor)| {
            let str = sensor.read_raw(SensorSubFunctionType::Input).ok()?;
            let energy = str.trim().parse::<u64>().expect(&format!("Could not parse {}", str));
            Some((key, energy))
        }).collect()
}
