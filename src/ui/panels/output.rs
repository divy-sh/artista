use crate::ui::components::button::Button;
use crate::ui::components::skeleton::Skeleton;
use dioxus::prelude::*;

#[component]
pub fn OutputPanel(
    progress_status: Signal<String, SyncStorage>,
    is_generating: Signal<bool, SyncStorage>,
    generated_image: Signal<Option<String>, SyncStorage>,
) -> Element {
    rsx! {
        if is_generating() {
            div { class: "w-full flex flex-col items-center space-y-2",
                Skeleton { class: "aspect-square w-full rounded-2xl" }
                p { class: "text-xs text-muted-foreground animate-pulse text-center",
                    "{progress_status()}"
                }
            }
        } else if let Some(image_url) = generated_image() {
            div { class: "w-full overflow-hidden rounded-2xl border shadow-sm",
                img {
                    src: "{image_url}",
                    alt: "Generated output",
                    class: "w-full h-auto aspect-square object-cover"
                }
            }
        }
        Button {
            onclick: move |_| generated_image.set(None),
            "Home"
        }
    }
}
