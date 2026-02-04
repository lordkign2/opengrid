use opengrid_core::OpenGridEngine;

pub struct TestInterface {
    inner: OpenGridEngine,
}

impl TestInterface {
    pub fn new() -> Self {
        TestInterface {
            inner: OpenGridEngine::new(),
        }
    }
}

uniffi::setup_scaffolding!();