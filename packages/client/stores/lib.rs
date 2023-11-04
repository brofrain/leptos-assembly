use client_utils::reactivity::use_global_context_with_initializer;
use common_macros::flatten_pub_mod;

pub trait Store: Copy + 'static {
    fn create() -> Self;
}

pub fn use_store<T>() -> T
where
    T: Store,
{
    use_global_context_with_initializer::<T>(T::create)
}

flatten_pub_mod!(names);
