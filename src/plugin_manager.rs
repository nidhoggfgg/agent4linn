use std::sync::Arc;

use crate::{configs::PluginManagerConfig, runtime::WasmRuntime};

pub struct PluginManager {
    config: PluginManagerConfig,
    runtime: Arc<WasmRuntime>,
}

impl PluginManager {
    fn new() -> Self {
        todo!()
    }
}
