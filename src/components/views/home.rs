use leptos::*;
use crate::app::components::icons::AddIcon;

#[component]
pub fn HomeView() -> impl IntoView {
    let (state, set_state) = create_signal("normal");

    view! {
        <main id="home_view">
            <div id="modal_add" class=move || if state.get() == "add" {"active"} else {""}>
                <div>
                    <section id="buttons">
                        <button id="cancel" on:click=move |_| set_state.set("normal")></button>
                        <button id="confirm"></button>
                    </section>
                </div>
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
