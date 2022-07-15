mod home;

use crate::home::Props;
use yew;

fn main() {
    println!("AICE Github Dashboard");
    let props = Props {};
    yew::start_app_with_props::<crate::home::Model>(props);
}
