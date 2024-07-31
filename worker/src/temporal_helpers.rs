use anyhow::anyhow;
use futures::future::BoxFuture;
use futures::FutureExt;
use temporal_sdk::{ActContext, ActivityError, ActivityOptions, ChildWorkflowOptions, WfContext, WfExitValue};
use temporal_sdk_core::protos::coresdk::activity_result::activity_resolution;
use temporal_sdk_core::protos::coresdk::child_workflow::child_workflow_result;
use temporal_sdk_core::protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};
use std::fmt::Debug;
use std::future::Future;

/// Mainly copied from https://github.com/h7kanna/temporal-rust-worker
/// This only works with workflows and activities with one input payload and that should be fine for most use cases. 
/// If needed a proc marco would also work to increase the number of arguments, or just duplicate the code... 
///  
/// Trait to represent an async function with 2 arguments
pub trait AsyncFn<Arg0, Arg1>: Fn(Arg0, Arg1) -> Self::OutputFuture {
    /// Output type of the async function which implements serde traits
    type Output;
    /// Future of the output
    type OutputFuture: Future<Output = <Self as AsyncFn<Arg0, Arg1>>::Output> + Send + 'static;
}

impl<F: ?Sized, Fut, Arg0, Arg1> AsyncFn<Arg0, Arg1> for F
where
    F: Fn(Arg0, Arg1) -> Fut,
    Fut: Future + Send + 'static,
{
    type Output = Fut::Output;
    type OutputFuture = Fut;
}

/// Execute activity which takes [ActContext] and an argument and returns a [Result] with 'R'
/// and [anyhow::Error] where 'R' implements [serde] traits for
/// serialization into Temporal [Payload](temporal_sdk_core_protos::temporal::api::common::v1::Payload).
/// Use [into_activity] to register the activity with the worker.
pub async fn execute_activity<A, F, R>(
    ctx: &WfContext,
    options: ActivityOptions,
    _f: F,
    a: A,
) -> Result<R, anyhow::Error>
where
    F: AsyncFn<ActContext, A, Output = Result<R, ActivityError>> + Send + Sync + 'static,
    A: AsJsonPayloadExt + Debug,
    R: FromJsonPayloadExt + Debug,
{
    let input = A::as_json_payload(&a).expect("serializes fine");
    let activity_type = if options.activity_type.is_empty() {
        std::any::type_name::<F>().to_string()
    } else {
        options.activity_type
    };
    let options = ActivityOptions {
        activity_type,
        input,
        ..options
    };
    let activity_resolution = ctx.activity(options).await;

    match activity_resolution.status {
        Some(status) => match status {
            activity_resolution::Status::Completed(success) => {
                Ok(R::from_json_payload(&success.result.unwrap()).unwrap())
            }
            activity_resolution::Status::Failed(failure) => Err(anyhow::anyhow!("{:?}", failure)),
            activity_resolution::Status::Cancelled(reason) => Err(anyhow::anyhow!("{:?}", reason)),
            activity_resolution::Status::Backoff(reason) => Err(anyhow::anyhow!("{:?}", reason)),
        },
        None => panic!("activity task failed {activity_resolution:?}"),
    }
}

/// Register child workflow which takes [WfContext] and an argument and returns a [Result] with 'R'
/// and [anyhow::Error] where 'R' implements [serde] traits for
/// serialization into Temporal [Payload](temporal_sdk_core_protos::temporal::api::common::v1::Payload).
/// Use [execute_child_workflow] to execute the workflow in the workflow definition.
pub fn into_workflow<A, F, R, O>(
    f: F,
) -> impl Fn(WfContext) -> BoxFuture<'static, Result<WfExitValue<O>, anyhow::Error>> + Send + Sync
where
    A: FromJsonPayloadExt + Send,
    F: AsyncFn<WfContext, A, Output = Result<R, anyhow::Error>> + Send + Sync + 'static,
    R: Into<WfExitValue<O>>,
    O: AsJsonPayloadExt + Debug,
{
    move |ctx: WfContext| match A::from_json_payload(&ctx.get_args()[0]) {
        Ok(a) => (f)(ctx, a).map(|r| r.map(|r| r.into())).boxed(),
        Err(e) => async move { Err(e.into()) }.boxed(),
    }
}

/// Execute child workflow which takes [WfContext] and an argument and returns a [Result] with 'R'
/// and [anyhow::Error] where 'R' implements [serde] traits for
/// serialization into Temporal [Payload](temporal_sdk_core_protos::temporal::api::common::v1::Payload).
/// Use [into_workflow] to register the workflow with the worker
pub async fn execute_child_workflow<A, F, R>(
    ctx: &WfContext,
    options: ChildWorkflowOptions,
    _f: F,
    a: A,
) -> Result<R, anyhow::Error>
where
    F: AsyncFn<WfContext, A, Output = Result<R, anyhow::Error>> + Send + Sync + 'static,
    A: AsJsonPayloadExt + Debug,
    R: FromJsonPayloadExt + Debug,
{
    let input = A::as_json_payload(&a).expect("serializes fine");
    let workflow_type = if options.workflow_type.is_empty() {
        std::any::type_name::<F>().to_string()
    } else {
        options.workflow_type
    };

    let child = ctx.child_workflow(ChildWorkflowOptions {
        workflow_type,
        input: vec![input],
        ..options
    });

    let started = child
        .start(ctx)
        .await
        .into_started()
        .expect("Child should start OK");

    match started.result().await.status {
        Some(status) => match status {
            child_workflow_result::Status::Completed(success) => {
                Ok(R::from_json_payload(&success.result.unwrap()).unwrap())
            }
            child_workflow_result::Status::Failed(failure) => Err(anyhow::anyhow!("{:?}", failure)),
            child_workflow_result::Status::Cancelled(reason) => {
                Err(anyhow::anyhow!("{:?}", reason))
            }
        },
        None => Err(anyhow!("Unexpected child WF status")),
    }
}