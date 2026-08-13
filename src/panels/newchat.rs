use dioxus::prelude::*;
use dioxus_primitives::select::SelectGroup;
use std::path::PathBuf;
use std::sync::mpsc;

use diffusion_rs::{
    api::gen_img_with_progress,
    preset::{
        Anima2Weight, AnimaWeight, ChromaRadianceWeight, DiffInstructStarWeight,
        ErnieImageWeight, Flux1MiniWeight, NitroSDRealismWeight, NitroSDVibrantWeight, Preset,
        PresetBuilder, QwenImageWeight, SDXS512DreamShaperWeight, SSD1BWeight,
    },
};

use crate::components::button::Button;
use crate::components::input::Input;
use crate::components::select::{Select, SelectGroupLabel, SelectOption, };
use crate::components::textarea::Textarea;
use crate::panels::modellist::model_list;

#[derive(Clone, Debug, PartialEq)]
pub struct ImageData {
    pub name: String,
    pub bytes: Vec<u8>,
}

fn base64_encode(data: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARSET[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARSET[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARSET[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARSET[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

/// Converts a Debug-formatted model name string back to a `Preset`
fn preset_from_name(name: &str) -> Preset {
    match name {
        s if s.starts_with("StableDiffusion1_4") => Preset::StableDiffusion1_4,
        s if s.starts_with("StableDiffusion1_5") => Preset::StableDiffusion1_5,
        s if s.starts_with("StableDiffusion2_1") => Preset::StableDiffusion2_1,
        s if s.starts_with("SDXLBase1_0") => Preset::SDXLBase1_0,
        s if s.starts_with("SDTurbo") => Preset::SDTurbo,
        s if s.starts_with("SDXLTurbo1_0") => Preset::SDXLTurbo1_0,
        s if s.starts_with("Flux1Mini") => Preset::Flux1Mini(Flux1MiniWeight::default()),
        s if s.starts_with("NitroSDRealism") => Preset::NitroSDRealism(NitroSDRealismWeight::default()),
        s if s.starts_with("NitroSDVibrant") => Preset::NitroSDVibrant(NitroSDVibrantWeight::default()),
        s if s.starts_with("DiffInstructStar") => Preset::DiffInstructStar(DiffInstructStarWeight::default()),
        s if s.starts_with("ChromaRadiance") => Preset::ChromaRadiance(ChromaRadianceWeight::default()),
        s if s.starts_with("SSD1B") => Preset::SSD1B(SSD1BWeight::default()),
        s if s.starts_with("QwenImage") => Preset::QwenImage(QwenImageWeight::default()),
        s if s.starts_with("DreamShaperXL2_1Turbo") => Preset::DreamShaperXL2_1Turbo,
        s if s.starts_with("SDXS512DreamShaper") => Preset::SDXS512DreamShaper(SDXS512DreamShaperWeight::default()),
        s if s.starts_with("SegmindVega") => Preset::SegmindVega,
        s if s.starts_with("Anima2") => Preset::Anima2(Anima2Weight::default()),
        s if s.starts_with("Anima") => Preset::Anima(AnimaWeight::default()),
        s if s.starts_with("ErnieImageTurbo") => Preset::ErnieImageTurbo(ErnieImageWeight::default()),
        s if s.starts_with("ErnieImage") => Preset::ErnieImage(ErnieImageWeight::default()),
        s if s.starts_with("HiDreamO1ImageDev") => Preset::HiDreamO1ImageDev,
        s if s.starts_with("HiDreamO1Image") => Preset::HiDreamO1Image,
        _ => Preset::StableDiffusion1_5,
    }
}

#[component]
pub fn NewChat(on_submit: Option<EventHandler<(String, Option<ImageData>)>>) -> Element {
    let mut text_input = use_signal(String::new);
    let mut attached_image = use_signal(|| None::<ImageData>);

    // Load available models dynamically from model_list()
    let models = model_list();
    let default_model_key = models
        .keys()
        .find(|k| k.starts_with("StableDiffusion1_5"))
        .cloned()
        .unwrap_or_else(|| models.keys().next().cloned().unwrap_or_default());

    let mut selected_model_key = use_signal(move || Some(default_model_key));

    let mut progress_status = use_signal_sync(String::new);
    let mut is_generating = use_signal_sync(|| false);
    let mut generated_image = use_signal_sync(|| None::<String>);

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
        let key_str = selected_model_key.read().clone().unwrap_or_default();
        let preset = preset_from_name(&key_str);

        if text.trim().is_empty() && image.is_none() {
            return;
        }

        if let Some(callback) = &on_submit {
            callback.call((text.clone(), image));
        }

        text_input.set(String::new());
        attached_image.set(None);

        is_generating.set(true);
        generated_image.set(None);
        progress_status.set("Starting image generation...".into());

        let (progress_tx, progress_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel::<Result<Vec<u8>, String>>();

        // Thread 1: Generation
        std::thread::spawn(move || {
            let (config, mut model_config) = PresetBuilder::default()
                .preset(preset)
                .prompt(text)
                .build()
                .unwrap();

            match gen_img_with_progress(&config, &mut model_config, progress_tx) {
                Ok(_) => {
                    let candidate_paths = [
                        PathBuf::from("output.png"),
                        PathBuf::from("output_0.png"),
                        PathBuf::from("output_00001_.png"),
                    ];

                    let mut loaded_bytes = None;
                    for path in &candidate_paths {
                        if let Ok(bytes) = std::fs::read(path) {
                            loaded_bytes = Some(bytes);
                            break;
                        }
                    }

                    if let Some(bytes) = loaded_bytes {
                        let _ = result_tx.send(Ok(bytes));
                    } else {
                        let _ = result_tx.send(Err("Generated image output file not found.".into()));
                    }
                }
                Err(err) => {
                    let _ = result_tx.send(Err(format!("{:?}", err)));
                }
            }
        });

        // Thread 2: Progress & Output Listener
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

    // Build option elements using the custom Select components
    let model_options = models.iter().enumerate().map(|(index, (key, label))| {
        let key_str = key.clone();
        rsx! {
            SelectOption::<String> {
                index,
                value: key_str.clone(),
                text_value: "{label}",
                disabled: *is_generating.read(),
                "{label}"
            }
        }
    });

    rsx! {
        div {
            class: "flex justify-center items-center h-full",
            flex_direction: "column",
            text_align: "center",
            h1 { "New Chat" }

            // Custom Select Dropdown
            div { class: "mb-4 w-full max-w-xs relative",
                Select::<String> {
                    width: "100%",
                    value: Some(ReadSignal::from(selected_model_key)),
                    on_value_change: move |new_val: Option<String>| {
                        selected_model_key.set(new_val);
                    },
                    SelectGroup {
                        SelectGroupLabel { "Diffusion Models" }
                        {model_options}
                    }
                }
            }

            div {
                class: "flex justify-center items-center w-full",
                flex_direction: "row",
                gap: ".5rem",

                Input {
                    id: "image-upload",
                    r#type: "file",
                    accept: "image/*",
                    class: "hidden",
                    onchange: handle_image_upload,
                }

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

                Button { onclick: handle_submit, disabled: "{is_generating}",
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

            if !progress_status.read().is_empty() {
                p { class: "mt-4 text-sm text-gray-400 animate-pulse", "{progress_status}" }
            }

            if let Some(img_url) = generated_image.read().as_ref() {
                div { class: "mt-6 max-w-md rounded-lg overflow-hidden border border-input shadow-md",
                    img {
                        src: "{img_url}",
                        alt: "Generated Output",
                        class: "w-full h-auto object-cover",
                    }
                }
            }
        }
    }
}