use indexmap::indexmap;

use crate::{EnergyProbe, Energy};

pub struct Http {
    path: String,
    header: String,
    agent: ureq::Agent,
    energy: f32,
}

impl Http {
    pub fn now(path: String, header: String) -> Option<Self> {
        let config = ureq::Agent::config_builder()
            .user_agent(&format!(
                "{} {}/{}",
                env!("CARGO_PKG_NAME"),
                option_env!("CI_COMMIT_REF_NAME").unwrap_or_default(),
                option_env!("CI_PIPELINE_IID").unwrap_or_default()
            ))
            .build();
        let agent = ureq::Agent::new_with_config(config);
        let energy = read(&agent, &path, &header)?;
        Some(Self { path, header, agent, energy })
    }

    pub fn as_energy(self) -> Box<dyn EnergyProbe> {
        Box::new(self)
    }
}

impl EnergyProbe for Http {
    fn elapsed(&self) -> Energy {
        let prev = self.energy;
        let next = read(&self.agent, &self.path, &self.header).unwrap();
        indexmap!{
            self.header.clone() => next - prev,
        }
    }

    fn reset(&mut self) {
        self.energy = read(&self.agent, &self.path, &self.header).unwrap()
    }
}

fn read(agent: &ureq::Agent, path: &str, header: &str) -> Option<f32> {
    let resp = agent.get(path).call().ok()?;
    let str = resp.headers().get(header)?.to_str().ok()?;
    let energy = str.trim().parse::<f32>().expect(&format!("Could not parse {}", str));
    Some(energy)
}
