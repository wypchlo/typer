use leptos::{component, view, leptos_dom::IntoView};

#[component]
pub fn CancelIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 512 512">
            <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M 357.82338,357.82338 154.17662,154.17662 m 0,203.64676 203.64676,-203.64676"/>
        </svg>
    }
}
