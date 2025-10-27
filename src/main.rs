use agent4linn::runtime::WasmRuntime;

#[tokio::main]
async fn main() {
    let runtime = WasmRuntime::new();
    runtime.execute().await.unwrap();
}
