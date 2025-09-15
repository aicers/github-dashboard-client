use stylist::{css, yew::styled_component};
use yew::prelude::*;

use super::{FilterInputs, Tab};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub filters: FilterInputs,
    pub active_tab: Tab,
    pub on_repo_change: Callback<InputEvent>,
    pub on_author_change: Callback<InputEvent>,
    pub on_assignee_change: Callback<InputEvent>,
    pub on_begin_change: Callback<Event>,
    pub on_end_change: Callback<Event>,
    pub on_apply: Callback<MouseEvent>,
}

/// A component that renders the filter inputs and the apply button.
#[styled_component(FilterBar)]
pub fn filter_bar(props: &Props) -> Html {
    let input_style = css!(
        r#"
        padding: 8px 12px;
        font-size: 14px;
        border: 1px solid #d1d5da;
        border-radius: 6px;
        min-width: 150px;
        height: 38px;
        box-sizing: border-box;
    "#
    );

    let datetime_input_style = css!(
        r#"
        padding: 7px 12px;
        font-size: 14px;
        border: 1px solid #d1d5da;
        border-radius: 6px;
        font-family: inherit;
        height: 38px;
        box-sizing: border-box;
    "#
    );

    let apply_button_style = css!(
        r#"
        padding: 8px 16px;
        font-size: 14px;
        font-weight: 600;
        color: #fff;
        background-color: #28a745;
        border: 1px solid rgba(27, 31, 35, 0.15);
        border-radius: 6px;
        cursor: pointer;
        transition: background-color 0.2s;
        height: 38px;
        box-sizing: border-box;
        &:hover { background-color: #218838; }
    "#
    );

    html! {
        <div class={css!(r#"
            padding: 20px;
            display: flex;
            flex-direction: column;
            gap: 15px;
            border-bottom: 1px solid #d1d5da;
            align-items: center;
        "#)}>
            // First row: Date filters
            <div class={css!("display: flex; flex-wrap: wrap; gap: 15px; align-items: center;")}>
                <label class={css!("font-size: 14px; font-weight: 600;")}>{"Start"}</label>
                <input type="datetime-local" class={datetime_input_style.clone()} value={props.filters.begin.clone()} onchange={props.on_begin_change.clone()} />

                <label class={css!("font-size: 14px; font-weight: 600;")}>{"End"}</label>
                <input type="datetime-local" class={datetime_input_style} value={props.filters.end.clone()} onchange={props.on_end_change.clone()} />
            </div>

            // Second row: Other filters and button
            <div class={css!("display: flex; flex-wrap: wrap; gap: 15px; align-items: center;")}>
                <input type="text" placeholder="Repo (owner/repo)" class={input_style.clone()} value={props.filters.repo.clone()} oninput={props.on_repo_change.clone()} />
                <input type="text" placeholder="Author" class={input_style.clone()} value={props.filters.author.clone()} oninput={props.on_author_change.clone()} />

                { if props.active_tab == Tab::Issue {
                    html! { <input type="text" placeholder="Assignee" class={input_style} value={props.filters.assignee.clone()} oninput={props.on_assignee_change.clone()} /> }
                } else {
                    html! {}
                }}

                <button class={apply_button_style} onclick={props.on_apply.clone()}>
                    { "Apply" }
                </button>
            </div>
        </div>
    }
}
