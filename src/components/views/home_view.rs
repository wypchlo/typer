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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_date: String
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
            set_name_error.set(String::new());
            set_name.set(String::new());
            set_description.set(String::new());
        }
    };

    let cancel = move |event: leptos::ev::MouseEvent| {
        event.prevent_default();
        clear_form();
        set_state.set("normal");
    };
    
    let (sets, set_sets) = create_signal(Vec::<Set>::new());

    let fetch_sets = move || {
        spawn_local(async move {
            let result = invoke("get_all_sets", wasm_bindgen::JsValue::null()).await;
            match serde_wasm_bindgen::from_value::<Vec<Set>>(result) {
                Ok(fetched_sets) => set_sets.set(fetched_sets),
                Err(e) => println!("Failed to deserialize sets: {:?}", e),
            }
        });
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

    fetch_sets();

    view! {
        <main id="home_view">
            <div id="modal_add" class=move || if state.get() == "add" {"active"} else {""}>
                <form on:submit=on_submit ref=form_ref>
                    <section id="inputs">
                        <textarea on:input=update_name id="name" placeholder="Set name"/>
                        <Show when=move || !name_error.get().is_empty()>
                            <p class="error">{move || name_error}</p>
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

            <section id="content">
                <div class="seperator"> Recent <hr/> </div>
                <ul>
                    {move || sets.get().iter().map(|set| view! {
                        <li>
                            <strong>{&set.name}</strong>
                            <p>{set.description.clone().unwrap_or_else(|| "No description".to_string())}</p>
                            <small>{"Created on: "}{&set.created_date}</small>
                        </li>
                    }).collect_view()}
                </ul>
            </section>
        </main>
    }
}
