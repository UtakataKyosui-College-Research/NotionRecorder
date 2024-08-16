use std::{collections::HashMap};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum ContentId {
    #[serde(rename = "database_id")]
    DatabaseId,
    #[serde(rename = "page_id")]
    PageId
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct DatabaseParent {
    #[serde(rename = "type")]
    pub  r#type: ContentId,
    /* #[serde(rename = "page_id")]
    pub PageId: Option<String>, */
    #[serde(rename = "database_id")]
    pub database_id: Option<String>
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Text {
    pub content: String,
    pub link: Option<String>
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Title {
    #[serde(rename = "type")]
    pub r#type: String,
    pub text: Text
}

#[derive(Debug,Deserialize,Serialize)]
pub struct ItemDate {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>
}

#[derive(Debug,Deserialize,Serialize)]
pub enum Property {
    #[serde(rename = "title")]
    NotionTitle(Vec<Title>),
    #[serde(rename = "date")]
    NotionDate(ItemDate)
}

#[derive(Debug,Deserialize,Serialize)]
pub struct PostPage {
    pub parent: DatabaseParent,// ContentId,
    pub properties: HashMap<String,Property>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Filter {
    pub filter: TextFilter //LogicalOperators
}


#[derive(Debug,Serialize,Deserialize)]
pub enum LogicalOperators {
    #[serde(rename = "and")]
    And(TextFilter),
    #[serde(rename = "or")]
    Or(TextFilter)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct TextFilter {
    pub property: String,
    pub rich_text: FilterOption
}


#[derive(Debug,Serialize,Deserialize)]
pub enum FilterOption {
    #[serde(rename = "contains")]
    Contains(String)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ListResponse {
    pub object: String,
    pub results: Vec<PageObject>,
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct PageObject {
    pub object: String,
    pub id: String,
    pub created_time: DateTime<Utc>,
    pub last_edited_time: DateTime<Utc>,
    pub parent: DatabaseParent,
    /* pub properties: HashMap<String,HashMap<String,String>> */
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UpdateObject {
    pub properties: HashMap<String,Property>
}