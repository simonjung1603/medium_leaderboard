use serde::Deserialize;
use serde::Serialize;

use crate::server::graphql::{GraphQlRequest, GraphQlResponse};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostPageQuery<'a> {
    pub(crate) post_id: &'a str,
}

pub type PostPageResult = Vec<GraphQlResponse<PostResponse>>;

impl<'a> From<PostPageQuery<'a>> for GraphQlRequest<'a, PostPageQuery<'a>> {
    fn from(variables: PostPageQuery<'a>) -> Self {
        Self {
            operation_name: "PostPageQuery",
            query: POST_PAGE_QUERY,
            variables,
        }
    }
}

const POST_PAGE_QUERY: &str = "query PostPageQuery($postId: ID!) {postResult(id: $postId) {__typename\n ... on Post {id\n creator {id\n name\n username\n __typename}\n mediumUrl\n latestPublishedVersion\n latestPublishedAt\n clapCount\n title\n previewImage{id\n __typename}\n tags{\n id\n __typename}\n wordCount\n __typename}}}";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    pub(crate) id: String,
    pub(crate) creator: CreatorResponse,
    #[allow(unused)]
    pub(crate) medium_url: String,
    pub(crate) latest_published_version: String,
    pub(crate) latest_published_at: i64,
    pub(crate) clap_count: i32,
    pub(crate) title: String,
    pub(crate) preview_image: PreviewImageResponse,
    pub(crate) word_count: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreatorResponse {
    #[allow(unused)]
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) username: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreviewImageResponse {
    pub(crate) id: String,
}