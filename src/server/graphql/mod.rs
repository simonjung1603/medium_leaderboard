use serde::{Deserialize, Serialize};

pub mod clap_count_query;
pub mod story_details_query;

pub const GRAPHQL_ENDPOINT: &str = "https://medium.com/_/graphql";

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlRequest<'a, V: Serialize> {
    operation_name: &'a str,
    query: &'a str,
    variables: V,
}

#[derive(Deserialize, Debug)]
pub struct GraphQlResponse<T> {
    pub(crate) data: PostResult<T>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostResult<T> {
    pub(crate) post_result: T,
}
