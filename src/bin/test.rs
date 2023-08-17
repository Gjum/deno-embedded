use std::path::Path;
use std::rc::Rc;

use deno_core::error::AnyError;
use deno_runtime::deno_core::FsModuleLoader;
use deno_core::ModuleSpecifier;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
  let js_path = Path::new("example.js");
  let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
  let mut worker = MainWorker::bootstrap_from_options(
    main_module.clone(),
    PermissionsContainer::allow_all(),
    WorkerOptions {
      module_loader: Rc::new(FsModuleLoader),
      ..Default::default()
    },
  );
  worker.execute_main_module(&main_module).await?;
  worker.run_event_loop(/*wait_for_inspector=*/false).await?;
  Ok(())
}
