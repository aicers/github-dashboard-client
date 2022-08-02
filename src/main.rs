mod fetch;
mod home;

use crate::home::{Model, Props};

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
    let props = Props {};
    yew::start_app_with_props::<Model>(props);
}
