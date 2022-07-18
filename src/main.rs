mod home;

use crate::home::{Model, Props};

fn main() {
    let props = Props {};
    yew::start_app_with_props::<Model>(props);
}
