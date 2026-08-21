use dioxus::prelude::*;
use lucide_dioxus::{ArrowUp, ChevronsUpDown, Search};

use crate::core::models::model_list;
use crate::ui::components::combobox::{
    Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
};
use crate::ui::components::input_prompt::{
    InputPrompt, InputPromptFooter, InputPromptSubmit, InputPromptTextarea,
};
use crate::ui::components::popover::{Popover, PopoverContent, PopoverTrigger};

#[component]
pub fn InputPanel(
    input_value: Signal<String>,
    selected_model_key: Signal<Option<String>>,
    is_generating: bool,
    on_submit: EventHandler<()>,
) -> Element {
    let mut is_popover_open = use_signal(|| false);

    rsx! {
        InputPrompt {
            InputPromptTextarea {
                class: "w-full",
                value: input_value,
                placeholder: "Type a prompt to generate an image...",
                on_submit: move |_| on_submit.call(()),
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
                    disabled: input_value().trim().is_empty() || is_generating,
                    onclick: move |_| on_submit.call(()),
                    ArrowUp {},

                }
            }
        }
    }
}

#[component]
fn ModelListItems(selected_key: Option<String>, on_select: EventHandler<String>) -> Element {
    let models = model_list();
    rsx! {
        for (key, label) in models.into_iter() {
            CommandItem {
                key: "{key}",
                value: key.clone(),
                selected: selected_key.as_deref() == Some(&key),
                on_select: move |_| on_select.call(key.clone()),
                "{label}"
            }
        }
    }
}
