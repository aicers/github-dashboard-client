use std::collections::HashMap;

use gloo_utils::window;
use web_sys::{console, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Clone, PartialEq, Debug)]
pub struct SearchFilters {
    pub query: String,
    pub repo_type: String,
    pub language: String,
    pub sort_by: String,
    pub per_page: u32,
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            query: String::new(),
            repo_type: "all".to_string(),
            language: "any".to_string(),
            sort_by: "updated".to_string(),
            per_page: 20,
        }
    }
}

impl SearchFilters {
    fn from_query_params(params: &HashMap<String, String>) -> Self {
        Self {
            query: params.get("q").cloned().unwrap_or_default(),
            repo_type: params
                .get("type")
                .cloned()
                .unwrap_or_else(|| "all".to_string()),
            language: params
                .get("language")
                .cloned()
                .unwrap_or_else(|| "any".to_string()),
            sort_by: params
                .get("sort")
                .cloned()
                .unwrap_or_else(|| "updated".to_string()),
            per_page: params
                .get("per_page")
                .and_then(|s| s.parse().ok())
                .unwrap_or(20),
        }
    }

    fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if !self.query.is_empty() {
            params.push(format!("q={}", urlencoding::encode(&self.query)));
        }
        if self.repo_type != "all" {
            params.push(format!("type={}", self.repo_type));
        }
        if self.language != "any" {
            params.push(format!("language={}", self.language));
        }
        if self.sort_by != "updated" {
            params.push(format!("sort={}", self.sort_by));
        }
        if self.per_page != 20 {
            params.push(format!("per_page={}", self.per_page));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

pub enum SearchMsg {
    UpdateQuery(String),
    UpdateRepoType(String),
    UpdateLanguage(String),
    UpdateSortBy(String),
    UpdatePerPage(u32),
    ApplyFilters,
}

pub struct Search {
    filters: SearchFilters,
    navigator: Navigator,
}

impl Component for Search {
    type Message = SearchMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let navigator = ctx.link().navigator().unwrap();
        let location = ctx.link().location().unwrap();

        // Parse query parameters from current URL
        let query_string = location.query_str();
        let query_params = parse_query_params(query_string);
        let filters = SearchFilters::from_query_params(&query_params);
        console::log_1(&format!("Parsed filters: {:?}", filters).into());

        Self { filters, navigator }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SearchMsg::UpdateQuery(query) => {
                self.filters.query = query;
                true
            }
            SearchMsg::UpdateRepoType(repo_type) => {
                self.filters.repo_type = repo_type;
                true
            }
            SearchMsg::UpdateLanguage(language) => {
                self.filters.language = language;
                true
            }
            SearchMsg::UpdateSortBy(sort_by) => {
                self.filters.sort_by = sort_by;
                true
            }
            SearchMsg::UpdatePerPage(per_page) => {
                self.filters.per_page = per_page;
                true
            }
            SearchMsg::ApplyFilters => {
                // Update URL with current filter values
                let query_string = self.filters.to_query_string();
                let new_url = format!("/search{}", query_string);
                self.navigator.push(&Route::Search);

                // Update browser history with query parameters
                if let Ok(history) = window().history() {
                    let _ = history.replace_state_with_url(
                        &wasm_bindgen::JsValue::NULL,
                        "",
                        Some(&new_url),
                    );
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_query_input = link.callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            SearchMsg::UpdateQuery(input.value())
        });

        let on_repo_type_change = link.callback(|e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            SearchMsg::UpdateRepoType(select.value())
        });

        let on_language_change = link.callback(|e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            SearchMsg::UpdateLanguage(select.value())
        });

        let on_sort_change = link.callback(|e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            SearchMsg::UpdateSortBy(select.value())
        });

        let on_per_page_change = link.callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<u32>() {
                SearchMsg::UpdatePerPage(value)
            } else {
                SearchMsg::UpdatePerPage(20)
            }
        });

        let on_apply = link.callback(|_| SearchMsg::ApplyFilters);

        console::log_1(&self.filters.repo_type.clone().into());

        html! {
            <div>
                <h1>{"Repository Search"}</h1>
                <p>{"This page demonstrates URL parameter synchronization with form values."}</p>

                <div class="search-form">
                    <div class="form-group">
                        <label for="search-query">{"Search Query:"}</label>
                        <input
                            id="search-query"
                            type="text"
                            placeholder="Enter search terms..."
                            value={self.filters.query.clone()}
                            oninput={on_query_input}
                        />
                    </div>

                    <div class="form-row">
                        <div class="form-group">
                            <label for="repo-type">{"Repository Type:"}</label>
                            <select id="repo-type" value={self.filters.repo_type.clone()} onchange={on_repo_type_change}>
                                <option value="all">{"All repositories"}</option>
                                <option value="public" selected=true>{"Public only"}</option>
                                <option value="private">{"Private only"}</option>
                                <option value="forks">{"Forks"}</option>
                            </select>
                        </div>

                        <div class="form-group">
                            <label for="language">{"Language:"}</label>
                            <select id="language" value={self.filters.language.clone()} onchange={on_language_change}>
                                <option value="any">{"Any language"}</option>
                                <option value="rust">{"Rust"}</option>
                                <option value="javascript">{"JavaScript"}</option>
                                <option value="typescript">{"TypeScript"}</option>
                                <option value="python">{"Python"}</option>
                                <option value="java">{"Java"}</option>
                                <option value="go">{"Go"}</option>
                                <option value="cpp">{"C++"}</option>
                            </select>
                        </div>
                    </div>

                    <div class="form-row">
                        <div class="form-group">
                            <label for="sort-by">{"Sort by:"}</label>
                            <select id="sort-by" value={self.filters.sort_by.clone()} onchange={on_sort_change}>
                                <option value="updated">{"Recently updated"}</option>
                                <option value="stars">{"Most stars"}</option>
                                <option value="forks">{"Most forks"}</option>
                                <option value="created">{"Recently created"}</option>
                            </select>
                        </div>

                        <div class="form-group">
                            <label for="per-page">{"Results per page:"}</label>
                            <input
                                id="per-page"
                                type="number"
                                min="10"
                                max="100"
                                step="10"
                                value={self.filters.per_page.to_string()}
                                oninput={on_per_page_change}
                            />
                        </div>
                    </div>

                    <button class="btn-primary" onclick={on_apply}>
                        {"Apply Filters & Update URL"}
                    </button>
                </div>

                <div class="search-info">
                    <h3>{"Current URL Parameters:"}</h3>
                    <div class="code-block">
                        <code>{format!("/search{}", self.filters.to_query_string())}</code>
                    </div>

                    <h3>{"How it works:"}</h3>
                    <ul>
                        <li>{"Form values are synchronized with URL query parameters"}</li>
                        <li>{"Changes to form fields update the component state"}</li>
                        <li>{"Clicking 'Apply Filters' updates the browser URL"}</li>
                        <li>{"URL parameters are parsed when the component loads"}</li>
                        <li>{"Users can bookmark or share URLs with specific search criteria"}</li>
                        <li>{"Browser back/forward buttons work correctly"}</li>
                    </ul>
                </div>

                <div class="search-results">
                    <h3>{"Search Results"}</h3>
                    <p>{"In a real application, this would show filtered results based on the current parameters:"}</p>
                    <div class="mock-results">
                        <div class="result-item">
                            <strong>{"Query: "}</strong> {&self.filters.query}
                        </div>
                        <div class="result-item">
                            <strong>{"Type: "}</strong> {&self.filters.repo_type}
                        </div>
                        <div class="result-item">
                            <strong>{"Language: "}</strong> {&self.filters.language}
                        </div>
                        <div class="result-item">
                            <strong>{"Sort: "}</strong> {&self.filters.sort_by}
                        </div>
                        <div class="result-item">
                            <strong>{"Per page: "}</strong> {self.filters.per_page}
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn parse_query_params(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if query.is_empty() {
        return params;
    }

    // Remove leading '?' if present
    let query = query.strip_prefix('?').unwrap_or(query);

    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if let Ok(decoded_value) = urlencoding::decode(value) {
                params.insert(key.to_string(), decoded_value.to_string());
            }
        }
    }

    params
}
