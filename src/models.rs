use crate::feed_types::FeedType;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogInfo {
    pub name: String,
    pub domain: String,
    pub feed_type: FeedType,
    pub custom_selectors: Option<CustomSelectors>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSelectors {
    pub article_selector: String,
    pub article_item_selector: String,
    pub title_selector: String,
    pub url_selector: String,
    pub date_selector: String,
    pub date_format: String,
}

#[derive(Debug, Clone)]
pub struct BlogArticle {
    pub title: String,
    pub url: String,
    pub date: NaiveDate,
    pub blog_name: String,
    pub authors: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EprintConfig {
    pub keywords: Vec<String>,
    pub authors: Vec<String>,
}