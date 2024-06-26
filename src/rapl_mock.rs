use std::time::Instant;

use crate::RaplReader;

pub struct RaplMock {
    handle: Instant,
    package_id: usize,
}

impl RaplReader for RaplMock {
    fn now(package_id: usize) -> Option<Self> {
        if package_id < 4 {
            let handle = Instant::now();
            Some(RaplMock { handle, package_id })
        } else {
            None
        }
    }

    fn elapsed(&self) -> u64 {
        self.handle.elapsed().as_micros() as u64 * (self.package_id + 1) as u64
    }

    fn label(&self) -> String {
        self.package_id.to_string()
    }
}
