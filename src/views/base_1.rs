use dioxus::prelude::*;

use crate::components::{Navbar, Sidebar};

#[component]
pub fn Base1() -> Element {
    let sidebar_visible = use_signal(|| false);

    rsx! {
        Sidebar { sidebar_visible }
        Navbar { sidebar_visible }
        Outlet::<crate::Route> {}
    }
}
