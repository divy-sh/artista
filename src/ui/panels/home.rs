use dioxus::prelude::*;
use std::sync::mpsc;

use crate::core::controller::{base64_encode, generate, model_list};
use crate::core::models::image::ImageData;

use crate::ui::panels::input::InputPanel;
use crate::ui::panels::output::OutputPanel;

#[component]
pub fn Home() -> Element {
    let mut input_value = use_signal(String::new);
    let mut progress_status = use_signal_sync(String::new);
    let mut is_generating = use_signal_sync(|| false);
    let mut generated_image = use_signal_sync(|| None::<String>);

    let models = model_list();
    let default_model_key = models
        .keys()
        .find(|k| k.starts_with("SDTurbo"))
        .cloned()
        .or_else(|| models.keys().next().cloned());

    let selected_model_key = use_signal(move || default_model_key);

    let handle_submit = move |_| {
        let text = input_value.peek().trim().to_string();
        let model_key = selected_model_key.peek().clone().unwrap_or_default();

        if text.is_empty() || *is_generating.peek() {
            return;
        }

        input_value.set(String::new());
        is_generating.set(true);
        generated_image.set(None);
        progress_status.set("Starting image generation...".into());

        let (progress_tx, progress_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel::<Result<Vec<u8>, String>>();

        generate(
            text,
            None::<ImageData>,
            model_key,
            progress_tx,
            &progress_rx,
            result_tx,
            &result_rx,
        );

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
            div { class: "w-full max-w-lg space-y-4 flex flex-col items-center",
                if is_generating() || generated_image().is_some() {
                    OutputPanel {
                        progress_status: progress_status,
                        is_generating: is_generating,
                        generated_image: generated_image,
                    }
                } else {
                    InputPanel {
                        input_value: input_value,
                        selected_model_key: selected_model_key,
                        is_generating: is_generating(),
                        on_submit: handle_submit,
                    }
                }
            }
    }
}
