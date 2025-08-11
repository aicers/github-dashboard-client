use chrono::{DateTime, Utc};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// GraphQL 응답 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse {
    pub data: QueryData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub enum Msg {
    UpdateQuery(String),
    SubmitQuery,
    QueryResponse(Result<GraphQLResponse, String>),
    ClearHistory,
    NoOp,
}

pub struct RAGQaComponent {
    current_query: String,
    query_history: Vec<QueryResult>,
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
                let query = self.current_query.clone();
                let link = ctx.link().clone();

                spawn_local(async move {
                    let result = Self::send_graphql_query(&query).await;
                    link.send_message(Msg::QueryResponse(result));
                });

                self.current_query.clear();
                true
            }
            Msg::QueryResponse(result) => {
                self.is_loading = false;
                match result {
                    Ok(response) => {
                        self.query_history.push(response.data.query);
                    }
                    Err(error) => {
                        web_sys::console::error_1(&format!("Query failed: {error}").into());
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
                        disabled={self.query_history.is_empty()}
                    >
                        {"Clear History"}
                    </button>
                </div>

                <div class="qa-chat-area">
                    {self.render_chat_history()}
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
                                if e.key() == "Enter" {
                                    Msg::SubmitQuery
                                } else {
                                    Msg::NoOp
                                }
                            })}
                            disabled={self.is_loading}
                        />
                        <button
                            class="submit-btn"
                            onclick={ctx.link().callback(|_| Msg::SubmitQuery)}
                            disabled={self.is_loading || self.current_query.trim().is_empty()}
                        >
                            {if self.is_loading { "Loading..." } else { "Send" }}
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
                    <p>{"질문을 입력해보세요!"}</p>
                </div>
            };
        }

        html! {
            <div class="chat-history">
                {for self.query_history.iter().rev().map(|item| self.render_qa_item(item))}
            </div>
        }
    }
    #[allow(clippy::unused_self)]
    fn render_qa_item(&self, item: &QueryResult) -> Html {
        let cleaned_answer = Self::clean_answer(&item.answer);
        let formatted_time = item.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

        html! {
            <div class="qa-item">
                <div class="question-bubble">
                    <div class="bubble-content">
                        <strong>{"Q: "}</strong>
                        {&item.query}
                    </div>
                    <div class="timestamp">{formatted_time}</div>
                </div>

                <div class="answer-bubble">
                    <div class="bubble-content">
                        <strong>{"A: "}</strong>
                        <div class="answer-text">
                            { Html::from_html_unchecked(AttrValue::from(cleaned_answer)) }
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn clean_answer(raw_answer: &str) -> String {
        let unescaped = raw_answer.replace("\\\"", "\"").replace("\\n", "\n");

        let re = regex::Regex::new(r"<think>.*?</think>").unwrap();
        let without_think = re.replace_all(&unescaped, "");

        let trimmed = without_think.trim_matches('"').trim();

        let with_bold = trimmed.replace("**", "<strong>").replace("**", "</strong>");

        with_bold.to_string()
    }

    async fn send_graphql_query(query: &str) -> Result<GraphQLResponse, String> {
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
            variables: serde_json::json!({
                "query": query
            }),
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
        } else {
            Err(format!("HTTP Error: {}", response.status()))
        }
    }
}

#[function_component(RAGComponent)]
pub fn app() -> Html {
    html! {
        <>
            <RAGQaComponent />
        </>
    }
}
