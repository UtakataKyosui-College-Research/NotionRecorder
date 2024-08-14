use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use uuid::{fmt::Urn, Uuid};
use std::str::FromStr;
use notion::{
    ids::{DatabaseId ,PageId, PropertyId}, 
    models::{
        properties::{Color, DateValue, PropertyValue,DateOrDateTime},
         text::{RichText, RichTextCommon, Text},
         Page, PageCreateRequest, Properties, Utc,
         
    },
    NotionApi
};
use std::collections::HashMap;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct Config {
  notion_token: String,
  database_id: DatabaseId// String,
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
    Check,
}

#[tokio::main]
async fn main() {
    let args = Recs::parse();
    dotenv().expect("dotenv Error");
    match envy::from_env::<Config>() {
        Ok(config) => {
            println!("{:?}",config);
            let client = NotionApi::new(config.notion_token)
                .expect("Notion Client Error");
            match args.command {
                Subs::Start { title } => {
                    // 開始の打刻をする
                    let mut properties : HashMap<String,PropertyValue>= HashMap::new();

                    properties.insert(String::from("名前"), PropertyValue::Title { 
                        id: PropertyId::from_str("example").expect("Title Property Id Error"),
                        title: vec![
                            RichText::Text {
                                rich_text: RichTextCommon {
                                    plain_text: title.clone(),
                                    href: None,
                                    annotations: None
                                },
                                text: Text {
                                    content: title.clone(),
                                    link: None
                                }
                            }
                        ]
                    });

                    properties.insert(String::from("開始時刻"), PropertyValue::Date { 
                        id: PropertyId::from_str(String::from("test").as_str()).unwrap(),
                        date: Some(DateValue {
                            start: DateOrDateTime::DateTime(Utc::now()),
                            end: None,
                            time_zone: Some(String::from("Asia/Tokyo"))
                        })
                    });



                    let page_request = PageCreateRequest {
                        parent: notion::models::Parent::Database { database_id: (config.database_id) },
                        properties: Properties {
                            properties
                        }
                    };
                    let result = client.create_page(page_request).await
                        .expect("Page Create Expect");
                    println!("Create Success Page Id: {}",result.id)
                },
                Subs::End { title } => {
                    // 終了の打刻をする
                    let mut properties : HashMap<String,PropertyValue>= HashMap::new();

                    properties.insert(String::from("名前"), PropertyValue::Title { 
                        id: PropertyId::from_str("example").expect("Title Property Id Error"),
                        title: vec![
                            RichText::Text {
                                rich_text: RichTextCommon {
                                    plain_text: title.clone(),
                                    href: None,
                                    annotations: None
                                },
                                text: Text {
                                    content: title.clone(),
                                    link: None
                                }
                            }
                        ]
                    });

                    properties.insert(String::from("終了時刻"), PropertyValue::Date { 
                        id: PropertyId::from_str(String::from("test").as_str()).unwrap(),
                        date: Some(DateValue {
                            start: DateOrDateTime::DateTime(Utc::now()),
                            end: None,
                            time_zone: Some(String::from("Asia/Tokyo"))
                        })
                    });



                    let page_request = PageCreateRequest {
                        parent: notion::models::Parent::Database { database_id: (config.database_id) },
                        properties: Properties {
                            properties
                        }
                    };
                    let result = client.create_page(page_request).await
                        .expect("Page Create Expect");
                    println!("Create Success Page Id: {}",result.id)
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
