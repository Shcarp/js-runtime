use std::future::Future;

pub fn run_loacal_future<R>(fut: impl Future<Output = R>){
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(fut);
}
