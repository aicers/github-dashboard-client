use stylist::{css, yew::styled_component};
use yew::prelude::*;

use super::{stats_query, Tab};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub active_tab: Tab,
    pub data: stats_query::ResponseData,
}

/// A presentational component that renders the statistics data.
#[styled_component(StatsDisplay)]
pub fn stats_display(props: &Props) -> Html {
    let stat_grid_style = css!(
        r#"
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 20px;
        text-align: center;
    "#
    );
    let stat_card_style = css!(
        r#"
        background-color: #f6f8fa;
        border: 1px solid #d1d5da;
        border-radius: 6px;
        padding: 20px;

        & > h4 { font-size: 14px; font-weight: 600; color: #586069; margin: 0 0 8px 0; }
        & > p { font-size: 28px; font-weight: 600; color: #24292e; margin: 0; }
    "#
    );
    let table_style = css!(
        r#"
        width: 100%;
        border-collapse: collapse;
        margin-top: 20px;
        & th, & td { border: 1px solid #d1d5da; padding: 12px; text-align: left; }
        & th { background-color: #f6f8fa; font-weight: 600; }
        & tr:nth-child(even) { background-color: #f6f8fa; }
    "#
    );

    match &props.active_tab {
        Tab::Discussion => html! {
            <div>
                <h2>{ "Discussion Statistics" }</h2>
                <div class={stat_grid_style}>
                    <div class={stat_card_style.clone()}>
                        <h4>{ "Total Discussions" }</h4>
                        <p>{ props.data.discussion_stat.total_count }</p>
                    </div>
                    <div class={stat_card_style}>
                        <h4>{ "Total Comments" }</h4>
                        <p>{ props.data.discussion_stat.comment_count }</p>
                    </div>
                </div>
            </div>
        },
        Tab::Issue => html! {
            <div>
                <h2>{ "Issue Statistics" }</h2>
                <div class={stat_grid_style}>
                    <div class={stat_card_style.clone()}>
                        <h4>{ "Open Issues" }</h4>
                        <p>{ props.data.issue_stat.open_issue_count }</p>
                    </div>
                    <div class={stat_card_style}>
                        <h4>{ "Resolved Issues" }</h4>
                        <p>{ props.data.issue_stat.resolved_issue_count }</p>
                    </div>
                </div>
                <h3 style="margin-top: 30px;">{ "Resolved Issue Size Distribution" }</h3>
                <table class={table_style}>
                    <thead> <tr><th>{ "Size" }</th><th>{ "Count" }</th></tr> </thead>
                    <tbody>
                        { for props.data.issue_stat.resolved_issue_size_distribution.iter().map(|dist| html! {
                            <tr><td>{ &dist.size }</td><td>{ dist.count }</td></tr>
                        }) }
                    </tbody>
                </table>
            </div>
        },
        Tab::PullRequest => html! {
            <div>
                <h2>{ "Pull Request Statistics" }</h2>
                <div class={stat_grid_style}>
                    <div class={stat_card_style.clone()}>
                        <h4>{ "Open PRs" }</h4>
                        <p>{ props.data.pull_request_stat.open_pr_count }</p>
                    </div>
                    <div class={stat_card_style.clone()}>
                        <h4>{ "Merged PRs" }</h4>
                        <p>{ props.data.pull_request_stat.merged_pr_count }</p>
                    </div>
                    <div class={stat_card_style.clone()}>
                        <h4>{ "Average Review Comments" }</h4>
                        <p>{ props.data.pull_request_stat.avg_review_comment_count.map_or("N/A".to_string(), |avg| format!("{avg:.2}")) }</p>
                    </div>
                    <div class={stat_card_style}>
                        <h4>{ "Average Merge Time (Days)" }</h4>
                        <p>{ props.data.pull_request_stat.avg_merge_days.map_or("N/A".to_string(), |days| format!("{days:.2}")) }</p>
                    </div>
                </div>
            </div>
        },
    }
}
