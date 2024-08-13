use clap::{Parser, Subcommand};
use notion::{ids::DatabaseId, NotionApi};
use std::collections::HashMap;
use serde::Deserialize;


#[derive(Deserialize,Debug)]
struct Env {
    notion_token: String,
    database_id: DatabaseId
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
    match envy::from_env::<Env>(){
        Ok(env) => {
            let client = NotionApi::new(env.notion_token)
                .expect("Notion Client Error");
        },
        Err(error) => {
            panic!("{:#?}",error)
        }
    }
    let args = Recs::parse();

    match args.command {
        Subs::Start { title } => {
            let mut page_props:HashMap<String,PropertyValue> = HashMap::new();
        },
        Subs::End { title } => {},
        Subs::Check => {},
        Subs::Times => {}
    }

    // println!("{}",args.title);
}
