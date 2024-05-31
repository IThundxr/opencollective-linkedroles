use std::collections::HashMap;
use std::env;
use std::ptr::null;
use reqwest::header::HeaderMap;
use rocket::yansi::Paint;
use serde::Deserialize;
use serde_json::json;
use crate::utils;

pub struct Response {
    state: String,
    oc: OpenCollectiveUser,
    metadata: OpenCollectiveMetadata
}

#[derive(Deserialize)]
struct OpenCollectiveMeQueryResult {
    data: OpenCollectiveMeData
}

#[derive(Deserialize)]
struct OpenCollectiveMeData {
    me: OpenCollectiveUser
}

#[derive(Deserialize)]
struct OpenCollectiveUser {
    id: String,
    name: String,
    slug: String
}

#[derive(Deserialize)]
struct OpenCollectiveMetadata {
    total_donated: f64,
    last_donation: String,
    last_donation_amount: f64,
    is_backer: bool,
}

#[derive(Deserialize)]
struct CodeToTokenResponse {
    access_token: String
}

pub async fn get_data(code: String) -> Response {
    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", &env::var("OPEN_COLLECTIVE_CLIENT_ID").expect("Missing Open Collective Client ID")),
        ("client_secret", &env::var("OPEN_COLLECTIVE_CLIENT_SECRET").expect("Missing Open Collective Client Secret")),
        ("code", &code),
        ("redirect_uri", &utils::base_url("open-collective/redirect")),
    ];

    let client = reqwest::Client::new();
    let token = client.post("https://opencollective.com/oauth/token")
        .form(&params)
        .send()
        .await.unwrap()
        .json::<CodeToTokenResponse>()
        .await.unwrap()
        .access_token;

    let user = client.post("https://opencollective.com/api/graphql/v2")
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({"query": "{ me { id slug name } }"}))
        .send()
        .await.unwrap()
        .json::<OpenCollectiveMeQueryResult>()
        .await.unwrap();
    
    let user_metadata_graphql = "fragment AccountParts on Account {
          memberOf (account: {slug: $slug}, limit: 1) {
            nodes {
              totalDonations {
                value
              }
              role
            }
          }
          transactions(fromAccount: {slug: $slug}, limit: 1, type: DEBIT) {
            nodes {
              account { id name slug }
              netAmountInHostCurrency {
                value
              }
              createdAt
            }
          }
        }
        query metadata($slug: String, $account_id: String) {
          account (id: $account_id) {
            ...AccountParts
            organizations: memberOf (accountType: [COLLECTIVE, ORGANIZATION], limit: 1) {
              nodes {
                account {
                  ...AccountParts
                }
              }
            }
          }
        }".replace("$slug", &env::var("OPEN_COLLECTIVE_SLUG").expect("Missing Open Collective Slug")).replace("$account_id", &user.data.me.id);

    let user_metadata = client.post("https://opencollective.com/api/graphql/v2")
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({"query": user_metadata_graphql}))
        .send()
        .await.unwrap()
        .json::<OpenCollectiveMetadata>()
        .await.unwrap();

    return Response {
        state: "".to_string(),
        oc: user.data.me,
        metadata: user_metadata,
    }
}