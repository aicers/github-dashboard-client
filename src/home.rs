use crate::fetch::{Common, QueryIssue};
use crate::CommonError;
use gloo_console::log;
use gloo_events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::{
    prelude::*,
    {html, Component, Context, Html, NodeRef},
};

use crate::fetch::Issues;

pub enum Message {
    QueryResult(Vec<Issues>),
    SignIn(JsValue),
    Err(CommonError),
}

pub struct Model {
    res_query: Vec<Issues>,
    node_ref: NodeRef,
    click_listener: Option<EventListener>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

impl QueryIssue for Model {
    fn success_issues_info(issues: Vec<Issues>) -> Self::Message {
        Message::QueryResult(issues)
    }
}

impl Common for Model {
    fn common_error(error: CommonError) -> Self::Message {
        Message::Err(error)
    }
}

impl Component for Model {
    type Message = Message;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            res_query: Vec::new(),
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
            Message::Err(error) => {
                log!("error", format!("{:#?}", error));
                false
            }
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
                    Message::Err(CommonError::UnknownError)
                }
            });
            let listener = EventListener::new(&element, "onsuccess", move |e: &Event| {
                callback.emit(e.clone());
            });
            self.click_listener = Some(listener);
        }
        self.fetch_iussue_info(ctx);
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
                <div>
                <p>{ "AICE GitHub Dashboard" }</p>

                <table border="1px">
                {
                    for self.res_query.iter().map(|(number,title)| {
                        html! {
                            <tr>
                                <td>{number}</td>
                                <td>{title}</td>
                            </tr>
                        }
                    })
                }
                </table>
                <div ref={self.node_ref.clone()} id="my-signin2"/>
                </div>
        }
    }
}
