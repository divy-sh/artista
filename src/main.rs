use diffusion_rs::{
    api::gen_img,
    preset::{Preset, PresetBuilder},
};

fn main() {
    let (config, mut model_config) = PresetBuilder::default()
        .preset(Preset::SegmindVega)
        .prompt("a lovely duck drinking water from a bottle")
        .build()
        .unwrap();
    gen_img(&config, &mut model_config).unwrap();
}
