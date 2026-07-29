use diffusion_rs::{api::gen_img, preset::{Preset, PresetBuilder}};
use dioxus::prelude::*;

use crate::components::{button::{Button, ButtonVariant}, popover::{PopoverContent, PopoverRoot, PopoverTrigger}};

#[component]
pub fn Hero() -> Element {
    let mut open = use_signal(|| false);
    let mut confirmed = use_signal(|| false);

    rsx! {
        div { display: "flex", flex_direction: "column", gap: "0.5rem",
            PopoverRoot { open: open(), on_open_change: move |v| open.set(v),
                PopoverTrigger { "Show Popover" }
                PopoverContent { gap: "0.25rem",
                    h3 {
                        padding_top: "0.25rem",
                        padding_bottom: "0.25rem",
                        width: "100%",
                        text_align: "center",
                        margin: 0,
                        "Delete Item?"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            open.set(false);
                            confirmed.set(true);
                        },
                        "Confirm"
                    }
                    Button {
                        r#type: "button",
                        "data-style": "outline",
                        onclick: move |_| {
                            open.set(false);
                        },
                        "Cancel"
                    }
                }
            }
            if confirmed() {
                p { style: "color: var(--contrast-error-color); margin-top: 16px; font-weight: 600;",
                    "Item deleted!"
                }
            }
        }
    }
}

fn generate() {
    let (config, mut model_config) = PresetBuilder::default()
        .preset(Preset::SegmindVega)
        .prompt("a lovely duck drinking water from a bottle")
        .build()
        .unwrap();
    gen_img(&config, &mut model_config).unwrap();
}
