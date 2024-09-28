mod substack;
mod rss;
mod atom;
mod custom_html;
mod eprint;

pub use substack::SubstackFetcher;
pub use rss::RssFetcher;
pub use atom::AtomFetcher;
pub use custom_html::CustomHtmlFetcher;
pub use eprint::EprintFetcher;

use chrono::NaiveDate;
use crate::models::BlogArticle;
use crate::errors::AppError;

use async_trait::async_trait;

#[async_trait]
pub trait ArticleFetcher: Send {
    async fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str) -> Result<Vec<BlogArticle>, AppError>;
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FeedType {
    Substack,
    RSS,
    Atom,
    CustomHTML,
    Eprint
}

impl std::str::FromStr for FeedType {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Substack" => Ok(FeedType::Substack),
            "RSS" => Ok(FeedType::RSS),
            "Atom" => Ok(FeedType::Atom),
            "CustomHTML" => Ok(FeedType::CustomHTML),
            "Eprint" => Ok(FeedType::Eprint),
            _ => Err(AppError::UnknownFeedType(s.to_string())),
        }
    }
}