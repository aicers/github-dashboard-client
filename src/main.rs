mod fetch;
mod home;
mod top_pane;
mod pages;
mod navigation;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::home::Model as HomeModel;
use crate::pages::{dashboard::Dashboard, about::About, settings::Settings, profile::Profile};
use crate::navigation::Navigation;

#[derive(Debug)]
pub enum CommonError {
    SendGraphQLQueryError,
    HttpStatusNoSuccess,
    GraphQLResponseError,
    GraphQLParseError,
    ResponseParseError,
    UnknownError,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/profile")]
    Profile,
    #[at("/settings")]
    Settings,
    #[at("/about")]
    About,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomeModel /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::Profile => html! { <Profile /> },
        Route::Settings => html! { <Settings /> },
        Route::About => html! { <About /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navigation />
            <div class="main-content">
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
