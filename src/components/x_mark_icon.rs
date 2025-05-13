use dioxus::prelude::*;

#[component]
pub fn XMarkIcon(size_class: String) -> Element {
    rsx! {
        svg {
            class: size_class,
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M6 18 18 6M6 6l12 12",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
