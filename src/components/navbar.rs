use dioxus::prelude::*;

use super::ShowSidebarButton;
use crate::Route;

#[component]
pub fn Navbar(sidebar_visible: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "flex flex-row p-2 justify-between",
            Link {
                class: "text-2xl text-[#6b3f1d] font-bold block",
                to: Route::Home,
                "HOME"
            }
            ShowSidebarButton { sidebar_visible }
        }
    }
}
