use futures::Future;
use leptos::on_cleanup;

use crate::utils::future::spawn_local_with_handle;

pub fn spawn_local_owned<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    let handle = spawn_local_with_handle(future);
    on_cleanup(move || {
        _ = handle();
    });
}
