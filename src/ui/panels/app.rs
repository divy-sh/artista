use dioxus::prelude::*;
use lucide_dioxus::{CircleUser, House, SlidersHorizontal, Wallet};

use crate::ui::components::bottom_nav::{
    BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel,
};

// Assuming your panel components are located here based on your first prompt
use crate::ui::panels::{history::History, home::Home, models::Models, settings::Settings};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum NavPage {
    Home,
    #[default]
    History,
    Models,
    Settings,
}

impl NavPage {
    fn label(self) -> &'static str {
        match self {
            NavPage::Home => "Home",
            NavPage::History => "History",
            NavPage::Models => "Models",
            NavPage::Settings => "Settings",
        }
    }

    fn icon(self) -> Element {
        match self {
            NavPage::Home => rsx! { House { class: "size-5" } },
            NavPage::History => rsx! { Wallet { class: "size-5" } },
            NavPage::Models => rsx! { CircleUser { class: "size-5" } },
            NavPage::Settings => rsx! { SlidersHorizontal { class: "size-5" } },
        }
    }
}

const PAGES: &[NavPage] = &[
    NavPage::Home,
    NavPage::History,
    NavPage::Models,
    NavPage::Settings,
];

#[component]
pub fn App() -> Element {
    let mut active = use_signal(|| NavPage::Home);

    rsx! {
        div { class: "min-h-screen w-full flex flex-col",

            // Added `flex flex-col items-center justify-center` to center panel content
            div { class: "flex-1 w-full flex flex-col items-center justify-center overflow-y-auto p-4",
                match active() {
                    NavPage::Home => rsx! { Home {} },
                    NavPage::History => rsx! { History {} },
                    NavPage::Models => rsx! { Models {} },
                    NavPage::Settings => rsx! { Settings {} },
                }
            }

            // Stays at the bottom of the viewport
            div { class: "w-full",
                BottomNav {
                    BottomNavGrid {
                        for page in PAGES {
                            {
                                let page = *page;
                                rsx! {
                                    BottomNavButton {
                                        onclick: move |_| active.set(page),
                                        aria_current: if active() == page { "page" } else { "" },
                                        {page.icon()}
                                        BottomNavLabel { {page.label()} }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
