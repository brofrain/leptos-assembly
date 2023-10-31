use std::rc::Rc;

use leptos_use::{use_debounce_fn_with_arg_and_options, DebounceOptions};
use time::Duration;

use crate::app::prelude::*;

#[derive(Clone, Copy)]
pub struct UseSwitch {
    activation_meter: StoredValue<i32>,
    flush: Action<i32, ()>,
}

impl UseSwitch {
    fn update_activation_meter(&self, f: impl FnOnce(i32) -> i32) {
        let activation_meter = f(self.activation_meter.get_value());
        self.activation_meter.set_value(activation_meter);
        self.flush.dispatch(activation_meter);
    }

    pub fn enable(&self) {
        self.update_activation_meter(|v| v + 1);
    }

    pub fn disable(&self) {
        self.update_activation_meter(|v| v - 1);
    }
}

pub fn use_switch<F1, F2>(
    on_activation: F1,
    on_deactivation: F2,
    debounce: Duration,
) -> UseSwitch
where
    F1: Fn() + 'static,
    F2: Fn() + 'static,
{
    let ms = debounce.whole_milliseconds() as f64;

    let activation_meter = StoredValue::new(0);
    let active = StoredValue::new(false);

    let on_activation = Rc::new(on_activation);
    let on_deactivation = Rc::new(on_deactivation);

    let flush = use_debounce_fn_with_arg_and_options(
        move |activation_meter: i32| {
            if active() {
                if activation_meter <= 0 {
                    on_deactivation();
                    active.set_value(false);
                }
            } else if activation_meter > 0 {
                on_activation();
                active.set_value(true);
            }
        },
        ms,
        DebounceOptions::default().max_wait(Some(ms * 3.0)),
    );

    let flush = create_action(move |v| {
        flush(*v);
        async {}
    });

    UseSwitch {
        activation_meter,
        flush,
    }
}
