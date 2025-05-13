use dioxus::prelude::*;

use crate::components::Bars3Icon;

#[component]
pub fn ShowSidebarButton(sidebar_visible: Signal<bool>) -> Element {
    rsx! {
        button { class: "text-[#6b3f1d] rounded-xl font-bold text-xl",
            onclick: move |_evt| {
                sidebar_visible.set(true);
            },
            Bars3Icon { size_class: "size-12" },
        }
    }
}
