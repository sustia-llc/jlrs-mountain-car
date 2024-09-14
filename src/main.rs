use jlrs::{async_util::task::Register, data::managed::array::Array, prelude::*};
struct MountainCarTask {}

#[async_trait(?Send)]
impl AsyncTask for MountainCarTask {
    type Output = JlrsResult<Vec<f64>>;

    async fn run<'base>(&mut self, mut frame: AsyncGcFrame<'base>) -> Self::Output {
        frame
            .async_scope(|mut frame| async move {
                let result = unsafe {
                    Module::main(&frame)
                        .submodule(&frame, "MountainCarAI")?
                        .as_managed()
                        .function(&frame, "run_simulation")?
                        .as_managed()
                        .call_async(&mut frame, &[])
                        .await
                        .into_jlrs_result()?
                };
                
                let positions: Vec<f64> = unsafe {
                    let array = result.cast::<Array>()?;
                    let data = array.data_ptr();
                    let slice = std::slice::from_raw_parts(data.cast::<f64>(), array.length());
                    slice.to_vec()
                };
                Ok(positions)
            })
            .await
    }
}

#[async_trait(?Send)]
impl Register for MountainCarTask {
    async fn register<'base>(mut frame: AsyncGcFrame<'base>) -> JlrsResult<()> {
        unsafe {
            Value::eval_string(&mut frame, "push!(LOAD_PATH, \"../MountainCarAI\")")
                .into_jlrs_result()?;
            Value::eval_string(&mut frame, "using MountainCarAI").into_jlrs_result()?;
        }
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (julia, handle) = Builder::new()
        .async_runtime(Tokio::<1>::new(false))
        .channel_capacity(4)
        .spawn()
        .expect("Could not init Julia");

    julia
        .register_task::<MountainCarTask>()
        .dispatch()
        .await
        .expect("channel closed")
        .await
        .expect("channel closed")
        .expect("registration failed");

    let t1 = julia
        .task(MountainCarTask {})
        .dispatch()
        .await
        .expect("channel closed");

    let result: Vec<f64> = t1.await.unwrap().unwrap();
    println!("pos1: {}, pos2: {}", result[0], result[1]);

    std::mem::drop(julia);
    handle.join().unwrap();
}
