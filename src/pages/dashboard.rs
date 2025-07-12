use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div>
            <h1>{"Dashboard"}</h1>
            <p>{"Welcome to the main dashboard page!"}</p>
            <div class="dashboard-content">
                <div class="widget">
                    <h3>{"Recent Activity"}</h3>
                    <p>{"View your recent GitHub activity here."}</p>
                </div>
                <div class="widget">
                    <h3>{"Quick Stats"}</h3>
                    <p>{"Overview of your repositories and contributions."}</p>
                </div>
            </div>
        </div>
    }
}
