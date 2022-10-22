use anyhow::*;
use reqwest::Client;

use crate::graphql::organization::{get_organization_data, organization_query, OrganizationQuery};

mod graphql;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let contents =
    //     fs::read_to_string("placeholder.svg").expect("Should have been able to read the file");

    // let modified = contents
    //     .replace("%TITLE%", "CONTRIBUTOR OF OCTOBER 2022")
    //     .replace("%PRS%", "3000")
    //     .replace("%ISSUES%", "100")
    //     .replace("%PRREVIEWS%", "20");

    // let mut file = File::create("output.svg").expect("Error creating modified file");

    // file.write(modified.as_bytes()).expect("WTF");

    let github_api_token = std::env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN env var");

    let variables = organization_query::Variables {
        organization: "novuhq".to_string(),
    };

    let client = Client::builder()
        .user_agent("graphql-rust/0.11.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_api_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let response_data = get_organization_data(variables, client).await;

    let name: Option<String> = response_data.organization.unwrap().name;

    println!("{:?}", name);

    Ok(())
}
