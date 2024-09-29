use super::ArticleFetcher;
use crate::models::BlogArticle;
use crate::errors::AppError;
use chrono::NaiveDate;
use async_trait::async_trait;
use scraper::{Html, Selector};
use crate::utils::replace_url;

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
    async fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, blog_name: &str, custom_url_replace: Option<String>) -> Result<Vec<BlogArticle>, AppError> {
        let content = reqwest::get(feed_url).await?.text().await?;
        let document = Html::parse_document(&content);

        let article_selector = Selector::parse(&self.article_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid article selector: {:?}", e)))?;
        let title_selector = Selector::parse(&self.title_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid title selector: {:?}", e)))?;
        let url_selector = Selector::parse(&self.url_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid URL selector: {:?}", e)))?;
        let date_selector = Selector::parse(&self.date_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid date selector: {:?}", e)))?;
        let article_item_selector = Selector::parse(&self.article_item_selector)
            .map_err(|e| AppError::ParseError(format!("Invalid article item selector: {:?}", e)))?;

        let mut blog_articles = Vec::new();

        let article_wrapper = document.select(&article_selector).next()
            .ok_or_else(|| AppError::ParseError("No article wrapper found".to_string()))?;

        let article_elements = article_wrapper.select(&article_item_selector).collect::<Vec<_>>();

        for article in article_elements.iter() {
            let title = article.select(&title_selector).next()
                .ok_or_else(|| AppError::ParseError("Missing title".to_string()))?
                .text().collect::<String>();

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

            let final_url = replace_url(&url, custom_url_replace.as_ref());

            let date_str = article.select(&date_selector).next()
                .ok_or_else(|| AppError::ParseError("Missing date".to_string()))?
                .text().collect::<String>();

            let date = NaiveDate::parse_from_str(&date_str, &self.date_format)?;

            if date >= *since_date {
                blog_articles.push(BlogArticle {
                    title,
                    url: final_url, // Use the final URL after replacement
                    date,
                    blog_name: blog_name.to_string(),
                    authors: None, // Set authors to None for Custom HTML
                });
            }
        }

        Ok(blog_articles)
    }
}