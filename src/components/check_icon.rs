use dioxus::prelude::*;

#[component]
pub fn CheckIcon() -> Element {
    rsx! {
        svg {
            class: "size-6",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "m4.5 12.75 6 6 9-13.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
