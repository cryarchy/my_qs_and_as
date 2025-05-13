use dioxus::prelude::*;

use views::{Base1, Home, Login, PageNotFound, Signup};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Base1)]
        #[route("/")]
        Home,

        #[route("/login")]
        Login,

        #[route("/signup")]
        Signup,

        #[route("/:..route")]
        PageNotFound { route: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
