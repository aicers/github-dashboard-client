use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <nav class="navbar">
            <div class="nav-brand">
                <Link<Route> to={Route::Home}>
                    <h2>{"AICE GitHub Dashboard"}</h2>
                </Link<Route>>
            </div>
            <div class="nav-links">
                <Link<Route> to={Route::Home} classes="nav-link">
                    {"Home"}
                </Link<Route>>
                <Link<Route> to={Route::Dashboard} classes="nav-link">
                    {"Dashboard"}
                </Link<Route>>
                <Link<Route> to={Route::Search} classes="nav-link">
                    {"Search"}
                </Link<Route>>
                <Link<Route> to={Route::Profile} classes="nav-link">
                    {"Profile"}
                </Link<Route>>
                <Link<Route> to={Route::Settings} classes="nav-link">
                    {"Settings"}
                </Link<Route>>
                <Link<Route> to={Route::About} classes="nav-link">
                    {"About"}
                </Link<Route>>
            </div>
        </nav>
    }
}
