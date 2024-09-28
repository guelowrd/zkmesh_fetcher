#[cfg(test)]
mod tests;

use chrono::NaiveDate;
use std::env;
use std::fs::File;
use std::io::Write;
use tokio;

mod feed_types;
mod errors;
mod utils;
mod config;
mod models;

use feed_types::{FeedType, ArticleFetcher, SubstackFetcher, RssFetcher, AtomFetcher, CustomHtmlFetcher, EprintFetcher};
use errors::AppError;

fn capitalize_title(title: &str) -> String {
    let words = title.split_whitespace().collect::<Vec<&str>>();
    let mut capitalized_title = Vec::new();

    for (i, &word) in words.iter().enumerate() {
        let is_first_or_last = i == 0 || i == words.len() - 1;
        let is_preposition_or_conjunction = matches!(word.to_lowercase().as_str(), 
            "and" | "but" | "or" | "for" | "nor" | "so" | "yet" | "to" | "the" | "a" | "an");

        // Check if the word should be left untouched
        let is_untouched = word.starts_with('(') || 
                           (word.chars().all(|c| c.is_uppercase()) && word.len() > 1) || 
                           word.chars().filter(|c| c.is_uppercase()).count() >= 3;

        // Determine if the previous word ends with a colon
        let capitalize_next = if i > 0 && words[i - 1].ends_with(':') {
            true
        } else {
            false
        };

        // Special behavior for "ZKsync" and "AggLayer"
        let capitalized_word = if word.eq_ignore_ascii_case("zksync") {
            "ZKsync".to_string() // Always capitalize as "ZKsync"
        } else if word.eq_ignore_ascii_case("agglayer") {
            "AggLayer".to_string() // Always capitalize as "AggLayer"
        } else if word.eq_ignore_ascii_case("agglayer’s") {
            "AggLayer’s".to_string() // Always capitalize as "AggLayer"
        } else if is_untouched {
            // Leave the word untouched
            word.to_string()
        } else if capitalize_next || is_first_or_last || word.len() > 3 || !is_preposition_or_conjunction {
            // Capitalize the first letter and lowercase the rest
            let mut c = word.to_lowercase();
            c.get_mut(0..1).map(|s| s.make_ascii_uppercase());
            c
        } else {
            // Lowercase the word
            word.to_lowercase()
        };

        capitalized_title.push(capitalized_word);
    }

    capitalized_title.join(" ")
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
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
            fetcher.fetch_articles(&blog.domain, &since_date, &blog.name).await
        });
        tasks.push(task);
    }

    let mut eprint_articles = Vec::new();
    let mut other_articles = Vec::new();

    for task in tasks {
        let articles = task.await??;
        for article in articles {
            if article.blog_name == "Eprint" {
                eprint_articles.push(article);
            } else {
                other_articles.push(article);
            }
        }
    }

    // Sort both lists by date in ascending order
    eprint_articles.sort_by(|a, b| a.date.cmp(&b.date));
    other_articles.sort_by(|a, b| a.date.cmp(&b.date));

    // Create HTML output
    let mut html_output = String::from("<html><body>");

    // Add header for Eprint papers
    if !eprint_articles.is_empty() {
        html_output.push_str("<h2>Eprint Papers</h2><ul>");
        for article in eprint_articles {
            let authors_or_blog_name = article.authors.clone().unwrap_or_else(|| "Unknown Author".to_string());
            let capitalized_title = capitalize_title(&article.title); // Capitalize the title
            html_output.push_str(&format!("<li><a href=\"{}\">{}</a> | {}</li>", article.url, capitalized_title, authors_or_blog_name));
        }
        html_output.push_str("</ul>");
    }

    // Add header for Blog articles
    if !other_articles.is_empty() {
        html_output.push_str("<h2>Blog Articles</h2><ul>");
        for article in other_articles {
            let authors_or_blog_name = article.blog_name.clone();
            let capitalized_title = capitalize_title(&article.title); // Capitalize the title
            html_output.push_str(&format!("<li><a href=\"{}\">{}</a> | {}</li>", article.url, capitalized_title, authors_or_blog_name));
        }
        html_output.push_str("</ul>");
    }

    html_output.push_str("</body></html>");

    // Write the HTML output to a file
    let mut file = File::create("output.html")?;
    file.write_all(html_output.as_bytes())?;

    Ok(())
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