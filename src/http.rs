use std::time::Duration;

use indexmap::{indexmap, IndexMap};

use crate::Energy;

pub struct Http {
    path: String,
    header: String,
    agent: ureq::Agent,
    energy: f64,
}

impl Http {
    pub fn now(path: String, header: String) -> Option<Box<dyn Energy>> {
        let agent = ureq::AgentBuilder::new()
            .user_agent(&format!(
                "{} {}/{}",
                env!("CARGO_PKG_NAME"),
                option_env!("CI_COMMIT_REF_NAME").unwrap_or_default(),
                option_env!("CI_PIPELINE_IID").unwrap_or_default()
            ))
            .build();
        let energy = read(&agent, &path, &header).unwrap();
        Some(Box::new(Http { path, header, agent, energy }))
    }
}

impl Energy for Http {
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
