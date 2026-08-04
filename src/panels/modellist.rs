use std::collections::HashMap;
use diffusion_rs::preset::{
    Preset, Flux1Weight, Flux1MiniWeight, ChromaWeight, NitroSDRealismWeight,
    NitroSDVibrantWeight, DiffInstructStarWeight, ChromaRadianceWeight, SSD1BWeight,
    Flux2Weight, ZImageTurboWeight, QwenImageWeight, OvisImageWeight,
    TwinFlowZImageTurboExpWeight, SDXS512DreamShaperWeight, Flux2Klein4BWeight,
    Flux2KleinBase4BWeight, Flux2Klein9BWeight, Flux2KleinBase9BWeight,
    AnimaWeight, Anima2Weight, ErnieImageWeight, LongCatImageWeight,
};
use dioxus::prelude::*;

use crate::components::{label::Label, modelcard::modelcard::ModelCard};

fn model_list() -> HashMap<String, String> {
    let mut models = HashMap::new();
    
    // List all variants matching your Preset definition with default weights
    let all_presets = vec![
        Preset::StableDiffusion1_4,
        Preset::StableDiffusion1_5,
        Preset::StableDiffusion2_1,
        Preset::StableDiffusion3Medium,
        Preset::StableDiffusion3_5Medium,
        Preset::StableDiffusion3_5Large,
        Preset::StableDiffusion3_5LargeTurbo,
        Preset::SDXLBase1_0,
        Preset::SDTurbo,
        Preset::SDXLTurbo1_0,
        Preset::Flux1Dev(Flux1Weight::default()),
        Preset::Flux1Schnell(Flux1Weight::default()),
        Preset::Flux1Mini(Flux1MiniWeight::default()),
        Preset::JuggernautXL11,
        Preset::Chroma(ChromaWeight::default()),
        Preset::NitroSDRealism(NitroSDRealismWeight::default()),
        Preset::NitroSDVibrant(NitroSDVibrantWeight::default()),
        Preset::DiffInstructStar(DiffInstructStarWeight::default()),
        Preset::ChromaRadiance(ChromaRadianceWeight::default()),
        Preset::SSD1B(SSD1BWeight::default()),
        Preset::Flux2Dev(Flux2Weight::default()),
        Preset::ZImageTurbo(ZImageTurboWeight::default()),
        Preset::QwenImage(QwenImageWeight::default()),
        Preset::OvisImage(OvisImageWeight::default()),
        Preset::DreamShaperXL2_1Turbo,
        Preset::TwinFlowZImageTurboExp(TwinFlowZImageTurboExpWeight::default()),
        Preset::SDXS512DreamShaper(SDXS512DreamShaperWeight::default()),
        Preset::Flux2Klein4B(Flux2Klein4BWeight::default()),
        Preset::Flux2KleinBase4B(Flux2KleinBase4BWeight::default()),
        Preset::Flux2Klein9B(Flux2Klein9BWeight::default()),
        Preset::Flux2KleinBase9B(Flux2KleinBase9BWeight::default()),
        Preset::SegmindVega,
        Preset::Anima(AnimaWeight::default()),
        Preset::Anima2(Anima2Weight::default()),
        Preset::ErnieImage(ErnieImageWeight::default()),
        Preset::ErnieImageTurbo(ErnieImageWeight::default()),
        Preset::HiDreamO1ImageDev,
        Preset::HiDreamO1Image,
        Preset::LongCatImage(LongCatImageWeight::default()),
        Preset::Lens,
        Preset::LensTurbo,
    ];

    for model in all_presets {
        let name = format!("{:?}", model);
        models.insert(name.clone(), name);
    }
    
    models
}

#[component]
pub fn ModelList() -> Element {
    let models = model_list();
    let model_items = models.iter().map(|(key, value)| {
        rsx! {
            ModelCard { model_name: key.clone() }
        }
    });

    rsx! {
        div { style: "flex-direction: row; gap: 0.5rem;",
            Label { html_for: "model list", "Available Models" }
            {model_items}
        }
    }
}