use yew::prelude::*;
use yew_router::prelude::*;

use crate::home::Model as HomePage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/stats")]
    Stats,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/> },
        Route::Stats => html! { <super::stats_page::StatsPage /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}
