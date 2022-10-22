use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use reqwest::Client;

use crate::graphql::{DateTime, URI};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query_1.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct OrganizationQuery;

pub async fn get_organization_data(
    variables: organization_query::Variables,
    client: Client,
) -> organization_query::ResponseData {
    let response_body =
        post_graphql::<OrganizationQuery, _>(&client, "https://api.github.com/graphql", variables)
            .await
            .unwrap();

    let response_data: organization_query::ResponseData =
        response_body.data.expect("missing response data");

    response_data
}
