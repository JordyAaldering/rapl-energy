use indexmap::indexmap;

use crate::{Probe, Elapsed};

pub struct Http {
    path: String,
    header: String,
    agent: ureq::Agent,
    value: f32,
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
        let value = read(&agent, &path, &header)?;
        Some(Self { path, header, agent, value })
    }
}

impl Probe for Http {
    fn elapsed(&self) -> Elapsed {
        let prev = self.value;
        let next = read(&self.agent, &self.path, &self.header).unwrap();
        indexmap!{
            self.header.clone() => next - prev,
        }
    }

    fn reset(&mut self) {
        self.value = read(&self.agent, &self.path, &self.header).unwrap()
    }
}

fn read(agent: &ureq::Agent, path: &str, header: &str) -> Option<f32> {
    let resp = agent.get(path).call().ok()?;
    let str = resp.headers().get(header)?.to_str().ok()?;
    let value = str.trim().parse::<f32>().expect(&format!("Could not parse {}", str));
    Some(value)
}
