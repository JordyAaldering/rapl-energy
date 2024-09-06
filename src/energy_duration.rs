use std::time::Duration;

use indexmap::IndexMap;

pub trait EnergyDuration {
    type Builder;

    fn now(builder: Self::Builder) -> Option<Box<Self>>;

    fn elapsed(&self) -> IndexMap<String, f64>;

    fn power(&mut self, duration: Duration) -> IndexMap<String, f64>;
}
