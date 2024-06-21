use std::time::Instant;

use rapl_energy as rapl;

pub struct RaplMock {
    handle: Instant,
}

impl rapl::RaplReader for RaplMock {
    fn now(package_id: u8) -> Option<Self> {
        if package_id < 4 {
            let handle = Instant::now();
            Some(RaplMock { handle })
        } else {
            None
        }
    }

    fn elapsed(&self) -> u64 {
        self.handle.elapsed().as_micros() as u64
    }
}

#[test]
fn dependency_injection() {
    let packages = rapl::packages::<RaplMock>();
    assert_eq!(4, packages.len());
    let elapsed = rapl::elapsed(&packages);
    assert_eq!(4, elapsed.len());
}
