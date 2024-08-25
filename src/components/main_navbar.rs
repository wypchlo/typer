use leptos::*;

use crate::app::components::icons::{HomeIcon, HomeFillIcon, FlagIcon, FlagFillIcon, SettingsIcon, SettingsFillIcon};

#[component]
pub fn MainNavbar() -> impl IntoView {
    let current_path = move || leptos_router::use_location().pathname.get();
    let is_focused = move |path: &str| if current_path() == path {"focused"} else {""};

    let (c_hovered, set_c_hovered) = create_signal(0);
    let is_hovered = move |index: i8| if c_hovered.get() == index {"hovered"} else {""};
    
    let link_class = move |path: &str, index: i8| format!("{} {}", is_focused(path), is_hovered(index));
    let touch_start = move |index: i8| move |_| set_c_hovered.set(index);
    let touch_stop = move |_| set_c_hovered.set(0);

    view! {
        <nav id="main_navbar">
            <a href="/" class=move || link_class("/", 1)
            on:touchstart=touch_start(1) on:touchend=touch_stop on:touchcancel=touch_stop>

                <Show when=move || { current_path() == "/" } fallback=|| view! { 
                    <HomeIcon/> Home }> 
                    <HomeFillIcon/> Home 
                </Show>
            </a>
            
            <a href="/languages" class=move || link_class("/languages", 2)
            on:touchstart=touch_start(2) on:touchend=touch_stop on:touchcancel=touch_stop>

                <Show when=move || { current_path() == "/languages"} fallback=|| view! { 
                    <FlagIcon/> Languages }> 
                    <FlagFillIcon/> Languages 
                </Show>
            </a>
            
            <a href="/settings" class=move || link_class("/settings", 3)
            on:touchstart=touch_start(3) on:touchend=touch_stop on:touchcancel=touch_stop>

                <Show when=move || { current_path() == "/settings" } fallback=|| view! {
                    <SettingsIcon/> Settings }>
                    <SettingsFillIcon/> Settings 
                </Show>
            </a>
        </nav>
    }
}
