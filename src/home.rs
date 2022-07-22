use gloo_net::http::Request;
use yew::prelude::*;

const QUERY: &str = "{\"query\": \"{ issues }\"}";

pub enum Message {
    QueryResult(String),
    Err,
}

pub struct Model {
    res_query: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

impl Component for Model {
    type Message = Message;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            res_query: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::QueryResult(text) => {
                self.res_query = text;
                true
            }
            Message::Err => false,
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
            </div>
        }
    }
}
