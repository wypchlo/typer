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
pub fn HomeView(set_hide_navbar: WriteSignal<bool>) -> impl IntoView {
    let form_ref = create_node_ref::<leptos::html::Form>();
    
    let (selected, set_selected) = create_signal(Vec::<i32>::new());
    let (pressed, set_pressed) = create_signal(Vec::<i32>::new());

    let (state, set_state) = create_signal("normal");
    
    let (name, set_name) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());

    let (name_error, set_name_error) = create_signal(String::new());

    let clear_form = move || {
        if let Some(form) = form_ref.get() {
            form.reset();
            set_name.set(String::new());
            set_name_error.set(String::new());
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

            fetch_sets();
        });
    };
    
    create_effect(move |_| {
        match state.get() {
            "normal" => { 
                set_selected.set(Vec::<i32>::new());
                set_hide_navbar.set(false);
            }
            "add" => {
                set_selected.set(Vec::<i32>::new());
                set_hide_navbar.set(true);
            }
            _ => { set_hide_navbar.set(true) }
        }
    });

    use leptos_dom::helpers::IntervalHandle;
    use std::time::Duration;

    let (interval_handle, set_interval_handle) = create_signal::<Option<IntervalHandle>>(None);
    let (hold_duration, set_hold_duration) = create_signal(0);
    let (moved, set_moved) = create_signal(false);

    let reset_interval = move || {
        if interval_handle.get().is_some() { 
            interval_handle.get().unwrap().clear();
            set_interval_handle.set(None);
        };
        set_hold_duration.set(0);
    };

    let set_touch_start = move |set_id: i32| {
        set_timeout(move || {
            if moved.get() { return }
            if !selected.get().is_empty() {
                return if selected.get().contains(&set_id) { set_selected.update(|selected| selected.retain(|value| *value != set_id)) }
                else { set_selected.update(|selected| selected.push(set_id)) };
            };
            
            set_pressed.update(|pressed| pressed.push(set_id));

            if pressed.get().len() > 1 { 
                set_state.set("select");
                return set_selected.update(|selected| selected.push(set_id)) 
            }

            let interval_handle = set_interval_with_handle(move || {
                set_hold_duration.update(|value| *value += 1);

                if hold_duration.get() > 30 || pressed.get().len() > 1 {
                    set_selected.update(|selected| selected.push(set_id));
                    set_state.set("select");
                    reset_interval();
                }
            }, Duration::from_millis(10));

            set_interval_handle.set(Some(interval_handle.unwrap()));
        }, Duration::from_millis(50));
    };

    let set_touch_move = move || { set_moved.set(true) };

    let set_touch_stop = move |set_id: i32| { 
        set_timeout(move || {
            if selected.get().is_empty() { set_state.set("normal") };
            set_pressed.update(|pressed| pressed.retain(|value| *value != set_id));
            reset_interval();
            set_moved.set(false);
        }, Duration::from_millis(50));
    };

    let link_class = move |id: i32| {
        let mut classes = vec!["set"];
        if selected.get().contains(&id) { classes.push("selected")};
        if pressed.get().contains(&id) { classes.push("pressed")};
        classes.join(" ")
    };
    
    fetch_sets();
    
    use crate::components::Textarea;

    view! {
        <main id="home_view">
            <form id="backdrop" on:submit=on_submit ref=form_ref class=move || if state.get() == "add" {"active"} else {""}>
                <div id="modal_container">
                    <div id="modal_add">
                        <Textarea placeholder="Set name" id="name" value=name set_value=set_name/>
                        <Show when=move || !name_error.get().is_empty()> <p class="error">{move || name_error}</p> </Show>
                        <Textarea placeholder="description" id="description" value=description set_value=set_description/>
                    </div>
                </div>

                <section id="buttons">
                    <button id="cancel" on:click=cancel><CancelIcon/></button>
                    <button id="confirm"><ConfirmIcon/></button>
                </section>
            </form>

            <header>
                <h1> Word Sets </h1> 
                <button on:click=move |_| set_state.set("add")> 
                    <AddIcon/> 
                </button>
            </header>

            <section id="content">
                <div class="seperator"> Recent <hr/> </div>
                <div id="sets_container">
                    { move || sets.get().iter().enumerate().map(|(index, set)| {
                        let set_id = sets.get().get(index).unwrap().id;
                        view! {
                            <button class=move || link_class(set_id)
                            on:touchmove=move |_| set_touch_move()
                            on:touchstart=move |_| set_touch_start(set_id)
                            on:touchend=move |_| set_touch_stop(set_id)
                            on:touchcancel=move |_| set_touch_stop(set_id)>
                                <h1>{&set.name}</h1>
                                <p>{set.description.as_ref().unwrap()}</p>
                            </button>
                            }
                        }).collect_view()
                    }
                </div>
            </section>
        </main>
    }
}
