use std::time::Duration;

use indexmap::{indexmap, IndexMap};

use crate::EnergyDuration;

pub struct HttpBuilder {
    pub path: String,
    pub header: String,
}

pub struct Http {
    path: String,
    header: String,
    agent: ureq::Agent,
    energy: f64,
}

impl EnergyDuration for Http {
    type Builder = HttpBuilder;

    fn now(builder: Self::Builder) -> Option<Box<Self>> {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .user_agent(&format!(
                "{} {}/{}",
                env!("CARGO_PKG_NAME"),
                option_env!("CI_COMMIT_REF_NAME").unwrap_or_default(),
                option_env!("CI_PIPELINE_IID").unwrap_or_default()
            ))
            .build();
        let energy = read(&agent, &builder.path, &builder.header).unwrap();
        Some(Box::new(Http { path: builder.path, header: builder.header, agent, energy }))
    }

    fn elapsed(&self) -> IndexMap<String, f64> {
        let energy = read(&self.agent, &self.path, &self.header).unwrap();
        let energy = energy - self.energy;
        indexmap!{
            self.header.clone() => energy,
        }
    }

    fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        let prev_energy = self.energy;
        self.energy = read(&self.agent, &self.path, &self.header).unwrap();
        let energy = (self.energy - prev_energy) / duration.as_secs_f64();
        indexmap!{
            self.header.clone() => energy,
        }
    }
}

fn read(agent: &ureq::Agent, path: &str, header: &str) -> Option<f64> {
    let resp = agent.get(path).call().ok()?;
    let str = resp.header(header)?;
    str.trim().parse::<f64>().ok()
}
