use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use uuid::Uuid;
use std::str::FromStr;
use notion::{
    ids::{DatabaseId ,PageId, PropertyId}, 
    models::{
        properties::PropertyValue,
        Page,
        Properties,
        Utc,        
        text::{Text,RichText},
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
                    let uuid = Uuid::new_v4().to_string().as_str();
                    let properties : HashMap<String,PropertyValue>= HashMap::new();
                    properties.insert(String::from("名前"), PropertyValue::Title { 
                        id: PropertyId::from_str("id").expect("Title Property Error") ,
                        title: vec![
                            RichText {
                               text: {
                                    
                               }
                            }
                        ]
                    });

                    let page: Page = Page {
                        parent: notion::models::Parent::Database { database_id: (config.database_id) },
                        created_time: Utc::now(),
                        last_edited_time: Utc::now(),
                        id: PageId::from_str(uuid).unwrap(),
                        archived: false,
                        properties: Properties {

                        }
                    }
                    client.create_page(page)
                },
                Subs::End { title } => {
                    // 終了の打刻をする
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
