use gloo_console::log;
use yew::prelude::*;

pub enum Message {
    SignOut,
}

pub struct Model {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

impl Component for Model {
    type Message = Message;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SignOut => {
                log!("text", "hello");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <div class={"g-signin2"} data-onsuccess="onSignIn" ></div>
                <a href="#" onclick={link.callback(|_| Message::SignOut)}>{"Sign out"}</a>
            </div>
        }
    }
}
