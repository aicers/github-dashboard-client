use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <h1>{"About"}</h1>
            <p>{"AICE GitHub Dashboard - A powerful tool for managing your GitHub workflow."}</p>
            <div class="about-content">
                <h3>{"Features"}</h3>
                <ul>
                    <li>{"View pull requests and issues"}</li>
                    <li>{"Track review progress"}</li>
                    <li>{"Monitor repository activity"}</li>
                    <li>{"Manage assignees and reviewers"}</li>
                </ul>

                <h3>{"Version"}</h3>
                <p>{"Version 0.1.0"}</p>

                <h3>{"Contact"}</h3>
                <p>{"For support, please contact the AICE team."}</p>
            </div>
        </div>
    }
}
