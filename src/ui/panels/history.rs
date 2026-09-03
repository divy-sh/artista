use dioxus::prelude::*;

use crate::core::models::image::ImageData;
use crate::ui::composables::generation::get_history;

#[component]
pub fn History() -> Element {
    let mut conversations = use_signal(Vec::<ImageData>::new);
    conversations.set(get_history().unwrap());

    rsx! {
        div { class: "p-6 max-w-7xl mx-auto",
            h2 { class: "text-2xl font-bold mb-6 text-gray-800", "Image History" }

            if conversations.read().is_empty() {
                div { class: "p-8 text-center bg-gray-50 rounded-lg text-gray-500 border border-dashed border-gray-300",
                    "No image conversations found."
                }
            } else {
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6",
                    for image in conversations.read().iter() {
                        div {
                            key: "{image.id}",
                            class: "bg-white rounded-xl shadow-md overflow-hidden border border-gray-100 flex flex-col hover:shadow-lg transition-shadow",

                            // Image Container
                            div { class: "relative h-48 bg-gray-100 flex items-center justify-center overflow-hidden",
                                img {
                                    src: "{image.to_data_uri()}",
                                    alt: "{image.name}",
                                    class: "w-full h-full object-cover"
                                }
                            }

                            // Meta Information
                            div { class: "p-4 flex flex-col justify-between flex-grow bg-white",
                                h3 { class: "text-sm font-semibold text-gray-800 truncate mb-1",
                                    "{image.name}"
                                }
                                div { class: "flex items-center justify-between mt-2 text-xs text-gray-500",
                                    span { "Updated:" }
                                    span { class: "font-mono", "{image.last_updated}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
