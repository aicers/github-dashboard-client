use graphql_client::{GraphQLQuery, Response};

use super::{stats_query, FilterInputs};

/// Asynchronously fetches statistics data from the GraphQL server.
pub async fn fetch_stats(
    variables: stats_query::Variables,
) -> Result<stats_query::ResponseData, String> {
    let request_body = super::StatsQuery::build_query(variables);
    let client = reqwest::Client::new();

    let res = client
        .post("http://localhost:8080/graphql")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_body: Response<stats_query::ResponseData> =
        res.json().await.map_err(|e| e.to_string())?;

    if let Some(data) = response_body.data {
        Ok(data)
    } else {
        Err(response_body
            .errors
            .map_or("Unknown error".to_string(), |e| e[0].message.clone()))
    }
}

/// A helper function to convert user filter inputs (`FilterInputs`) into GraphQL query variables.
/// It transforms empty strings into `None` and formats datetime values to meet API requirements.
pub fn build_query_variables(filters: &FilterInputs) -> stats_query::Variables {
    let to_option = |s: &str| {
        if s.is_empty() {
            None
        } else {
            Some(s.to_string())
        }
    };

    let format_datetime_local = |s: &str| {
        if s.is_empty() {
            None
        } else {
            Some(format!("{s}:00Z"))
        }
    };

    let repo_opt = to_option(&filters.repo);
    let author_opt = to_option(&filters.author);
    let assignee_opt = to_option(&filters.assignee);
    let begin_opt = format_datetime_local(&filters.begin);
    let end_opt = format_datetime_local(&filters.end);

    stats_query::Variables {
        issue_filter: stats_query::IssueStatFilter {
            repo: repo_opt.clone(),
            author: author_opt.clone(),
            assignee: assignee_opt,
            begin: begin_opt.clone(),
            end: end_opt.clone(),
        },
        pr_filter: stats_query::PullRequestStatFilter {
            repo: repo_opt.clone(),
            author: author_opt.clone(),
            begin: begin_opt.clone(),
            end: end_opt.clone(),
        },
        discussion_filter: stats_query::DiscussionStatFilter {
            repo: repo_opt,
            author: author_opt,
            begin: begin_opt,
            end: end_opt,
        },
    }
}
