use dioxus::prelude::*;

use crate::components::button::Button;
use crate::components::textarea::Textarea;
use crate::components::input::Input;
use crate::components::card::{Card, CardContent};

#[derive(Clone, Debug, PartialEq)]
pub struct ImageData {
    pub name: String,
    pub bytes: Vec<u8>,
}

#[component]
pub fn NewChat(
    on_submit: Option<EventHandler<(String, Option<ImageData>)>>,
) -> Element {
    let mut text_input = use_signal(String::new);
    let mut attached_image = use_signal(|| None::<ImageData>);

    let handle_image_upload = move |evt: FormEvent| async move {
        let files = evt.files();
        if let Some(file) = files.first() {
            let file_name = file.name();
            if let Ok(bytes) = file.read_bytes().await {
                attached_image.set(Some(ImageData {
                    name: file_name,
                    bytes: bytes.to_vec(),
                }));
            }
        }
    };

    let handle_submit = move |_| {
        let text = text_input.read().clone();
        let image = attached_image.read().clone();

        if !text.trim().is_empty() || image.is_some() {
            if let Some(callback) = &on_submit {
                callback.call((text, image));
            }

            text_input.set(String::new());
            attached_image.set(None);
        }
    };

    rsx! {
        div {
            class: "flex justify-center items-center h-full",
            flex_direction: "column",
            text_align: "center",
            h1 { "New Chat" }

            div {
                class: "flex justify-center items-center w-full",
                flex_direction: "row",
                gap: ".5rem",

                // Hidden file input
                Input {
                    id: "image-upload",
                    r#type: "file",
                    accept: "image/*",
                    class: "hidden",
                    onchange: handle_image_upload,
                }

                // Native HTML label targeting the hidden input ID
                label {
                    r#for: "image-upload",
                    class: "cursor-pointer inline-flex items-center justify-center p-2 rounded-md border border-input bg-background hover:bg-accent hover:text-accent-foreground",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "17 8 12 3 7 8" }
                        line {
                            x1: "12",
                            y1: "3",
                            x2: "12",
                            y2: "15",
                        }
                    }
                }

                Textarea {
                    placeholder: "Type a prompt or message...",
                    value: "{text_input}",
                    oninput: move |evt: FormEvent| text_input.set(evt.value()),
                }

                Button { onclick: handle_submit,
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "m22 2-7 20-4-9-9-4Z" }
                        path { d: "M22 2 11 13" }
                    }
                }
            }
        }
    }
}