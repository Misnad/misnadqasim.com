use leptos::prelude::*;
use leptos_use::{UseWindowSizeReturn, use_window_size};

#[component]
pub fn RightSide() -> impl IntoView {
    let (count, set_count) = signal(0);
    let UseWindowSizeReturn { width, height } = use_window_size();
    let width = width.get() as i32;

    view! {
        <div class=("right-side", true)>
            <div class=("right-inside", true)>
                <h1>"Recent works"</h1>
                <p>
                    "Lorem ipsum dolor sit, amet consectetur adipisicing \
                    elit. Cupiditate ducimus provident facilis odit nulla, \
                    totam dolorem quia iusto nostrum, atque necessitatibus \
                    earum eligendi tempora. Possimus cumque officia laborum \
                    porro harum"
                </p>
                <p>
                    "Lorem ipsum dolor sit amet consectetur adipisicing elit.
                    Porro, error molestiae dolorum suscipit ullam ducimus.
                    Incidunt dolore eum sapiente dolores tenetur deleniti
                    quibusdam quos corrupti? Unde incidunt animi molestias
                    tenetur."
                </p>

            </div>

            <div
                // class=("right-inside", move || { count.get() < 20 || width > 900 } )
                style:text-align="center"
                style:margin-bottom="10rem"
            >
                <button
                    on:click=move |_| set_count.set(count.get() + 1)
                    class="no-double-tap-zoom"
                    style:padding="2rem"
                >
                    "Click here for fun: " {count}
                </button>
                <Show
                    when=move || { count.get() >= 20 }
                    fallback=|| view! { <p>"Keep clicking..."</p> }
                >
                    <p>"You can stop now. Thanks a lot for clicking."</p>
                    <p>"Here's your fun."</p>
                    <iframe src="http://www.staggeringbeauty.com/" style="border: 1px inset #ddd" width={width.min(598) - 10} height="598"></iframe>
                </Show>
            </div>
        </div>
    }
}
