use leptos::{component, leptos_dom::IntoView, view};

#[component]
pub fn HomeFillIcon() -> impl IntoView {
    view! {
        <svg width="1em" height="1em" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path d="M 9,22 V 12 h 6 V 14.714848 22 h 4 c 1.104569,0 2,-0.895431 2,-2 V 9 L 12,2 3,9 v 11 c 0,1.104569 0.8954305,2 2,2 z" style="fill:currentColor;fill-opacity:1;stroke:currentColor;stroke-width:1.33333;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none" />
        </svg>
    }
}
