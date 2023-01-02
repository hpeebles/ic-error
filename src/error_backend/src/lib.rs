use ic_cdk_macros::{query, update};

#[query]
fn ping() -> String {
    "pong!".to_string()
}

#[update]
async fn error() -> bool {
    futures::future::join_all([async_then_panic(), async_then_panic()]).await;
    true
}

async fn async_then_panic() {
    do_something_async().await;
    panic!();
}

async fn do_something_async() {
    let _ = ic_cdk::api::management_canister::main::raw_rand().await;
}