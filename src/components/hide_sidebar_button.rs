use dioxus::prelude::*;

use super::XMarkIcon;

#[component]
pub fn HideSidebarButton(sidebar_visible: Signal<bool>) -> Element {
    rsx! {
        button { class: "text-[#6b3f1d] rounded-xl font-bold text-xl",
            onclick: move |_evt| {
                sidebar_visible.set(false);
            },
            XMarkIcon { size_class: "size-12" },
        }
    }
}
