use dioxus::prelude::*;
use lucide_dioxus::{ArrowUp, ChevronsUpDown, Search};
use std::sync::mpsc;

use crate::core::encoder::base64_encode;
use crate::core::generator::generate;
use crate::core::image::ImageData;
use crate::core::models::model_list;

use crate::ui::components::combobox::{
    Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
};
use crate::ui::components::input_prompt::{
    InputPrompt, InputPromptFooter, InputPromptSubmit, InputPromptTextarea,
};
use crate::ui::components::popover::{Popover, PopoverContent, PopoverTrigger};
use crate::ui::components::skeleton::Skeleton;

#[component]
pub fn DemoInputPrompt() -> Element {
    let mut input_value = use_signal(String::new);
    let mut is_popover_open = use_signal(|| false);

    // Generation state signals
    let mut progress_status = use_signal_sync(String::new);
    let mut is_generating = use_signal_sync(|| false);
    let mut generated_image = use_signal_sync(|| None::<String>);

    // Fetch available models and set default key
    let models = model_list();
    let default_model_key = models
        .keys()
        .find(|k| k.starts_with("SDTurbo"))
        .cloned()
        .or_else(|| models.keys().next().cloned());

    let mut selected_model_key = use_signal(move || default_model_key);

    let on_submit = move |_| {
        let text = input_value.peek().trim().to_string();
        let model_key = selected_model_key.peek().clone().unwrap_or_default();

        if text.is_empty() {
            return;
        }

        input_value.set(String::new());

        // Initialize generation state
        is_generating.set(true);
        generated_image.set(None);
        progress_status.set("Starting image generation...".into());

        // Setup channels
        let (progress_tx, progress_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel::<Result<Vec<u8>, String>>();

        // Start generator backend
        generate(
            text,
            None::<ImageData>,
            model_key,
            progress_tx,
            &progress_rx,
            result_tx,
            &result_rx,
        );

        // Background thread to stream progress and results
        std::thread::spawn(move || {
            while let Ok(progress) = progress_rx.recv() {
                progress_status.set(format!("Generating... {:?}", progress));
            }

            if let Ok(res) = result_rx.recv() {
                match res {
                    Ok(bytes) => {
                        let data_url = format!("data:image/png;base64,{}", base64_encode(&bytes));
                        generated_image.set(Some(data_url));
                        progress_status.set("Generation complete!".into());
                    }
                    Err(err) => {
                        progress_status.set(format!("Generation failed: {}", err));
                    }
                }
            }

            is_generating.set(false);
        });
    };

    rsx! {
        // Centered layout container occupying full screen height
        div { class: "min-h-screen w-full flex flex-col justify-center items-center p-4",
            div { class: "w-full max-w-lg space-y-4 flex flex-col items-center",

                // 1. Display skeleton while generating
                if is_generating() {
                    div { class: "w-full flex flex-col items-center",
                        Skeleton { class: "aspect-square w-full rounded-2xl" }
                        p { class: "text-xs text-muted-foreground animate-pulse text-center",
                            "{progress_status()}"
                        }
                    }
                }
                // 2. Display generated image once generation finishes
                else if let Some(image_url) = generated_image() {
                    div { class: "w-full overflow-hidden rounded-2xl border shadow-sm",
                        img {
                            src: "{image_url}",
                            alt: "Generated output",
                            class: "w-full h-auto aspect-square object-cover"
                        }
                    }
                }

                InputPrompt {
                    InputPromptTextarea {
                        class: "w-full",
                        value: input_value,
                        placeholder: "Type a prompt to generate an image...",
                        on_submit
                    }
                    InputPromptFooter {
                        span { class: "px-1 text-xs text-muted-foreground", "Shift+Enter for new line" }

                        Popover {
                            PopoverTrigger { class: "justify-between w-[200px]",
                                span { class: "truncate",
                                    {
                                        selected_model_key()
                                            .unwrap_or_else(|| "Select model...".into())
                                    }
                                }
                                ChevronsUpDown { class: "ml-auto opacity-50 size-4" }
                            }

                            PopoverContent { class: "p-0 w-[200px]",
                                Command {
                                    div { class: "flex gap-2 items-center px-2 border-b",
                                        Search { class: "size-4 text-muted-foreground shrink-0" }
                                        CommandInput {}
                                    }
                                    CommandList { class: "min-h-0",
                                        CommandEmpty { "No model found." }
                                        CommandGroup {
                                            ModelListItems {
                                                selected_key: selected_model_key(),
                                                on_select: move |key: String| {
                                                    selected_model_key.set(Some(key));
                                                    is_popover_open.set(false);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        InputPromptSubmit {
                            disabled: input_value().trim().is_empty() || is_generating(),
                            ArrowUp {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ModelListItems(selected_key: Option<String>, on_select: EventHandler<String>) -> Element {
    let models = model_list();
    rsx! {
        for (key, label) in models.iter() {
            CommandItem {
                key: "{key}",
                value: key.clone(),
                selected: selected_key.as_deref() == Some(key),
                on_select: {
                    let key = key.clone();
                    move |_| on_select.call(key.clone())
                },
                "{label}"
            }
        }
    }
}
