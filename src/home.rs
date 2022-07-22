//use std::sync::mpsc::channel;

//use gloo_net::http::Request;
use yew::prelude::*;

//const URL: &str = "127.0.0.1:8000";
//const QUERY: &str = "{\"query\": \"{ issues }\"}";

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
            <p>
            { "AICE GitHub Dashboard" }
            //{ self.home_view(ctx)}
            </p>
            </div>
        }
    }
}
/*
impl Model {
    fn home_view(&self, _ctx: &Context<Self>) -> Html {
        let (tx, rx) = channel();
        let mut uri = URL.to_string();
        uri.push_str("/graphql");

        wasm_bindgen_futures::spawn_local(async move {
            let post_str = Request::post(&uri)
                .header("Content-Type", "application/json")
                .body(QUERY)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            tx.send(post_str).unwrap();
        });
        let res_str = rx.recv().unwrap();

        html! {
            <div>{res_str}</div>
        }
    }
}
*/
