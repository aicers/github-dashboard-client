mod home;

use crate::home::{Model, Props};
use yew;

fn main() {
    println!("AICE Github Dashboard");
    let props = Props {};
    yew::start_app_with_props::<Model>(props);
}
