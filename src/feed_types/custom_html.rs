use super::ArticleFetcher;
use crate::models::BlogArticle;
use crate::errors::AppError;
use chrono::NaiveDate;
use async_trait::async_trait;
use scraper::{Html, Selector};

pub struct CustomHtmlFetcher {
    pub article_selector: String,
    pub article_item_selector: String, 
    pub title_selector: String,
    pub url_selector: String,
    pub date_selector: String,
    pub date_format: String,
}

#[async_trait]
impl ArticleFetcher for CustomHtmlFetcher {
    async fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str) -> Result<Vec<BlogArticle>, AppError> {
        // println!("Fetching articles from: {}", feed_url);
        let content = reqwest::get(feed_url).await?.text().await?;
        // println!("Received HTML content: {} characters", content.len());

        // println!("Full HTML content:\n{}", content);

        let document = Html::parse_document(&content);

        let article_selector = Selector::parse(&self.article_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid article selector: {:?}", e)))?;
        // println!("Article selector: {}", self.article_selector);

        let title_selector = Selector::parse(&self.title_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid title selector: {:?}", e)))?;
        // println!("Title selector: {}", self.title_selector);

        let url_selector = Selector::parse(&self.url_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid URL selector: {:?}", e)))?;
        // println!("URL selector: {}", self.url_selector);

        let date_selector = Selector::parse(&self.date_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid date selector: {:?}", e)))?;
        // println!("Date selector: {}", self.date_selector);

        let article_item_selector = Selector::parse(&self.article_item_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid article item selector: {:?}", e)))?;
        // println!("Article item selector: {}", self.article_item_selector);

        let mut blog_articles = Vec::new();

        let article_wrapper = document.select(&article_selector).next()
            .ok_or_else(|| AppError::ParseError("No article wrapper found".to_string()))?;

        let article_elements = article_wrapper.select(&article_item_selector).collect::<Vec<_>>();
        // println!("Found {} articles", article_elements.len());

        //for (i, article) in article_elements.iter().enumerate() {
        for (_i, article) in article_elements.iter().enumerate() {
        // println!("Processing article {}", i + 1);

            let title = article.select(&title_selector).next()
                .ok_or_else(|| AppError::ParseError("Missing title".to_string()))?
                .text().collect::<String>();
            // println!("Title: {}", title);

            let url = article.select(&url_selector).next()
                .ok_or_else(|| AppError::ParseError("Missing URL".to_string()))?
                .value().attr("href")
                .ok_or_else(|| AppError::ParseError("Missing href attribute".to_string()))?
                .to_string();

            // Ensure the URL is correctly formatted
            let url = if url.starts_with("http") {
                url // It's already a full URL
            } else {
                // Remove any leading slashes from the relative URL
                let trimmed_url = url.trim_start_matches('/');
                
                // Remove common subfolder from feed_url
                let base_url = feed_url.trim_end_matches('/');
                let common_subfolder = if base_url.ends_with("blog") {
                    "blog"
                } else if base_url.ends_with("posts") {
                    "posts"
                } else {
                    "" // No common subfolder
                };

                // Construct the final URL
                if !common_subfolder.is_empty() {
                    format!("{}/{}", base_url.trim_end_matches(&format!("/{}", common_subfolder)), trimmed_url)
                } else {
                    format!("{}/{}", base_url, trimmed_url)
                }
            };
            // println!("URL: {}", url);

            let date_str = article.select(&date_selector).next()
                .ok_or_else(|| AppError::ParseError("Missing date".to_string()))?
                .text().collect::<String>();
            // println!("Date string: {}", date_str);

            let date = NaiveDate::parse_from_str(&date_str, &self.date_format)?;
            // println!("Parsed date: {}", date);

            if date >= *since_date {
                blog_articles.push(BlogArticle {
                    title,
                    url,
                    date,
                    blog_name: blog_name.to_string(),
                    authors: None, // Set authors to None for Custom HTML
                });
                // println!("Article added to the list");
            } else {
                // println!("Article skipped (older than since_date)");
            }
        }

        // println!("Total articles found: {}", blog_articles.len());
        Ok(blog_articles)
    }
}