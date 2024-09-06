use std::time::Duration;

use indexmap::IndexMap;

pub trait EnergyDuration {
    fn elapsed(&self) -> IndexMap<String, f64>;

    fn power(&mut self, duration: Duration) -> IndexMap<String, f64>;
}
