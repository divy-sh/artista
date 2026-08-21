use dioxus::document::eval;
use dioxus::prelude::*;

use ui::components::button::Button;
use ui::panels::home::Home;

use crate::core::database::Database;

mod core;
mod ui;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

static DATABASE_INSTANCE: GlobalSignal<&Database> =
    Signal::global(|| core::database::Database::get_db());

fn main() {
    dioxus::launch(App);
    _ = DATABASE_INSTANCE.read().init_conversation_dao();
}

#[component]
fn App() -> Element {
    let mut is_dark = use_signal(|| true);

    use_effect(move || {
        let dark_class = if is_dark() { "dark" } else { "" };
        let js = format!("document.documentElement.className = '{}';", dark_class);
        eval(&js);
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Button {
            class: "fixed bottom-4 right-4 z-50 p-2 text-xs font-medium rounded-full border border-border bg-card text-card-foreground shadow-md cursor-pointer hover:bg-muted",
            onclick: move |_| is_dark.toggle(),
            if is_dark() { "☀️ Light" } else { "🌙 Dark" },
        }

        Home {}
    }
}
