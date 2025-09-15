// Define and import sub-modules
mod api;
mod filter_bar;
mod stats_display;

// Use the public items from our sub-modules
use api::{build_query_variables, fetch_stats};
use filter_bar::FilterBar;
use graphql_client::GraphQLQuery;
use stats_display::StatsDisplay;
use stylist::{css, yew::styled_component};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

// GraphQL macro and domain-specific structs remain in the main module file.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/stats_query.graphql",
    response_derives = "Clone, PartialEq, Debug, Default",
    variables_derives = "Clone, Debug, Default, PartialEq"
)]
struct StatsQuery;

#[derive(Clone, PartialEq, Debug)]
pub enum Tab {
    Discussion,
    Issue,
    PullRequest,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FilterInputs {
    pub repo: String,
    pub author: String,
    pub assignee: String,
    pub begin: String,
    pub end: String,
}

/// The main component for the statistics page, now acting as a "container" component.
/// It manages state and composes child components.
#[styled_component(StatsPage)]
pub fn stats_page() -> Html {
    let active_tab = use_state(|| Tab::Issue);
    let filter_inputs = use_state(FilterInputs::default);
    let query_variables = use_state(stats_query::Variables::default);

    let state = {
        let vars = (*query_variables).clone();
        use_async(async move { fetch_stats(vars).await })
    };

    use_effect_with((*query_variables).clone(), {
        let state = state.clone();
        move |_| {
            state.run();
            || ()
        }
    });

    // --- Callbacks remain in the container component as they modify the state here. ---
    let on_repo_change = {
        let filter_inputs = filter_inputs.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_filters = (*filter_inputs).clone();
            new_filters.repo = e.target_unchecked_into::<HtmlInputElement>().value();
            filter_inputs.set(new_filters);
        })
    };
    let on_author_change = {
        let filter_inputs = filter_inputs.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_filters = (*filter_inputs).clone();
            new_filters.author = e.target_unchecked_into::<HtmlInputElement>().value();
            filter_inputs.set(new_filters);
        })
    };
    let on_assignee_change = {
        let filter_inputs = filter_inputs.clone();
        Callback::from(move |e: InputEvent| {
            let mut new_filters = (*filter_inputs).clone();
            new_filters.assignee = e.target_unchecked_into::<HtmlInputElement>().value();
            filter_inputs.set(new_filters);
        })
    };
    let on_begin_change = {
        let filter_inputs = filter_inputs.clone();
        Callback::from(move |e: Event| {
            let mut new_filters = (*filter_inputs).clone();
            new_filters.begin = e.target_unchecked_into::<HtmlInputElement>().value();
            filter_inputs.set(new_filters);
        })
    };
    let on_end_change = {
        let filter_inputs = filter_inputs.clone();
        Callback::from(move |e: Event| {
            let mut new_filters = (*filter_inputs).clone();
            new_filters.end = e.target_unchecked_into::<HtmlInputElement>().value();
            filter_inputs.set(new_filters);
        })
    };
    let on_apply_filters = {
        let filter_inputs = filter_inputs.clone();
        let query_variables = query_variables.clone();
        Callback::from(move |_| {
            let new_vars = build_query_variables(&filter_inputs);
            query_variables.set(new_vars);
        })
    };
    let get_tab_click_handler = |tab: Tab| {
        let active_tab = active_tab.clone();
        Callback::from(move |_| {
            active_tab.set(tab.clone());
        })
    };

    // --- Styles for the main layout ---
    let tab_button_style = css!(
        r#"
        flex-grow: 1;
        padding: 15px 20px;
        border: none;
        background-color: transparent;
        cursor: pointer;
        font-size: 16px;
        font-weight: 600;
        color: #586069;
        transition: color 0.2s;
        border-bottom: 3px solid transparent;
        &:hover { color: #0366d6; }
    "#
    );
    let active_tab_style = css!("color: #24292e; border-bottom-color: #0366d6;");

    html! {
        <div class={css!(r#"
            max-width: 1200px;
            margin: 40px auto;
            background-color: #ffffff;
            border: 1px solid #d1d5da;
            border-radius: 8px;
            box-shadow: 0 4px 12px rgba(27, 31, 35, 0.05);
            overflow: hidden;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
        "#)}>
            // Tab Menu Area
            <div class={css!("display: flex; border-bottom: 1px solid #d1d5da; background-color: #f6f8fa;")}>
                <button class={classes!(tab_button_style.clone(), (*active_tab == Tab::Issue).then_some(active_tab_style.clone()))} onclick={get_tab_click_handler(Tab::Issue)}>
                    { "Issues" }
                </button>
                <button class={classes!(tab_button_style.clone(), (*active_tab == Tab::PullRequest).then_some(active_tab_style.clone()))} onclick={get_tab_click_handler(Tab::PullRequest)}>
                    { "Pull Requests" }
                </button>
                <button class={classes!(tab_button_style.clone(), (*active_tab == Tab::Discussion).then_some(active_tab_style))} onclick={get_tab_click_handler(Tab::Discussion)}>
                    { "Discussions" }
                </button>
            </div>

            // Render the FilterBar component, passing state and callbacks as props
            <FilterBar
                filters={(*filter_inputs).clone()}
                active_tab={(*active_tab).clone()}
                {on_repo_change}
                {on_author_change}
                {on_assignee_change}
                {on_begin_change}
                {on_end_change}
                on_apply={on_apply_filters}
            />

            // Content (Statistics Data) Area
            <div class={css!("padding: 30px;")}>
                {
                    if state.loading {
                        html! { <p>{ "Loading..." }</p> }
                    } else if let Some(data) = &state.data {
                        html!{ <StatsDisplay active_tab={(*active_tab).clone()} data={data.clone()} /> }
                    } else if let Some(error) = &state.error {
                        html! { <p class={css!("color: #cb2431; font-weight: 600;")}>{ format!("Error: {}", error) }</p> }
                    } else {
                        html! { <p>{ "Could not load data." }</p> }
                    }
                }
            </div>
        </div>
    }
}
