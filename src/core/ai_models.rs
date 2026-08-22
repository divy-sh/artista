use diffusion_rs::preset::{
    Anima2Weight, AnimaWeight, ChromaRadianceWeight, DiffInstructStarWeight, ErnieImageWeight,
    Flux1MiniWeight, NitroSDRealismWeight, NitroSDVibrantWeight, Preset, QwenImageWeight,
    SDXS512DreamShaperWeight, SSD1BWeight,
};

use std::collections::HashMap;

pub fn model_list() -> HashMap<String, String> {
    let mut models = HashMap::new();

    // List all variants matching your Preset definition with default weights
    let all_presets = vec![
        Preset::StableDiffusion1_4,
        Preset::StableDiffusion1_5,
        Preset::StableDiffusion2_1,
        Preset::SDXLBase1_0,
        Preset::SDTurbo,
        Preset::SDXLTurbo1_0,
        Preset::Flux1Mini(Flux1MiniWeight::default()),
        Preset::NitroSDRealism(NitroSDRealismWeight::default()),
        Preset::NitroSDVibrant(NitroSDVibrantWeight::default()),
        Preset::DiffInstructStar(DiffInstructStarWeight::default()),
        Preset::ChromaRadiance(ChromaRadianceWeight::default()),
        Preset::SSD1B(SSD1BWeight::default()),
        Preset::QwenImage(QwenImageWeight::default()),
        Preset::DreamShaperXL2_1Turbo,
        Preset::SDXS512DreamShaper(SDXS512DreamShaperWeight::default()),
        Preset::SegmindVega,
        Preset::Anima(AnimaWeight::default()),
        Preset::Anima2(Anima2Weight::default()),
        Preset::ErnieImage(ErnieImageWeight::default()),
        Preset::ErnieImageTurbo(ErnieImageWeight::default()),
        Preset::HiDreamO1ImageDev,
        Preset::HiDreamO1Image,
    ];

    for model in all_presets {
        let name = format!("{:?}", model);
        models.insert(name.clone(), name);
    }

    models
}
