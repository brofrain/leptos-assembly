use std::pin::Pin;

use client_i18n::use_i18n;
use client_utils::reactivity::{use_global_context, ViewCallback};
use exports::client::prelude::*;
use futures::{channel::oneshot, Future, FutureExt};
use utils::id;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct Confirm {
    id: id::Usize,
    body: ViewCallback,
    accept: ViewCallback,

    /// `None` means that the confirm is not cancelable
    cancel: Option<ViewCallback>,
}

#[derive(Debug)]
pub enum ResolutionStatus {
    Cancelled,
    Accepted,
}

impl ResolutionStatus {
    pub const fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
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

#[must_use]
#[derive(Default)]
pub struct Options {
    custom_body: Option<ViewCallback>,
    custom_accept: Option<ViewCallback>,
    custom_cancel: Option<ViewCallback>,
    disable_cancel: bool,
}

macro_rules! options_setters {
    ($($property:ident),*) => {
        paste::item! {
            $(
                pub fn [< set_ $property >] <V>(
                    mut self, f: impl Fn() -> V + 'static
                ) -> Self
                where
                    V: IntoView,
                {
                    self.[< custom_ $property >] = Some(ViewCallback::new(f));
                    self
                }
            )*
        }
    };
}

impl Options {
    options_setters!(body, accept, cancel);

    pub const fn disable_cancel(mut self) -> Self {
        self.disable_cancel = true;
        self
    }
}

impl From<Options> for Confirm {
    fn from(options: Options) -> Self {
        let i18n = use_i18n();
        Self {
            id: id::usize(),
            body: options.custom_body.unwrap_or_else(|| {
                ViewCallback::new(t!(i18n, common.confirm.body))
            }),
            accept: options.custom_accept.unwrap_or_else(|| {
                ViewCallback::new(t!(i18n, common.confirm.accept))
            }),
            cancel: if options.disable_cancel {
                None
            } else {
                Some(options.custom_cancel.unwrap_or_else(|| {
                    ViewCallback::new(t!(i18n, common.confirm.cancel))
                }))
            },
        }
    }
}

type ResolutionFuture = Pin<Box<dyn Future<Output = ResolutionStatus>>>;

#[derive(Clone, Copy)]
pub struct UseShowReturn {
    queue: Queue,
}

impl UseShowReturn {
    fn show(&self, options: Options) -> ResolutionFuture {
        let rx = self.queue.push(options.into());
        Box::pin(rx.map(Result::unwrap))
    }
}

impl FnOnce<(Options,)> for UseShowReturn {
    type Output = ResolutionFuture;

    extern "rust-call" fn call_once(self, args: (Options,)) -> Self::Output {
        self.show(args.0)
    }
}

impl FnMut<(Options,)> for UseShowReturn {
    extern "rust-call" fn call_mut(
        &mut self,
        args: (Options,),
    ) -> Self::Output {
        self.show(args.0)
    }
}

impl Fn<(Options,)> for UseShowReturn {
    extern "rust-call" fn call(&self, args: (Options,)) -> Self::Output {
        self.show(args.0)
    }
}

pub fn use_show() -> UseShowReturn {
    let queue = use_global_context::<Queue>();
    UseShowReturn { queue }
}

pub fn use_queue() -> Queue {
    use_global_context::<Queue>()
}
