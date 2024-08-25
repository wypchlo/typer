use leptos::*;
use serde::{Serialize, Deserialize};
use crate::app::components::icons::{AddIcon, CancelIcon, ConfirmIcon};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct AddSetArgs<'a>{
    name: &'a str,
    description: &'a str
}

#[component]
pub fn HomeView() -> impl IntoView {
    let (state, set_state) = create_signal("normal");
    
    let (name, set_name) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    
    let update_name = move |event| {
        let name = event_target_value(&event); 
        set_name.set(name);
    };

    let update_description = move |event| {
        let description = event_target_value(&event);
        set_description.set(description);
    };

    let on_submit = move |event: leptos::ev::SubmitEvent| { 
        event.prevent_default(); 
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() { return }
            let description = description.get_untracked();

            let args = to_value(&AddSetArgs { name: &name, description: &description }).unwrap();
            let result = invoke("add_set", args).await;
        });
    };

    view! {
        <main id="home_view">
            <div id="modal_add" class=move || if state.get() == "add" {"active"} else {""}>
                <form on:submit=on_submit>
                    <section id="inputs">
                        <textarea on:input=update_name id="name" placeholder="Set name"/>
                        <textarea on:input=update_description id="description" placeholder="description"/>
                    </section>
                    <section id="buttons">
                        <button id="cancel" on:click=move |_| set_state.set("normal")><CancelIcon/></button>
                        <button id="confirm"><ConfirmIcon/></button>
                    </section>
                </form>
            </div>

            <header>
                <h1> Word Sets </h1> 
                <button on:click=move |_| set_state.set("add")> 
                    <AddIcon/> 
                </button>
            </header>

            <section>
                <div class="seperator"> Recent <hr/> </div>
            </section>
        </main>
    }
}
