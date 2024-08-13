use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use std::env;
use notion::{ids::{DatabaseId, Identifier}, NotionApi};
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
