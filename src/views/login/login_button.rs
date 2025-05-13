use dioxus::prelude::*;

#[component]
pub fn LoginButton(disabled: ReadOnlySignal<bool>) -> Element {
    let disabled_class = use_memo(move || match disabled() {
        true => "disabled",
        false => "",
    });

    rsx! {
        button {
            r#type: "submit",
            class: "mt-2 border border-black rounded px-6 py-2 hover:bg-gray-100 transition {disabled_class}",
            "Login"
        }
    }
}
