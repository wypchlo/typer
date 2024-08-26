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
    let form_ref = create_node_ref::<leptos::html::Form>();

    let (state, set_state) = create_signal("normal");
    
    let (name, set_name) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
   
    let update_name = move |event| set_name.set(event_target_value(&event));
    let update_description = move |event| set_description.set(event_target_value(&event));

    let (name_error, set_name_error) = create_signal(String::new());

    let clear_form = move || {
        if let Some(form) = form_ref.get() {
            form.reset();
        }
    };

    let cancel = move |event: leptos::ev::MouseEvent| {
        event.prevent_default();
        clear_form();
        set_state.set("normal");
    };

    let on_submit = move |event: leptos::ev::SubmitEvent| { 
        event.prevent_default(); 
        spawn_local(async move {
            let name = name.get_untracked();
            let description = description.get_untracked();

            if name.is_empty() { return set_name_error.set(String::from("Set name is required")) }

            let args = to_value(&AddSetArgs { name: &name, description: &description }).unwrap();
            let errors = invoke("add_set", args).await.as_string().unwrap();
            
            if errors.contains("UNIQUE") { set_name_error.set(String::from("A set with this name already exists")); }
            else { 
                set_state.set("normal");
                clear_form();
                set_name_error.set(String::new());
            }
        });
    };

    view! {
        <main id="home_view">
            <div id="modal_add" class=move || if state.get() == "add" {"active"} else {""}>
                <form on:submit=on_submit ref=form_ref>
                    <section id="inputs">
                        <textarea on:input=update_name id="name" placeholder="Set name"/>
                        <Show when=move || !name_error.get().is_empty()>
                            {move || name_error}
                        </Show>
                        <textarea on:input=update_description id="description" placeholder="description"/>
                    </section>
                    <section id="buttons">
                        <button id="cancel" on:click=cancel><CancelIcon/></button>
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
