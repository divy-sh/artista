use crate::core::models::model_list;
use dioxus::prelude::*;

#[component]
pub fn ModelList() -> Element {
    let models = model_list();
    let model_items = models.iter().map(|(key, _value)| {
        rsx! {}
    });

    rsx! {
        div { style: "flex-direction: row; gap: 0.5rem;",
            label { "Available Models" }
            {model_items}
        }
    }
}
