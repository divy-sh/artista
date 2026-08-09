use diffusion_rs::{
    api::gen_img,
    preset::{Preset, PresetBuilder},
};
use dioxus::prelude::*;

use crate::{
    components::tabs::{TabContent, TabList, TabTrigger, Tabs},
    panels::{modellist::ModelList, newchat::NewChat},
};

#[component]
pub fn Run() -> Element {
    rsx! {
        // Main container taking full screen relative context
        div { class: "relative w-full h-full overflow-hidden",
            Tabs { default_value: "tab1".to_string(), horizontal: true,

                // Page contents filling the view space
                div { class: "w-full h-full pb-20", // Added bottom padding so content isn't hidden behind the bar
                    TabContent { index: 0usize, value: "tab1".to_string(), NewChat {} }
                    TabContent { index: 1usize, value: "tab2".to_string(),
                        div { class: "flex justify-center items-center h-full", ModelList {} }
                    }
                }

                // Persistent bottom center tab selector
                div { class: "fixed bottom-6 left-1/2 -translate-x-1/2 z-50",
                    TabList { class: "flex bg-background/80 backdrop-blur-md border border-border shadow-lg rounded-full p-1 gap-1",
                        TabTrigger {
                            value: "tab1".to_string(),
                            index: 0usize,
                            class: "px-4 py-2 rounded-full text-sm font-medium transition-all",
                            "Chat"
                        }
                        TabTrigger {
                            value: "tab2".to_string(),
                            index: 1usize,
                            class: "px-4 py-2 rounded-full text-sm font-medium transition-all",
                            "Models"
                        }
                    }
                }
            }
        }
    }
}

fn generate(prompt: String) {
    let (config, mut model_config) = PresetBuilder::default()
        .preset(Preset::SegmindVega)
        .prompt(prompt)
        .build()
        .unwrap();
    gen_img(&config, &mut model_config).unwrap();
}
