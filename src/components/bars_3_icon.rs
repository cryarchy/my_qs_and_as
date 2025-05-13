use dioxus::prelude::*;

#[component]
pub fn Bars3Icon(size_class: String) -> Element {
    rsx! {
        svg {
            class: size_class,
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
