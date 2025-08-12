use chrono::{DateTime, Utc};
use gloo_net::http::Request;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse {
    pub data: QueryData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryData {
    pub query: QueryResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub query: String,
    pub answer: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChatItem {
    pub id: Uuid,
    pub query: String,
    pub answer: Option<String>,
    pub timestamp: DateTime<Utc>,
}

pub enum Msg {
    UpdateQuery(String),
    SubmitQuery,
    QueryResponse(Uuid, Result<QueryResult, String>),
    ClearHistory,
    NoOp,
}

pub struct RAGQaComponent {
    current_query: String,
    query_history: Vec<ChatItem>,
    is_loading: bool,
    input_ref: NodeRef,
}

impl Component for RAGQaComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current_query: String::new(),
            query_history: Vec::new(),
            is_loading: false,
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateQuery(query) => {
                self.current_query = query;
                true
            }
            Msg::SubmitQuery => {
                if self.current_query.trim().is_empty() || self.is_loading {
                    return false;
                }

                self.is_loading = true;
                let query_text = self.current_query.clone();

                let new_chat_item = ChatItem {
                    id: Uuid::new_v4(),
                    query: query_text.clone(),
                    answer: None,
                    timestamp: Utc::now(),
                };
                self.query_history.insert(0, new_chat_item.clone());

                let link = ctx.link().clone();
                let item_id = new_chat_item.id;

                spawn_local(async move {
                    let result = Self::send_graphql_query(&query_text).await;
                    link.send_message(Msg::QueryResponse(item_id, result));
                });

                self.current_query.clear();
                true
            }
            Msg::QueryResponse(id, result) => {
                self.is_loading = false;

                if let Some(item_to_update) =
                    self.query_history.iter_mut().find(|item| item.id == id)
                {
                    match result {
                        Ok(response) => {
                            item_to_update.answer = Some(response.answer);
                        }
                        Err(error) => {
                            web_sys::console::error_1(&format!("Query failed: {error}").into());
                            item_to_update.answer =
                                Some(format!("**오류가 발생했습니다:**\n\n```\n{error}\n```"));
                        }
                    }
                }
                true
            }
            Msg::ClearHistory => {
                self.query_history.clear();
                true
            }
            Msg::NoOp => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("qa-container")}>
                <div class="qa-header">
                    <h2>{"GitHub Repository Q&A"}</h2>
                    <button
                        class={classes!("clear-btn")}
                        onclick={ctx.link().callback(|_| Msg::ClearHistory)}
                        disabled={self.query_history.is_empty() || self.is_loading}
                    >
                        {"기록 삭제"}
                    </button>
                </div>

                <div class="qa-chat-area">
                    { self.render_chat_history() }
                </div>

                <div class="qa-input-area">
                    <div class="input-group">
                        <input
                            ref={self.input_ref.clone()}
                            type="text"
                            class="query-input"
                            placeholder="질문을 입력하세요..."
                            value={self.current_query.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateQuery(input.value())
                            })}
                            onkeydown={ctx.link().callback(|e: KeyboardEvent| {
                                if e.key() == "Enter" { Msg::SubmitQuery } else { Msg::NoOp }
                            })}
                            disabled={self.is_loading}
                        />
                        <button
                            class="submit-btn"
                            onclick={ctx.link().callback(|_| Msg::SubmitQuery)}
                            disabled={self.is_loading || self.current_query.trim().is_empty()}
                        >
                            { "전송" }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}

impl RAGQaComponent {
    fn render_chat_history(&self) -> Html {
        if self.query_history.is_empty() {
            return html! {
                <div class="empty-state">
                    <p>{"질문을 입력하여 분석을 시작하세요!"}</p>
                </div>
            };
        }

        html! {
            <div class="chat-history">
                {for self.query_history.iter().map(Self::render_qa_item)}
            </div>
        }
    }

    fn render_qa_item(item: &ChatItem) -> Html {
        let formatted_time = item.timestamp.format("%Y-%m-%d %H:%M").to_string();

        html! {
            <div class="qa-item">
                <div class="question-bubble">
                    <div class="bubble-content">
                        {&item.query}
                    </div>
                    <div class="timestamp">{formatted_time}</div>
                </div>

                {
                    if let Some(answer_text) = &item.answer {
                        let answer_html = Self::markdown_to_html(answer_text);
                        html! {
                            <div class="answer-bubble">
                                <div class="markdown-content">
                                    { Html::from_html_unchecked(AttrValue::from(answer_html)) }
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="answer-bubble loading-bubble">
                                <div class="typing-indicator">
                                    <span></span>
                                    <span></span>
                                    <span></span>
                                </div>
                            </div>
                        }
                    }
                }
            </div>
        }
    }

    fn markdown_to_html(markdown_input: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(markdown_input, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    async fn send_graphql_query(query: &str) -> Result<QueryResult, String> {
        let graphql_query = r"
            query Query($query: String!) {
                query(query: $query) {
                    query
                    answer
                    timestamp
                }
            }
        ";
        let request = GraphQLRequest {
            query: graphql_query.to_string(),
            variables: serde_json::json!({ "query": query }),
        };
        let response = Request::post("/graphql")
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Request creation failed: {e}"))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        if response.ok() {
            response
                .json::<GraphQLResponse>()
                .await
                .map_err(|e| format!("JSON parsing failed: {e}"))
                .map(|gql_response| gql_response.data.query)
        } else {
            Err(format!("HTTP Error: {}", response.status()))
        }
    }
}

#[function_component(RagApp)]
pub fn app() -> Html {
    html! {
        <RAGQaComponent />
    }
}
