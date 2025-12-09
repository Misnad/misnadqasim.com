use leptos::prelude::*;

#[component]
pub fn LeftSide() -> impl IntoView {
    view! {
        <div class="left-side">
            <div class="left-inside">
                <img
                    src="assets/images/icon.jpg"
                    alt="Profile Picture"
                    class="profile-pic"
                />
                <br />
                <p class="left-text">
                    "Hi! I'm Misnad. "
                    <span class="left-text light">
                        "A programmer and student. I like playing with
                        computer and exploring nature."
                    </span>
                </p>
                <div class="left-bottom">
                <a href="https://gitlab.com/Misnad" target="blank"
                    ><img
                        src="assets/icons/gitlab.png"
                        alt="Gitlab"
                        class="icons"
                /></a>
                    <a href="https://github.com/Misnad" target="blank"
                        ><img
                            src="assets/icons/github.png"
                            alt="Github"
                            class="icons"
                    /></a>
                    <a href="https://mastodon.social/@misnad" target="blank"
                        ><img
                            src="assets/icons/mastodon.png"
                            alt="Mastodon"
                            class="icons"
                    /></a>
                    <a href="https://linkedin.com/in/Misnad" target="blank"
                        ><img
                            src="assets/icons/linkedin.png"
                            alt="Linkedin"
                            class="icons"
                    /></a>
                </div>
            </div>
        </div>
    }
}
