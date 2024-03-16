use std::collections::VecDeque;

use client_utils::{
    future::sleep,
    reactivity::{use_global_context, ViewCallback},
};
use common::{prelude::*, utils::id};

#[derive(Default, Clone, Copy)]
pub enum Severity {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Getters, Clone)]
#[getset(get = "pub")]
pub struct Toast {
    id: id::Usize,
    body: ViewCallback,
    severity: Severity,
}

#[derive(Clone, Copy)]
pub struct Queue(RwSignal<VecDeque<Toast>>);

impl Default for Queue {
    fn default() -> Self {
        Self(RwSignal::new(VecDeque::new()))
    }
}

#[derive(Clone, Copy)]
pub struct UsePushReturn {
    queue: Queue,
}

impl UsePushReturn {
    fn push<V>(&self, severity: Severity, body: impl Fn() -> V + 'static)
    where
        V: IntoView,
    {
        self.queue.0.update(|queue| {
            queue.push_back(Toast {
                id: id::runtime_usize(),
                body: ViewCallback::new(body),
                severity,
            });
        });

        let queue = self.queue;
        spawn_local(async move {
            sleep(3.seconds()).await;
            queue.0.update(|queue| {
                queue.pop_front();
            });
        });
    }
}

// TODO
// Implementing same behavior for FnOnce, FnMut, Fn repeats also in
// packages/client/utils/reactivity/callback.rs. Maybe a nice macro would make
// the code a bit smaller?

impl<V, F> FnOnce<(Severity, F)> for UsePushReturn
where
    V: IntoView,
    F: Fn() -> V + 'static,
{
    type Output = ();

    extern "rust-call" fn call_once(self, args: (Severity, F)) -> Self::Output {
        self.push(args.0, args.1);
    }
}

impl<V, F> FnMut<(Severity, F)> for UsePushReturn
where
    V: IntoView,
    F: Fn() -> V + 'static,
{
    extern "rust-call" fn call_mut(&mut self, args: (Severity, F)) -> Self::Output {
        self.push(args.0, args.1);
    }
}

impl<V, F> Fn<(Severity, F)> for UsePushReturn
where
    V: IntoView,
    F: Fn() -> V + 'static,
{
    extern "rust-call" fn call(&self, args: (Severity, F)) -> Self::Output {
        self.push(args.0, args.1);
    }
}

pub fn use_push() -> UsePushReturn {
    let queue = use_global_context::<Queue>();
    UsePushReturn { queue }
}

pub fn use_queue() -> ReadSignal<VecDeque<Toast>> {
    use_global_context::<Queue>().0.read_only()
}
