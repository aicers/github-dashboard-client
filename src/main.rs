mod home;

use crate::home::{Model, Props};
use yew;

fn main() {
    let props = Props {};
    yew::start_app_with_props::<Model>(props);
}
