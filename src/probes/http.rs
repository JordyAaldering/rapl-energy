use indexmap::indexmap;

use crate::{Energy, ProbeEnergy};

pub struct Http {
    path: String,
    header: String,
    agent: ureq::Agent,
    energy: f32,
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
        let energy = read(&agent, &path, &header)?;
        Some(Box::new(Self { path, header, agent, energy }))
    }

    fn read(&self) -> f32 {
        read(&self.agent, &self.path, &self.header).unwrap()
    }
}

impl Energy for Http {
    fn elapsed(&self) -> ProbeEnergy {
        let prev = self.energy;
        let next = self.read();
        indexmap!{
            self.header.clone() => next - prev,
        }
    }

    fn reset(&mut self) {
        self.energy = self.read();
    }
}

fn read(agent: &ureq::Agent, path: &str, header: &str) -> Option<f32> {
    let resp = agent.get(path).call().ok()?;
    let str = resp.header(header)?;
    let energy = str.trim().parse::<f32>().expect(&format!("Could not parse {}", str));
    Some(energy)
}
