use crate::fetch::{Common, Issues, QueryIssue};
use crate::CommonError;
use gloo_console::log;
use gloo_events::EventListener;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::{
    prelude::*,
    {html, Component, Context, Html, NodeRef},
};

pub enum Message {
    QueryResult(Vec<Issues>),
    SignIn(Detail),
    Err(CommonError),
}

pub struct Model {
    res_query: Vec<Issues>,
    node_ref: NodeRef,
    click_listener: Option<EventListener>,
    id_token: String,
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Detail {
    pub email: String,
    pub token: String,
}

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
            id_token: String::new(),
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
            Message::SignIn(detail) => {
                self.id_token.push_str(&detail.token);
                log!("email", detail.email);
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
                if let Ok(js_val) = js_sys::Reflect::get(&e, &JsValue::from_str("detail")) {
                    if let Ok(detail_val) = js_val.into_serde::<Detail>() {
                        Message::SignIn(detail_val)
                    } else {
                        Message::Err(CommonError::UnknownError)
                    }
                } else {
                    Message::Err(CommonError::UnknownError)
                }
            });
            let listener = EventListener::new(&element, "onsuccess", move |e: &Event| {
                callback.emit(e.clone());
            });
            self.click_listener = Some(listener);
        }
        self.fetch_iussue_info(ctx, &self.id_token.clone());
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
                <div>
                <p>{ "AICE GitHub Dashboard" }</p>
                <table border="1px">
                    <tr>
                        <th>{"Number"}</th>
                        <th>{"Issue Title"}</th>
                    </tr>
                    {
                        for self.res_query.iter().map(|(owner, repo, number, title)| {
                            let href= format!("https://github.com/{}/{}/issues/{}", owner, repo, number);
                            html! {
                                <tr>
                                    <td align="center">{number}</td>
                                    <td><a href={href}>{title}</a></td>
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
