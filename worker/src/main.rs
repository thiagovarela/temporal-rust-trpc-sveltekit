use std::{str::FromStr, sync::Arc};
use db::database_pool;
use temporal_helpers::into_workflow;
use temporal_sdk::{sdk_client_options, Worker};
use temporal_sdk_core::{init_worker, Url, CoreRuntime};
use temporal_sdk_core_api::{worker::WorkerConfigBuilder, telemetry::TelemetryOptionsBuilder};

mod errors;
mod auth; 
mod db;
mod temporal_helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;
    let client = server_options.connect("default", None).await?;
    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;
    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("default")
        .worker_build_id("rust-sdk")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core_worker), "default");
    let db_str =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");
    let pool = database_pool(&db_str).await;
    worker.insert_app_data(pool);
    
    worker.register_wf("sign-up-wf", into_workflow(auth::sign_up_wf));
    worker.register_activity("sign-up-activity", auth::sign_up_activity);
    worker.register_wf("login-wf", into_workflow(auth::login_wf));
    worker.register_activity("login-activity", auth::login_activity);

    worker.run().await?;
    Ok(())
}