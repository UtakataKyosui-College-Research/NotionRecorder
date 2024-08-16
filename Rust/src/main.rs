use chrono::Utc;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use tokio;
use dotenvy::dotenv;
use model::{DatabaseParent, Filter, ItemDate, ListResponse, LogicalOperators, PostPage, Property, Text, TextFilter, Title, UpdateObject};
use reqwest::{header::{self, HeaderMap, HeaderValue}, Client, ClientBuilder};
use std::collections::HashMap;
mod model;


pub async fn authorize(notion_token: &String) -> Client {
    let mut authorize_headers:HeaderMap = HeaderMap::new();
    let notion_api_version: &str = "2022-02-22";
    authorize_headers.insert("Notion-Version", HeaderValue::from_static(&notion_api_version));
    authorize_headers.insert(
        header::CONTENT_TYPE, 
        HeaderValue::from_static("application/json")
    );
    authorize_headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}",notion_token)).expect("Notion Token Not Found"));
    let client_builder:ClientBuilder = Client::builder();
    let notion_client = client_builder
        .default_headers(authorize_headers)
        .build()
        .expect("Client Build Error");
    return notion_client
}

#[derive(Deserialize, Debug)]
struct Config {
  notion_token: String,
  database_id:  String
}


#[derive(Debug,Parser)]
#[clap(
    name = "RecNote",
    version = "0.9",
    author = "Utakata Kyosui",
    about = "Support Work Memo Note Edit",
    arg_required_else_help = true
)]
struct Recs {
    #[clap(subcommand)]
    command: Subs
}

#[derive(Debug,Subcommand)]
enum Subs {
    #[clap(arg_required_else_help = true,about = "Start Work Command")]
    Start {
        #[clap(
            short = 't',
            long = "title",
            required = true,
            ignore_case = true
        )]
        title: String
    },
    #[clap(arg_required_else_help = true,about = "Ended Work Command")]
    End {
        #[clap(
            short = 't',
            long = "title",
            required = true,
            ignore_case = true,
        )]
        title: String
    },
    #[clap(arg_required_else_help = true,about = "Work Times All Records")]
    Times,
    #[clap(arg_required_else_help = true,about = "Today Works Exists Check")]
    Check
}

#[tokio::main]
async fn main() {
    let args = Recs::parse();
    dotenv().expect("dotenv Error");
    match envy::from_env::<Config>() {
        Ok(config) => {
            let notion_client = authorize(&config.notion_token).await;
            let notion_api_url: &str = "https://api.notion.com/v1";
            match args.command {
                Subs::Start { title } => {
                    let mut properties: HashMap<String,Property> = HashMap::new();
                    properties.insert(String::from("名前"), Property::NotionTitle(
                        vec![Title{
                            r#type: String::from("text"),
                            text: Text {
                                content: format!("{} -{}-",title,Utc::today()),
                                link: None
                            }
                        }]
                    ));
                    properties.insert(String::from("開始時刻"), Property::NotionDate(
                        ItemDate {
                            start: Utc::now(),
                            end: None
                        }
                    ));

                    let post_page: PostPage = PostPage {
                        parent: DatabaseParent {
                            r#type: model::ContentId::DatabaseId,
                            database_id: Some(String::from(config.database_id))
                        },
                        properties
                    };
                    let json = serde_json::to_string(&post_page).unwrap();

                    let result = notion_client.post(format!("{}/{}",notion_api_url,"pages"))
                        .body(json)
                        .send()
                        .await
                        .unwrap();

                    let text = result.text().await.expect("Fetch Error");
                },
                Subs::End { title } => {
                    let search_url = format!("{}/{}/{}/{}",&notion_api_url,"databases",&config.database_id,"query");
                    // println!("{}",&search_url);
                    let filter = Filter {
                        filter: TextFilter{
                            property: String::from("名前"),
                            rich_text: model::FilterOption::Contains(format!("{} -{}-",title,Utc::today()))
                        }
                    };
                    let result = notion_client.post(search_url)
                        .body(serde_json::to_string(&filter).unwrap())
                        .send()
                        .await
                        .expect("Get Error");

                    let result_json = serde_json::from_str::<ListResponse>(&result.text().await.unwrap())
                        .unwrap();
                    let first_page_id = result_json.
                        results
                        .first()
                        .unwrap()
                        .clone()
                        .id;
                    
                    let mut update_json: HashMap<String,Property> = HashMap::new();
                    update_json.insert(String::from("終了時刻"), Property::NotionDate(
                        ItemDate { start: Utc::now(), end: None }
                    ));
                    let update_property = UpdateObject {
                        properties: update_json
                    };
                    let update_result = notion_client
                        .patch(format!("{}/{}/{}",&notion_api_url,"pages",first_page_id))
                        .body(serde_json::to_string(&update_property).unwrap())
                        .send()
                        .await
                        .expect("Update Error");
                    
                    
                    print!("{}",update_result.text().await.expect("Update Error"));
                },
                Subs::Check => {
                    // 今日の活動の有無を確認する
                },
                Subs::Times => {
                    // これまでの活動時間の長さを確認する
                }
            }
        },
        Err(e) => println!("{:?}",e)
    }
    
}
