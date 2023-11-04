use client_globals::prelude::*;
use leptos::Callable;

#[derive(Clone, Copy)]
pub struct ViewCallback(Callback<(), View>);

impl ViewCallback {
    pub fn new<V>(f: impl Fn() -> V + 'static) -> Self
    where
        V: IntoView,
    {
        Self(Callback::new(move |()| f().into_view()))
    }
}

impl Callable<(), View> for ViewCallback {
    fn call(&self, input: ()) -> View {
        (self.0)(input)
    }
}

impl FnOnce<()> for ViewCallback {
    type Output = View;

    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        Callable::call(&self, args)
    }
}

impl FnMut<()> for ViewCallback {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> Self::Output {
        Callable::call(&*self, args)
    }
}

impl Fn<()> for ViewCallback {
    extern "rust-call" fn call(&self, args: ()) -> Self::Output {
        Callable::call(self, args)
    }
}
