use std::collections::VecDeque;

use super::{id, use_global_context, ViewCallback};
use crate::{app::prelude::*, utils::future::sleep};

#[derive(Default, Clone, Copy)]
pub enum Severity {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Getters, Clone)]
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

pub fn push<V>(severity: Severity, body: impl Fn() -> V + 'static)
where
    V: IntoView,
{
    let queue = use_global_context::<Queue>().0;

    update!(|queue| {
        queue.push_back(Toast {
            id: id::usize(),
            body: ViewCallback::new(body),
            severity,
        });
    });

    spawn_local(async move {
        sleep(3.seconds()).await;
        update!(|queue| {
            queue.pop_front();
        });
    });
}

pub fn use_queue() -> ReadSignal<VecDeque<Toast>> {
    use_global_context::<Queue>().0.read_only()
}