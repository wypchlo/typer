use leptos::*;

#[component]
pub fn Textarea(
    placeholder: &'static str, 
    id: &'static str, 
    value: ReadSignal<String>,
    set_value: WriteSignal<String>
) -> impl IntoView { 
    let resizer_value_handler = move || if value.get().is_empty() {String::from(placeholder)+" "} else {value.get()+" "};
    
    view! {
        <div class="custom-textarea-container" id=id>
            <div class="resizer textarea">{resizer_value_handler}</div>
            <textarea rows="1" class="textarea" on:input=move |ev| set_value.set(event_target_value(&ev)) placeholder=placeholder></textarea>
        </div>
    } 
}
