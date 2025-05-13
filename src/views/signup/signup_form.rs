use dioxus::prelude::*;
use validator::ValidateEmail;

use super::signup_button::SignupButton;
use crate::components::EmailAddressInput;

#[component]
pub fn SignupForm() -> Element {
    let email_address: Signal<Option<String>> = use_signal(|| None);

    let signup_disabled = use_memo(move || match email_address() {
        Some(email_address) => !email_address.validate_email(),
        None => true,
    });

    rsx! {
        form { class: "flex flex-col gap-4 w-full max-w-md",

            label { class: "text-left font-medium", r#for: "email", "Email Address" }
            EmailAddressInput { email_address }
            span {
                class: "text-xs hidden text-red-600 peer-[&:not(:placeholder-shown):not(:focus):invalid]:block",
                "Please enter a valid email address"
            }

            div { class: "text-sm text-gray-600",
                "Already have an account? ",
                Link {
                    class: "font-semibold underline",
                    to: crate::Route::Login,
                    "LOGIN"
                }
            }

            SignupButton { disabled: signup_disabled }
        }
    }
}
