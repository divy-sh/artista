use dioxus::prelude::*;

use crate::components::{button::{Button, ButtonVariant}, card::{Card, CardAction, CardDescription, CardHeader, CardTitle, CardContent, CardFooter}, input::Input, label::Label};

#[component]
pub fn ModelCard(model_name: String) -> Element {
    rsx! {
        Card { style: "width: 100%; max-width: 24rem; margin: 0.5rem",
            CardHeader {
                CardTitle { {model_name} }
                CardDescription { "<Model description placeholder>" }
                CardAction {
                    Button { variant: ButtonVariant::Ghost, "Expand" }
                }
            }
            CardContent {
            }
            CardFooter { style: "flex-direction: row; gap: 0.5rem;",
                Button { variant: ButtonVariant::Primary, "Download" }
                Button { variant: ButtonVariant::Destructive, "Delete" }
            }
        }
    }
}