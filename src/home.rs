use yew::prelude::*;

pub enum Message {}

pub struct Model {}
#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

impl Component for Model {
    type Message = Message;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{ "AICE Github Dashboard" }</p>
            </div>
        }
    }
}
