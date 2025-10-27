use std::path::PathBuf;

pub struct PluginManagerConfig {}

pub struct ExecutingConfig {
    pub id: String,
    pub name: String,
    pub wasm_path: PathBuf,
}
