use std::collections::VecDeque;

use super::{id, use_global_context};
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
    body: View,
    severity: Severity,
}

#[derive(Clone, Copy)]
pub struct Queue(RwSignal<VecDeque<Toast>>);

impl Default for Queue {
    fn default() -> Self {
        Self(RwSignal::new(VecDeque::new()))
    }
}

pub struct Payload {
    pub body: View,
    pub severity: Severity,
}

impl From<Payload> for Toast {
    fn from(payload: Payload) -> Self {
        Self {
            id: id::usize(),
            body: payload.body,
            severity: payload.severity,
        }
    }
}

pub fn push(payload: Payload) {
    let queue = use_global_context::<Queue>().0;

    update!(|queue| {
        queue.push_back(payload.into());
    });

    spawn_local(async move {
        sleep(5000.milliseconds()).await;
        update!(|queue| {
            queue.pop_front();
        });
    });
}

pub fn use_queue() -> ReadSignal<VecDeque<Toast>> {
    use_global_context::<Queue>().0.read_only()
}

// TODO some kind of `push_toast` macro maybe?
// it could accept i18n syntax and severity like:
// `push_toast!(Severity::Info, i18n, name.changed, new_name = name)`
