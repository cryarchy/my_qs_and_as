use dioxus::prelude::*;
use validator::ValidateEmail;

use super::{login_button::LoginButton, password_input::PasswordInput};
use crate::components::EmailAddressInput;

#[component]
pub fn LoginForm() -> Element {
    let email_address: Signal<Option<String>> = use_signal(|| None);
    let password: Signal<Option<String>> = use_signal(|| None);

    let email_valid = use_memo(move || match email_address() {
        Some(email_address) => email_address.validate_email(),
        None => false,
    });
    let password_set = use_memo(move || password().map(|p| !p.is_empty()).unwrap_or_default());

    let login_disabled = use_memo(move || !email_valid() || !password_set());

    rsx! {
        form { class: "flex flex-col gap-4 w-full max-w-md",

            label { class: "text-left font-medium", r#for: "email", "Username / Email Address" }
            EmailAddressInput { email_address }
            span {
                class: "text-xs hidden text-red-600 peer-[&:not(:placeholder-shown):not(:focus):invalid]:block",
                "Please enter a valid email address"
            }

            label { class: "text-left font-medium", r#for: "password", "Password" }
            PasswordInput { password }

            div { class: "text-sm text-gray-600",
                "Don't have an account? ",
                Link {
                    class: "font-semibold underline",
                    to: crate::Route::Signup,
                    "SIGN UP"
                }
            }

            LoginButton { disabled: login_disabled }
        }
    }
}
