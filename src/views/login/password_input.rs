use dioxus::prelude::*;

#[component]
pub fn PasswordInput(password: Signal<Option<String>>) -> Element {
    rsx! {
        input {
            r#type: "password",
            id: "password",
            placeholder: "********",
            class: "border border-gray-400 rounded px-4 py-2 w-full",
            oninput: move |event| password.set(Some(event.value()))
        }
    }
}
