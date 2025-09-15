mod fetch;
mod home;
mod top_pane;

#[derive(Debug)]
pub enum CommonError {
    SendGraphQLQueryError,
    HttpStatusNoSuccess,
    GraphQLResponseError,
    GraphQLParseError,
    ResponseParseError,
    UnknownError,
}

mod router;
mod stats_page;

use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav>
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::Stats}>{ "Stats" }</Link<Route>>
            </nav>
            <Switch<Route> render={|routes: router::Route| switch(&routes)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
