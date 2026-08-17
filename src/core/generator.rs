use crate::core::image::ImageData;
use diffusion_rs::{
    api::Progress,
    preset::{
        Anima2Weight, AnimaWeight, ChromaRadianceWeight, DiffInstructStarWeight, ErnieImageWeight,
        Flux1MiniWeight, NitroSDRealismWeight, NitroSDVibrantWeight, Preset, QwenImageWeight,
        SDXS512DreamShaperWeight, SSD1BWeight,
    },
};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use diffusion_rs::{api::gen_img_with_progress, preset::PresetBuilder};

pub fn generate(
    text: String,
    _image: Option<ImageData>,
    key_str: String,
    progress_tx: Sender<Progress>,
    _progress_rx: &Receiver<Progress>,
    result_tx: Sender<std::result::Result<Vec<u8>, std::string::String>>,
    _result_rx: &Receiver<std::result::Result<Vec<u8>, std::string::String>>,
) {
    let preset = preset_from_name(key_str.as_str());
    let prompt_text = text.to_string(); // Convert to owned String for 'static thread bound

    // Thread 1: Generation
    std::thread::spawn(move || {
        let (config, mut model_config) = PresetBuilder::default()
            .preset(preset)
            .prompt(&prompt_text)
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
        s if s.starts_with("NitroSDRealism") => {
            Preset::NitroSDRealism(NitroSDRealismWeight::default())
        }
        s if s.starts_with("NitroSDVibrant") => {
            Preset::NitroSDVibrant(NitroSDVibrantWeight::default())
        }
        s if s.starts_with("DiffInstructStar") => {
            Preset::DiffInstructStar(DiffInstructStarWeight::default())
        }
        s if s.starts_with("ChromaRadiance") => {
            Preset::ChromaRadiance(ChromaRadianceWeight::default())
        }
        s if s.starts_with("SSD1B") => Preset::SSD1B(SSD1BWeight::default()),
        s if s.starts_with("QwenImage") => Preset::QwenImage(QwenImageWeight::default()),
        s if s.starts_with("DreamShaperXL2_1Turbo") => Preset::DreamShaperXL2_1Turbo,
        s if s.starts_with("SDXS512DreamShaper") => {
            Preset::SDXS512DreamShaper(SDXS512DreamShaperWeight::default())
        }
        s if s.starts_with("SegmindVega") => Preset::SegmindVega,
        s if s.starts_with("Anima2") => Preset::Anima2(Anima2Weight::default()),
        s if s.starts_with("Anima") => Preset::Anima(AnimaWeight::default()),
        s if s.starts_with("ErnieImageTurbo") => {
            Preset::ErnieImageTurbo(ErnieImageWeight::default())
        }
        s if s.starts_with("ErnieImage") => Preset::ErnieImage(ErnieImageWeight::default()),
        s if s.starts_with("HiDreamO1ImageDev") => Preset::HiDreamO1ImageDev,
        s if s.starts_with("HiDreamO1Image") => Preset::HiDreamO1Image,
        _ => Preset::StableDiffusion1_5,
    }
}
