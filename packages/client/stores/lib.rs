common_exports::use_macros!();

use client_utils::reactivity::use_global_context_with_initializer;

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
