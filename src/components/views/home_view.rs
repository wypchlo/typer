use leptos::*;
use crate::app::components::icons::{AddIcon, CancelIcon, ConfirmIcon};

#[component]
pub fn HomeView() -> impl IntoView {
    let (state, set_state) = create_signal("normal");
    
    let on_submit = move |event: leptos::ev::SubmitEvent| event.prevent_default();

    view! {
        <main id="home_view">
            <div id="modal_add" class=move || if state.get() == "add" {"active"} else {""}>
                <form on:submit=on_submit>
                    <section id="inputs">
                        name
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
