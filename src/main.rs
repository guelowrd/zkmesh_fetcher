#[cfg(test)]
mod tests;
mod feed_types;
mod errors;
mod utils;
mod config;
mod models;

use chrono::NaiveDate;
use tokio;
use feed_types::{FeedType, ArticleFetcher, SubstackFetcher, RssFetcher, AtomFetcher, CustomHtmlFetcher, EprintFetcher};
use errors::AppError;
use crate::models::{BlogInfo, BlogArticle};
use crate::utils::{capitalize_title, parse_args, write_output};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let (blogs_file, since_date) = parse_args()?;
    let blogs = config::read_blogs_from_file(&blogs_file)?;
    let (eprint_articles, other_articles, errors) = fetch_articles(&blogs, &since_date).await?;
    let html_output = generate_html_output(eprint_articles, other_articles, errors, since_date, &blogs).await?; 
    write_output(&html_output)?;
    Ok(())
}

async fn fetch_articles(blogs: &[BlogInfo], since_date: &NaiveDate) -> Result<(Vec<BlogArticle>, Vec<BlogArticle>, Vec<(String, String)>), AppError> {
    let mut tasks = Vec::new();
    let mut errors = Vec::new();

    for blog in blogs {
        let fetcher: Box<dyn ArticleFetcher> = match blog.feed_type {
            FeedType::Substack => Box::new(SubstackFetcher),
            FeedType::RSS => Box::new(RssFetcher),
            FeedType::Atom => Box::new(AtomFetcher),
            FeedType::CustomHTML => {
                let custom_selectors = blog.custom_selectors.as_ref()
                    .ok_or_else(|| AppError::ParseError("Missing custom selectors for CustomHTML".to_string()))?;
                Box::new(CustomHtmlFetcher {
                    article_selector: custom_selectors.article_selector.clone(),
                    article_item_selector: custom_selectors.article_item_selector.clone(),
                    title_selector: custom_selectors.title_selector.clone(),
                    url_selector: custom_selectors.url_selector.clone(),
                    date_selector: custom_selectors.date_selector.clone(),
                    date_format: custom_selectors.date_format.clone(),
                })
            },
            FeedType::Eprint => Box::new(EprintFetcher),
        };

        let blog_clone = blog.clone();
        let since_date_clone = since_date.clone(); // Clone the since_date
        let custom_url_replace = blog.custom_url_replace.clone(); // Clone the custom_url_replace
        let task = tokio::spawn(async move {
            fetcher.fetch_articles(&blog_clone.domain, &since_date_clone, &blog_clone.name, custom_url_replace).await
        });
        tasks.push((task, blog.name.clone()));
    }

    let mut eprint_articles = Vec::new();
    let mut other_articles = Vec::new();

    for (task, blog_name) in tasks {
        match task.await {
            Ok(Ok(articles)) => {
                for article in articles {
                    if article.blog_name == "Eprint" {
                        eprint_articles.push(article);
                    } else {
                        other_articles.push(article);
                    }
                }
            }
            Ok(Err(e)) => {
                errors.push((blog_name, e.to_string()));
            }
            Err(e) => {
                errors.push((blog_name, e.to_string()));
            }
        }
    }

    // Sort articles by date
    eprint_articles.sort_by(|a, b| a.date.cmp(&b.date));
    other_articles.sort_by(|a, b| a.date.cmp(&b.date));

    Ok((eprint_articles, other_articles, errors))
}

async fn generate_html_output(
    eprint_articles: Vec<BlogArticle>, 
    other_articles: Vec<BlogArticle>, 
    errors: Vec<(String, String)>, 
    since_date: NaiveDate, 
    blogs: &[BlogInfo] 
) -> Result<String, AppError> {
    let mut html_output = String::from("<html><body>");

    // Add Eprint articles
    if !eprint_articles.is_empty() {
        html_output.push_str("<h2>Eprint Papers</h2><ul>");
        for article in eprint_articles {
            let authors_or_blog_name = article.authors.clone().unwrap_or_else(|| "Unknown Author".to_string());
            let capitalized_title = capitalize_title(&article.title);
            html_output.push_str(&format!("<li><a href=\"{}\">{}</a> | {}</li>", article.url, capitalized_title, authors_or_blog_name));
        }
        html_output.push_str("</ul>");
    }

    // Add other articles
    if !other_articles.is_empty() {
        html_output.push_str("<h2>Blog Articles</h2><ul>");
        for article in other_articles {
            let authors_or_blog_name = article.blog_name.clone();
            let capitalized_title = capitalize_title(&article.title);
            html_output.push_str(&format!("<li><a href=\"{}\">{}</a> | {}</li>", article.url, capitalized_title, authors_or_blog_name));
        }
        html_output.push_str("</ul>");
    }

    // Add fetching information
    html_output.push_str(&format!("<h2>Fetching Info</h2>"));
    html_output.push_str(&format!("<p>Date threshold: {}</p>", since_date));
    let run_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    html_output.push_str(&format!("<p>Generation date/time: {}</p>", run_date));

    // Add list of blogs
    html_output.push_str("<h3>List of Blogs:</h3><ul>");
    for blog in blogs { 
        html_output.push_str(&format!("<li><a href=\"{}\">{}</a></li>", blog.domain, blog.name));
    }
    html_output.push_str("</ul>");

    // Add errors if any
    if !errors.is_empty() {
        html_output.push_str("<h3>Errors:</h3><ul>");
        for (blog_name, error) in errors {
            html_output.push_str(&format!("<li><strong>{}</strong>: {}</li>", blog_name, error));
        }
        html_output.push_str("</ul>");
    }

    html_output.push_str("</body></html>");
    Ok(html_output)
}

#[cfg(test)]
pub async fn run_with_args(args: Vec<String>) -> Result<(), AppError> {
    let blogs_file = args.get(1).ok_or_else(|| AppError::ParseError("Missing blogs file argument".to_string()))?;
    let since_date_str = args.get(2).ok_or_else(|| AppError::ParseError("Missing since date argument".to_string()))?;

    let blogs = config::read_blogs_from_file(blogs_file)?;
    let since_date = NaiveDate::parse_from_str(since_date_str, "%Y-%m-%d")?;

    let mut tasks = Vec::new();

    for blog in blogs {
        let fetcher: Box<dyn ArticleFetcher> = match blog.feed_type {
            FeedType::Substack => Box::new(SubstackFetcher),
            FeedType::RSS => Box::new(RssFetcher),
            FeedType::Atom => Box::new(AtomFetcher),
            FeedType::CustomHTML => {
                let custom_selectors = blog.custom_selectors.as_ref()
                    .ok_or_else(|| AppError::ParseError("Missing custom selectors for CustomHTML".to_string()))?;
                Box::new(CustomHtmlFetcher {
                    article_selector: custom_selectors.article_selector.clone(),
                    article_item_selector: custom_selectors.article_item_selector.clone(),
                    title_selector: custom_selectors.title_selector.clone(),
                    url_selector: custom_selectors.url_selector.clone(),
                    date_selector: custom_selectors.date_selector.clone(),
                    date_format: custom_selectors.date_format.clone(),
                })
            },
            FeedType::Eprint => Box::new(EprintFetcher),
        };

        let task = tokio::spawn(async move {
            fetcher.fetch_articles(&blog.domain, &since_date, &blog.name, blog.custom_url_replace).await
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