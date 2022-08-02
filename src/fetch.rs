use crate::CommonError;
use anyhow::{anyhow, Result};
use gloo_net::http::Request;
use graphql_client::QueryBody;
use graphql_client::{GraphQLQuery, Response as GraphQlResponse};
use serde::Serialize;
use yew::{Component, Context};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/server_issues.graphql",
    response_derives = "Clone, PartialEq, Debug"
)]
struct ServerIssues;

pub type Issues = (i64, String);

fn request<V>(query: &QueryBody<V>) -> Result<Request>
where
    V: Serialize,
{
    let body = serde_json::to_value(&query)
        .map_err(|e| anyhow!("cannot create a GraphQL query: {}", e))?;
    let request = Request::post("/graphql")
        .header("Content-Type", "application/json")
        .json(&body)?;
    Ok(request)
}

pub trait QueryIssue: Component + Common {
    fn success_issues_info(issues: Vec<Issues>) -> Self::Message;
    fn fetch_iussue_info(&mut self, ctx: &Context<Self>) {
        let variables = server_issues::Variables {};

        let response = move |res: GraphQlResponse<server_issues::ResponseData>| {
            if let Some(val) = res.data {
                let mut vec_list: Vec<Issues> = Vec::new();
                for item in val.issues {
                    vec_list.push((item.number, item.title));
                }
                Self::success_issues_info(vec_list)
            } else {
                Self::common_error(CommonError::ResponseParseError)
            }
        };

        self.send_qeury::<ServerIssues, _>(ctx, variables, response);
    }
}

pub trait Common: Component {
    fn common_error(error: CommonError) -> Self::Message;

    fn send_qeury<G, F>(&self, ctx: &Context<Self>, var: G::Variables, f: F)
    where
        G: GraphQLQuery,
        F: 'static + FnOnce(GraphQlResponse<G::ResponseData>) -> Self::Message,
    {
        if let Ok(req) = request(&G::build_query(var)) {
            ctx.link().send_future(async move {
                if let Ok(res) = req.send().await {
                    if res.ok() {
                        if let Ok(res) = res.json::<GraphQlResponse<G::ResponseData>>().await {
                            f(res)
                        } else {
                            Self::common_error(CommonError::GraphQLParseError)
                        }
                    } else {
                        Self::common_error(CommonError::HttpStatusNoSuccess)
                    }
                } else {
                    Self::common_error(CommonError::GraphQLResponseError)
                }
            });
        } else {
            ctx.link()
                .send_message(Self::common_error(CommonError::SendGraphQLQueryError));
        };
    }
}
