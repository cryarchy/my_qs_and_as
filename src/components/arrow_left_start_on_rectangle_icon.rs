use dioxus::prelude::*;

#[component]
pub fn ArrowLeftStartOnRectangleIcon(size_class: String) -> Element {
    rsx! {
        svg {
            class: size_class,
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M8.25 9V5.25A2.25 2.25 0 0 1 10.5 3h6a2.25 2.25 0 0 1 2.25 2.25v13.5A2.25 2.25 0 0 1 16.5 21h-6a2.25 2.25 0 0 1-2.25-2.25V15m-3 0-3-3m0 0 3-3m-3 3H15",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
