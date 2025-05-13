mod signup_button;
mod signup_form;

use dioxus::prelude::*;

use signup_form::SignupForm;

#[component]
pub fn Signup() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen text-[#6b3f1d] px-4",
            div { class: "text-3xl font-semibold mb-4", "Create an account" }

            div { class: "mb-6 rounded max-w-md flex flex-col gap-6",
                span {
                    "Proceed to unlock your personal Q&A vault!"
                }

                span {
                    "Create your free account now and start saving time on every application. Your perfect answers, organized, and AI-optimized, are waiting."
                }

                span {
                    "Let's get you set!"
                }

            }

            SignupForm {  }

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
