// main.rs
use std::rc::Rc;
use deno_core::{op2, extension};
use deno_core::anyhow::Error;

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, Error> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op2(async)]
#[string]
async fn op_write_file(#[string] path: String, #[string] contents: String) -> Result<(), Error> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op2(fast)]
#[string]
fn op_remove_file(#[string] path: String) -> Result<(), Error> {
    std::fs::remove_file(path)?;
    Ok(())
}


async fn run_js(file_path: &str) -> Result<(), Error> {
    let main_module = deno_core::resolve_path(file_path, &std::env::current_dir()?)?;

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_ext::init_ops()],
        ..Default::default()
    });
    js_runtime.execute_script("[runjs:runtime.js]", include_str!("./runtime.js")).unwrap();

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

extension!(
    runjs_ext,
    ops = [op_read_file, op_write_file, op_remove_file],
    esm_entry_point = "ext:runjs/src/runtime.js",
    esm = ["src/runtime.js"],
);
