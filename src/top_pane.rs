use yew::{
    prelude::*,
    {html, Component, Context, Html},
};

pub(crate) struct TopModel;
pub(crate) enum Message {}

#[derive(Clone, Eq, PartialEq, Properties)]
pub(crate) struct Props {
    pub email: String,
}

impl Component for TopModel {
    type Message = Message;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let top_style = "background: rgb(204, 255, 229); width:100%; height:40px; margin:-8px;";
        let email_style = "font-size: 30px; float: right;";
        html! {
            <div style={top_style}>
                <div style={email_style}>{&ctx.props().email}</div>
            </div>
        }
    }
}
