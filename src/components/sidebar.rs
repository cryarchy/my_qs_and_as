use dioxus::prelude::*;

use super::HideSidebarButton;
use crate::components::ArrowLeftEndOnRectangleIcon;

#[component]
pub fn Sidebar(sidebar_visible: Signal<bool>) -> Element {
    let sidebar_display = use_memo(move || {
        if sidebar_visible() {
            "translate-x-0"
        } else {
            "-translate-x-full"
        }
    });

    let menu_item_leaf_clicked = move |_evt| {
        sidebar_visible.set(false);
    };

    rsx! {
        div {
            class: "fixed top-0 left-0 min-h-screen bg-[#f8f5f1] w-full p-2 transform -translate-x-full transition-transform duration-300 ease-in-out z-10 {sidebar_display}",
            div {
                class: "flex flex-row-reverse",
                HideSidebarButton { sidebar_visible }
            }
            Link {
                class: "flex flex-row gap-4 items-center text-xl",
                to: crate::Route::Login,
                onclick: menu_item_leaf_clicked,
                ArrowLeftEndOnRectangleIcon { size_class: "size-6" }
                span { "Login" }
            }
        }
    }
}
