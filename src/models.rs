use crate::feed_types::FeedType;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct BlogInfo {
    pub name: String,
    pub domain: String,
    pub feed_type: FeedType,
}

#[derive(Debug)]
pub struct BlogArticle {
    pub title: String,
    pub url: String,
    pub date: NaiveDate,
    pub blog_name: String,
}