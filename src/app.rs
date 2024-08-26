#[path = "./components/mod.rs"]
pub mod components;

use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

use components::MainNavbar;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=components::views::HomeView/>
                <Route path="/languages" view=components::views::LanguagesView/>
                <Route path="/settings" view=components::views::SettingsView/>
            </Routes>

            <MainNavbar/>
        </Router>
    }
}
