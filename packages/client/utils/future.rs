use common::vendor::{
    leptos::{on_cleanup, request_animation_frame, set_timeout, spawn_local},
    time::Duration,
};
use futures::{channel, select, Future, FutureExt};

pub async fn sleep(time: Duration) {
    let (tx, rx) = channel::oneshot::channel();

    set_timeout(
        move || {
            _ = tx.send(());
        },
        time.try_into().unwrap(),
    );

    rx.await.unwrap();
}

pub fn spawn_local_with_handle(
    future: impl Future<Output = ()> + 'static,
) -> impl FnOnce() -> Result<(), ()> {
    let mut future_fuse = Box::pin(future).fuse();
    let (tx, rx) = channel::oneshot::channel();

    spawn_local(async move {
        let mut rx_fuse = rx.fuse();

        select! {
            () = future_fuse => (),
            _ = rx_fuse => ()
        };
    });

    move || tx.send(())
}

pub async fn next_tick() {
    let (tx, rx) = channel::oneshot::channel();

    request_animation_frame(move || {
        tx.send(()).unwrap();
    });

    rx.await.unwrap();
}

pub fn spawn_local_owned(future: impl Future<Output = ()> + 'static) {
    let handle = spawn_local_with_handle(future);
    on_cleanup(move || {
        _ = handle();
    });
}
