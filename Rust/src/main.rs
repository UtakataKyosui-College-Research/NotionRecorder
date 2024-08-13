use clap::{Parser,Subcommand};
use notion::{chrono::Date, ids::{self, DatabaseId, PageId}, models::{properties::PropertyValue, text::RichText, Database, Page, PageCreateRequest, Parent, Properties, Utc}, NotionApi};
use std::collections::HashMap;

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


fn add_work(client: &NotionApi,database_id: &str){
    /* client.create_page(page: Page {
        archived: false,
        created_time: Utc::now(),
        parent: Parent::Database { database_id: (database_id) },
        properties: Properties { properties: () },
        id: PageId::as_id(&self),
        last_edited_time:: Utc::now(),
    }) */
   client.create_page(
    
   )
}
 
fn client_reset() -> NotionApi{
    let client = NotionApi::new("token".to_string()).expect("API Token Error");
    return client;
}

fn main() {
    let client = client_reset();
    let database_id = "database_id";

    
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
