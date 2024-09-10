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

    fn next(&self) -> f64 {
        read(&self.agent, &self.path, &self.header).unwrap()
    }
}

impl Energy for Http {
    fn elapsed(&self) -> IndexMap<String, f64> {
        let prev = self.energy;
        let next = self.next();

        indexmap!{
            self.header.clone() => next - prev,
        }
    }

    fn elapsed_mut(&mut self) -> IndexMap<String, f64> {
        let prev = self.energy;
        let next = self.next();
        self.energy = next;

        indexmap!{
            self.header.clone() => next - prev,
        }
    }
}

fn read(agent: &ureq::Agent, path: &str, header: &str) -> Option<f64> {
    let resp = agent.get(path).call().ok()?;
    let str = resp.header(header)?;
    let energy = str.trim().parse::<f64>().unwrap();
    Some(energy)
}
