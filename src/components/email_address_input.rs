use dioxus::prelude::*;

#[component]
pub fn EmailAddressInput(email_address: Signal<Option<String>>) -> Element {
    rsx! {
        input {
            r#type: "email",
            id: "email",
            placeholder: "email@example.com",
            class: "peer border border-gray-400 rounded px-4 py-2 w-full",
            oninput: move |event| email_address.set(Some(event.value()))
        }
    }
}
