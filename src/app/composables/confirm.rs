use futures::channel::oneshot;

use super::{id, use_global_context};
use crate::app::prelude::*;

#[derive(Getters, Clone, PartialEq)]
pub struct Confirm {
    id: id::Usize,
    body: View,
    accept: View,

    /// `None` means that the confirm is not cancelable
    cancel: Option<View>,
}

#[derive(Debug)]
pub enum ResolutionStatus {
    Cancelled,
    Accepted,
}

impl ResolutionStatus {
    pub fn is_accepted(&self) -> bool {
        matches!(self, ResolutionStatus::Accepted)
    }
}

#[derive(Clone, Copy)]
pub struct Queue(
    RwSignal<Vec<(StoredValue<Confirm>, oneshot::Sender<ResolutionStatus>)>>,
);

impl Default for Queue {
    fn default() -> Self {
        Self(RwSignal::new(Vec::new()))
    }
}

impl Queue {
    fn push(&self, confirm: Confirm) -> oneshot::Receiver<ResolutionStatus> {
        let (tx, rx) = oneshot::channel();
        self.0.update(|queue| {
            queue.push((StoredValue::new(confirm), tx));
        });
        rx
    }

    pub fn resolve_by_id(&self, id: id::Usize, status: ResolutionStatus) {
        if let Some(i) = self.0.with(|v| {
            v.iter()
                .position(|(item, ..)| item.with_value(|v| v.id) == id)
        }) {
            self.0.update(|v| {
                let (_, tx) = v.remove(i);
                tx.send(status).unwrap();
            });
        }
    }

    pub fn get(&self) -> Vec<StoredValue<Confirm>> {
        self.0.with(|v| {
            v.iter()
                .map(|(confirm, ..)| confirm)
                .copied()
                .collect::<Vec<_>>()
        })
    }
}

pub mod payload {
    use super::Confirm;
    use crate::app::{composables::id, prelude::*};

    pub struct Cancelable {
        pub body: View,
        pub accept: View,
        pub cancel: View,
    }

    impl Default for Cancelable {
        fn default() -> Self {
            let i18n = use_i18n();
            Self {
                body: t_view_untracked!(i18n, common.confirm.body),
                accept: t_view_untracked!(i18n, common.confirm.accept),
                cancel: t_view_untracked!(i18n, common.confirm.cancel),
            }
        }
    }

    impl From<Cancelable> for Confirm {
        fn from(confirm: Cancelable) -> Self {
            Self {
                id: id::usize(),
                body: confirm.body,
                accept: confirm.accept,
                cancel: Some(confirm.cancel),
            }
        }
    }

    pub struct Noncancelable {
        pub body: View,
        pub accept: View,
    }

    impl Default for Noncancelable {
        fn default() -> Self {
            let i18n = use_i18n();
            Self {
                body: t_view_untracked!(i18n, common.confirm.body),
                accept: t_view_untracked!(i18n, common.confirm.accept),
            }
        }
    }

    impl From<Noncancelable> for Confirm {
        fn from(confirm: Noncancelable) -> Self {
            Self {
                id: id::usize(),
                body: confirm.body,
                accept: confirm.accept,
                cancel: None,
            }
        }
    }
}

pub async fn show<T>(payload: T) -> ResolutionStatus
where
    T: Into<Confirm>,
{
    let rx = use_global_context::<Queue>().push(payload.into());
    rx.await.unwrap()
}

pub fn use_queue() -> Queue {
    use_global_context::<Queue>()
}
