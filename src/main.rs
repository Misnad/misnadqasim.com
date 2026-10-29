mod components;
use crate::components::left_side::LeftSide;
use crate::components::right_side::RightSide;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;


#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <p>"404. Not found"</p> }>
                <Route path=path!("/")     view=Home/>
                <Route path=path!("/project01") view=ProjectPage/>
            </Routes>
        </Router>
    }
}


#[component]
fn Home() -> impl IntoView {
    view! {
         <div class="container">
            <LeftSide />
            <RightSide />
        </div>
    }
}

#[component]
fn ProjectPage() -> impl IntoView {
    view! {
        <h1>"A Unified Malware Classification Approach For Heterogeneous Consumer Devices"</h1>
        <ul>
            <li><a href="/public/A_Deep_Learning_Approach_for_Cross_Domain_Malware_Detection_Classification_based_on_Image_Representation_of_Binaries.pdf" rel="external">"Link to Draft Paper"</a></li>
            <li><a href="/public/MalwareClassification-Report.pdf" rel="external">"Link to Slides"</a></li>
        </ul>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
