#[cfg(test)]
mod tests;

use chrono::NaiveDate;
use std::env;
use tokio;

mod feed_types;
mod errors;
mod utils;

use feed_types::{FeedType, ArticleFetcher, SubstackFetcher, RssFetcher, AtomFetcher};
use errors::AppError;

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

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    let blogs_file = args.get(1).ok_or_else(|| AppError::ParseError("Missing blogs file argument".to_string()))?;
    let since_date_str = args.get(2).ok_or_else(|| AppError::ParseError("Missing since date argument".to_string()))?;

    let blogs = utils::read_blogs_from_file(blogs_file)?;
    let since_date = NaiveDate::parse_from_str(since_date_str, "%Y-%m-%d")?;

    let mut tasks = Vec::new();

    for blog in blogs {
        let fetcher: Box<dyn ArticleFetcher> = match blog.feed_type {
            FeedType::Substack => Box::new(SubstackFetcher),
            FeedType::RSS => Box::new(RssFetcher),
            FeedType::Atom => Box::new(AtomFetcher),
        };

        let task = tokio::spawn(async move {
            fetcher.fetch_articles(&blog.domain, &since_date, &blog.name).await
        });
        tasks.push(task);
    }

    for task in tasks {
        let articles = task.await??;
        for article in articles {
            println!("[{}]({}) | {} ({})", article.title, article.url, article.blog_name, article.date);
        }
    }

    Ok(())
}

#[cfg(test)]
pub async fn run_with_args(args: Vec<String>) -> Result<(), AppError> {
    let blogs_file = args.get(1).ok_or_else(|| AppError::ParseError("Missing blogs file argument".to_string()))?;
    let since_date_str = args.get(2).ok_or_else(|| AppError::ParseError("Missing since date argument".to_string()))?;

    let blogs = utils::read_blogs_from_file(blogs_file)?;
    let since_date = NaiveDate::parse_from_str(since_date_str, "%Y-%m-%d")?;

    let mut tasks = Vec::new();

    for blog in blogs {
        let fetcher: Box<dyn ArticleFetcher> = match blog.feed_type {
            FeedType::Substack => Box::new(SubstackFetcher),
            FeedType::RSS => Box::new(RssFetcher),
            FeedType::Atom => Box::new(AtomFetcher),
        };

        let task = tokio::spawn(async move {
            fetcher.fetch_articles(&blog.domain, &since_date, &blog.name).await
        });
        tasks.push(task);
    }

    for task in tasks {
        let articles = task.await??;
        for article in articles {
            println!("[{}]({}) | {} ({})", article.title, article.url, article.blog_name, article.date);
        }
    }

    Ok(())
}