use notion::{ids::DatabaseId, models::{Database, Parent}, NotionApi};
use clap::Parser;
use serde::{Deserialize,Serialize};


fn add_work(client: &NotionApi){
    let page_content = notion::models::PageCreateRequest {
        parent: Parent {
            Database {
                database_id: String::from("")
            },
        },
        properties {

        }
    }
    client.create_page(page)
}

fn client_reset() -> Result<NotionApi,()>{
    let client = NotionApi::new("token".to_string()).expect("API Token Error");
    Ok(client)
}

/* #[derive(Parser,Debug)]
#[clap(version = "1.0",author = "Utakata Kyosui")]
struct Args {
    #[clap(subcommand)]
    command: 
} 
 */

fn main() {
    let client = NotionApi::new("token".to_string()).expect("API Token Error");

    println!("Hello, world!");
}
