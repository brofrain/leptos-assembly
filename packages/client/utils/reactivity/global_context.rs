use std::cell::RefCell;

use common::vendor::leptos::{provide_context, use_context, with_owner, Owner};

thread_local! {
    static GLOBAL_CTX_OWNER: RefCell<Option<Owner>> = const { RefCell::new(None) };
}

pub fn provide_global_context() {
    GLOBAL_CTX_OWNER.with(|owner| {
        *owner.borrow_mut() =
            Some(Owner::current().expect("Reactive runtime should be present"));
    });
}

pub fn use_global_context_with_initializer<T>(
    initializer: impl FnOnce() -> T + 'static,
) -> T
where
    T: Copy + 'static,
{
    if let Some(ctx) = use_context::<T>() {
        return ctx;
    }

    with_owner(
        GLOBAL_CTX_OWNER
            .with(|owner| owner.borrow().expect("Owner should be provided")),
        move || {
            provide_context(initializer());
        },
    );

    use_context::<T>().unwrap()
}

pub fn use_global_context<T>() -> T
where
    T: Copy + Default + 'static,
{
    use_global_context_with_initializer(T::default)
}
