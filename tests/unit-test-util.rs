#[maybe_async::both]
async fn some_function() -> bool {
    true
}

#[maybe_async::test(feature = "is_sync", async(not(feature = "is_sync"), async_std::test))]
async fn test_async_fn() {
    let res = some_function().await;
    assert_eq!(res, true);
}

#[maybe_async::test(feature = "is_sync", async(not(feature = "is_sync"), tokio::test))]
async fn test_async_fn2() {
    let res = some_function().await;
    assert_eq!(res, true);
}

#[maybe_async::test(feature = "is_sync")]
async fn test_async_fn3() {
    let res = some_function().await;
    assert_eq!(res, true);
}

#[maybe_async::test(feature = "is_sync")]
async fn test_sync_fn() {
    let res = some_function();
    assert_eq!(res, true);
}
