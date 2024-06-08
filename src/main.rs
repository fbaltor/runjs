// main.rs
use std::rc::Rc;

async fn run_js(file_path: &str) -> Result<(), deno_core::anyhow::Error> {
    let main_module = deno_core::resolve_path(file_path, &std::env::current_dir()?)?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    let poll_ev_options = deno_core::PollEventLoopOptions {
        wait_for_inspector: false,
        pump_v8_message_loop: false,
    };
    js_runtime.run_event_loop(poll_ev_options).await?;
    result.await
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js("example.js")) {
        eprintln!("error: {}", error);
    }
}
