use std::{io::Read, path::PathBuf};

use wasmer::{Engine, Module};
use wasmer_wasix::{
    Pipe,
    runners::wasi::{RuntimeOrEngine, WasiRunner},
};

use crate::{
    configs::ExecutingConfig,
    error::{AgentError, AgentResult},
};

pub struct WasmRuntime {}

impl WasmRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_execute_config(&self) -> ExecutingConfig {
        todo!()
    }

    pub async fn execute(&self) -> AgentResult<i32> {
        let config = self.make_execute_config();
        execute(config).await
    }

    pub fn execute_blocking(&self) -> AgentResult<i32> {
        let config = self.make_execute_config();
        execute_blocking(config)
    }
}

async fn execute(config: ExecutingConfig) -> AgentResult<i32> {
    tokio::task::spawn_blocking(move || execute_blocking(config))
        .await
        .map_err(AgentError::TaskError)?
}

fn execute_blocking(config: ExecutingConfig) -> AgentResult<i32> {
    let engine = Engine::default();

    let wasm_bytes = std::fs::read(config.wasm_path).map_err(AgentError::IoError)?;

    let module = Module::new(&engine, &wasm_bytes).map_err(AgentError::CompileError)?;

    let (stdout_tx, mut stdout_rx) = Pipe::channel();

    {
        let mut runner = WasiRunner::new();
        runner.with_stdout(Box::new(stdout_tx));

        runner
            .run_wasm(
                RuntimeOrEngine::Engine(engine),
                &config.name,
                module,
                wasmer_types::ModuleHash::xxhash(wasm_bytes),
            )
            .map_err(AgentError::AnyError)?;
    }

    let mut buf = String::new();
    stdout_rx.read_to_string(&mut buf).unwrap();

    println!("stdout: {}", buf);

    Ok(0)
}
