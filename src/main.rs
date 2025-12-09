mod components;
use crate::components::left_side::LeftSide;
use crate::components::right_side::RightSide;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
         <div class="container">
            <LeftSide />
            <RightSide />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
