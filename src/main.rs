mod fetch;
mod home;
mod rag;
mod top_pane;

use crate::home::Model;

#[derive(Debug)]
pub enum CommonError {
    SendGraphQLQueryError,
    HttpStatusNoSuccess,
    GraphQLResponseError,
    GraphQLParseError,
    ResponseParseError,
    UnknownError,
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
