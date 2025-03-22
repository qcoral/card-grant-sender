use serde::Deserialize;
use serde_json::json;
use std::env;
use reqwest::Error;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;

// get request seems to work pretty well
// now we just need to figure out the construction of the body for the card grant

// relevant ruby reference:
/*

    def create_card_grant(email:, amount_cents:, merchant_lock: nil, category_lock: nil, keyword_lock: nil)
      conn.post("organizations/#{@hcb_org_slug}/card_grants", email:, amount_cents:, category_lock:, merchant_lock:, keyword_lock:).body
    end

    relevant cURL command:

    curl -X POST "https://hcb.hackclub.com/api/v4/organizations/hackpad/card_grants" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "alexren@hackclub.com",
    "amount_cents": 200           
  }'
*/

#[tokio::main]
async fn main() -> Result<(), Error> {

    let auth_token = env::var("AUTH_TOKEN");

    let grant_body = json!({
        "email": "alexren@hackclub.com",
        "amount_cents": 300  
        });

    let request_url = format!("https://hcb.hackclub.com/api/v4/organizations/{org}/card_grants",
                              org = "hackpad");

    println!("{}", request_url);
    println!("token: {:?}", auth_token.clone().unwrap());
    
    let client = reqwest::Client::new();
    let res = client.post("https://hcb.hackclub.com/api/v4/organizations/hackpad/card_grants")
        .header(AUTHORIZATION, format!("Bearer {:?}",
                                    auth_token.unwrap()))
        .header(CONTENT_TYPE, "application/json")
        .json(&grant_body)
        .send()
        .await?;

    println!("{:?}", res);
    Ok(())
}
