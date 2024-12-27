use crate::server::graphql::{GraphQlRequest, GraphQlResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClapCountQuery<'a> {
    pub(crate) post_id: &'a str,
    pub(crate) include_first_boosted_at: bool,
}

pub type ClapCountResult = Vec<GraphQlResponse<ClapCountResponse>>;

impl<'a> From<ClapCountQuery<'a>> for GraphQlRequest<'a, ClapCountQuery<'a>> {
    fn from(variables: ClapCountQuery<'a>) -> GraphQlRequest<'a, ClapCountQuery<'a>> {
        Self {
            operation_name: "ClapCountQuery",
            query: CLAP_COUNT_QUERY,
            variables,
        }
    }
}

const CLAP_COUNT_QUERY: &str = "query ClapCountQuery($postId: ID!, $includeFirstBoostedAt: Boolean!) {\n  postResult(id: $postId) {\n    __typename\n    ... on Post {\n      id\n      clapCount\n      firstBoostedAt @include(if: $includeFirstBoostedAt)\n      __typename\n    }\n  }\n}\n";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClapCountResponse {
    pub(crate) clap_count: i32,
}
