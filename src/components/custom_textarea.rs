use leptos::*;

#[component]
pub fn Textarea(placeholder: &'static str, node_ref: NodeRef<html::Textarea>) -> impl IntoView {
    let (value, set_value) = create_signal(String::new());
    
    let resizer_value_handler = move || if value.get().is_empty() {String::from(placeholder)} else {value.get()};
    
    view! {
        <div class="custom-textarea-container">
            <div class="resizer textarea">{resizer_value_handler}</div>
            <textarea rows="1" class="textarea" node_ref=node_ref on:input=move |ev| set_value.set(event_target_value(&ev)) placeholder=placeholder></textarea>
        </div>
    } 
}
