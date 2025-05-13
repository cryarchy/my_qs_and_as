use dioxus::prelude::*;

#[component]
pub fn QAndAIcon() -> Element {
    rsx! {
        svg {
            width: "240",
            height: "120",
            view_box: "0 0 240 120",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                width: "240",
                height: "120",
                rx: "32",
                fill: "#f8f5f1",
            }
            text {
                x: "32",
                y: "80",
                font_family: "Playfair Display, serif",
                font_size: "72",
                fill: "#6b3f1d",
                font_weight: "bold",
                "Q"
            }
            text {
                x: "100",
                y: "90",
                font_family: "Brush Script MT, cursive",
                font_size: "60",
                fill: "#c7a07a",
                font_weight: "bold",
                "&"
            }
            text {
                x: "155",
                y: "60",
                font_family: "Playfair Display, serif",
                font_size: "56",
                fill: "#6b3f1d",
                font_weight: "bold",
                "A"
            }
            text {
                x: "192",
                y: "65",
                font_family: "Playfair Display, serif",
                font_size: "20",
                fill: "#6b3f1d",
                "s"
            }
        }
    }
}
