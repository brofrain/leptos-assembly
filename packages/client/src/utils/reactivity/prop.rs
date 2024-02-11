use common::vendor::leptos::{MaybeProp, TextProp};

pub type MaybeTextProp = MaybeProp<TextProp>;

pub trait MaybeTextPropExt {
    fn get_string(&self) -> Option<String>;
}

impl MaybeTextPropExt for MaybeTextProp {
    fn get_string(&self) -> Option<String> {
        self.with(|v| {
            let string: String = v.get().into();
            string
        })
    }
}
