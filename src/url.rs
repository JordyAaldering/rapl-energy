use std::time::Duration;

use indexmap::{indexmap, IndexMap};

#[allow(dead_code)]
pub struct Url {
    agent: ureq::Agent,
    url: String,
    header: String,
    energy: f64,
}

impl Url {
    pub fn now(url: String, header: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .user_agent(&format!(
                "{} {}/{}",
                env!("CARGO_PKG_NAME"),
                option_env!("CI_COMMIT_REF_NAME").unwrap_or_default(),
                option_env!("CI_PIPELINE_IID").unwrap_or_default()
            ))
            .build();
        let energy = read(&agent, &url, &header).unwrap();
        Url { agent, url, header, energy }
    }

    pub fn elapsed(&self) -> IndexMap<String, f64> {
        let energy = read(&self.agent, &self.url, &self.header).unwrap();
        let energy = energy - self.energy;
        indexmap!{
            self.header.clone() => energy,
        }
    }

    pub fn power(&mut self, duration: Duration) -> IndexMap<String, f64> {
        let prev_energy = self.energy;
        self.energy = read(&self.agent, &self.url, &self.header).unwrap();
        let energy = (self.energy - prev_energy) / duration.as_secs_f64();
        indexmap!{
            self.header.clone() => energy,
        }
    }
}

fn read(agent: &ureq::Agent, url: &str, header: &str) -> Option<f64> {
    let resp = agent.get(url).call().ok()?;
    let str = resp.header(header)?;
    str.trim().parse::<f64>().ok()
}
