mod login_button;
mod login_form;
mod password_input;

use dioxus::prelude::*;

use login_form::LoginForm;

#[component]
pub fn Login() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen text-[#6b3f1d] px-4",
            div { class: "text-3xl font-semibold mb-4", "Login" }

            div { class: "text-center text-gray-600 mb-6 border border-gray-200 rounded p-4 max-w-md",
                "Welcome back! We're dedicated to providing you with a seamless Q&A experience."
            }

            LoginForm {  }

            div { class: "my-4 text-gray-600", "Or" }

            div { class: "flex flex-col gap-2 w-full max-w-md",
                button {
                    class: "border border-gray-400 rounded px-4 py-2 hover:bg-gray-100 transition",
                    "Continue with Google"
                }
                button {
                    class: "border border-gray-400 rounded px-4 py-2 hover:bg-gray-100 transition",
                    "Continue with GitHub"
                }
            }

            div { class: "text-sm text-gray-600 mt-6 border border-gray-200 rounded p-4 max-w-md text-center",
                "If you encounter any issues or have ideas for new features, please don't hesitate to share your feedback. Your insights help us improve."
            }
        }
    }
}
