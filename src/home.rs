use gloo_console::log;
use gloo_events::EventListener;
use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::{
    prelude::*,
    {html, Component, Context, Html, NodeRef},
};

const QUERY: &str = "{\"query\": \"{ issues }\"}";

pub enum Message {
    QueryResult(String),
    SignIn(JsValue),
    Err,
}

pub struct Model {
    res_query: String,
    node_ref: NodeRef,
    click_listener: Option<EventListener>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

impl Component for Model {
    type Message = Message;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            res_query: String::new(),
            node_ref: NodeRef::default(),
            click_listener: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::QueryResult(text) => {
                self.res_query = text;
                true
            }
            Message::Err => false,
            Message::SignIn(text) => {
                log!("email", text);
                false
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        if let Some(element) = self.node_ref.cast::<Element>() {
            let callback = ctx.link().callback(|e: Event| {
                if let Ok(js_email) = js_sys::Reflect::get(&e, &JsValue::from_str("detail")) {
                    Message::SignIn(js_email)
                } else {
                    Message::Err
                }
            });
            let listener = EventListener::new(&element, "onsuccess", move |e: &Event| {
                callback.emit(e.clone());
            });
            self.click_listener = Some(listener);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.link().send_future(async move {
            if let Ok(res) = Request::post("/graphql")
                .header("Content-Type", "application/json")
                .body(QUERY)
                .send()
                .await
            {
                if let Ok(text) = res.text().await {
                    Message::QueryResult(text)
                } else {
                    Message::Err
                }
            } else {
                Message::Err
            }
        });
        html! {
            <div>
            <p>{ "AICE GitHub Dashboard" }</p>
            <p>{self.res_query.clone()}</p>
            <div ref={self.node_ref.clone()} id="my-signin2"/>
            </div>
        }
    }
}
