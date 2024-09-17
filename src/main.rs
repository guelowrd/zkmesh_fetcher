use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;

struct BlogArticle {
    title: String,
    url: String,
    date: NaiveDate,
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_url = "https://blog.gevulot.com/api/v1/posts/?limit=50";
    let since_date = NaiveDate::parse_from_str("2024-09-01", "%Y-%m-%d")?;

    let articles = fetch_gevulot_blog_articles(api_url, since_date)?;

    for article in articles {
        println!("[{}]({}) ({}) | Gevulot Blog", article.title, article.url, article.date);
    }

    Ok(())
}

fn fetch_gevulot_blog_articles(url: &str, since_date: NaiveDate) -> Result<Vec<BlogArticle>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let json: Value = response.json()?;

    let mut articles = Vec::new();

    if let Some(posts) = json.as_array() {
        for post in posts {
            let title = post["title"].as_str().unwrap_or_default().to_string();
            let slug = post["slug"].as_str().unwrap_or_default();
            let url = format!("https://blog.gevulot.com/p/{}", slug);
            let date_str = post["post_date"].as_str().unwrap_or_default();
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.fZ")?;

            if date >= since_date {
                articles.push(BlogArticle { title, url, date });
            }
        }
    }

    Ok(articles)
}