use super::ArticleFetcher;
use crate::models::BlogArticle;
use crate::errors::AppError;
use chrono::NaiveDate;
use async_trait::async_trait;
use reqwest::Client;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::name::QName;  
use std::str;  

#[derive(Debug, Clone)]
struct Record {
    datestamp: String,
    identifier: String,
    title: String,
    creators: Vec<String>,
    dates: Vec<String>,
    description: String,
}

pub struct EprintFetcher;

impl EprintFetcher {
    fn format_creators(mut creators: Vec<String>) -> String {
        match creators.len() {
            1 => creators[0].clone(),
            2 => format!("{} and {}", creators[0], creators[1]),
            _ => {
                let last = if let Some(creator) = creators.pop() {
                    creator // Remove the last creator safely
                } else {
                    return String::new(); // Return an empty string if there are no creators
                };
                format!("{} and {}", creators.join(", "), last) // Join the rest with commas and add the last with "and"
            }
        }
    }
}

fn should_include_record(record: &Record) -> bool {
    // Keywords to check in the description
    let keywords = ["zero-knowledge", "zero knowledge", "zk", "snark", "stark"];
    let authors_to_check = ["dan boneh", "alessandro chiesa"];

    // Check if the description contains any of the keywords
    let description_contains_keyword = keywords.iter().any(|&keyword| {
        record.description.to_lowercase().contains(keyword)
    });

    // Check if any of the authors match
    let authors_match = record.creators.iter().any(|author| {
        authors_to_check.iter().any(|&name| author.to_lowercase() == name)
    });

    description_contains_keyword || authors_match
}

#[async_trait]
impl ArticleFetcher for EprintFetcher {
    async fn fetch_articles(&self, feed_url: &str, since_date: &NaiveDate, _blog_name: &str) -> Result<Vec<BlogArticle>, AppError> {
        let client = Client::new();
        let response = client.get(feed_url).send().await?;
        let xml: String = response.text().await?;

        let mut reader = Reader::from_str(&xml);

        let mut current_element = String::new();
        let mut records: Vec<Record> = Vec::new();
        let mut record: Record = Record {
            datestamp: String::new(),
            identifier: String::new(),
            title: String::new(),
            creators: Vec::new(),
            dates: Vec::new(),
            description: String::new(),
        };

        while let Ok(event) = reader.read_event() {
            match event {
                Event::Start(ref e) => {
                    current_element = str::from_utf8(e.name().as_ref())
                        .expect("Failed to convert element name to string") // Handle potential UTF-8 conversion error
                        .to_string(); 
                }
                Event::Text(e) => {
                    let text = e.unescape()
                        .expect("Failed to unescape XML text"); // Handle potential unescape error
                    if !text.trim().is_empty() { // Only assign if text is not empty
                        match current_element.as_str() {
                            "dc:identifier" => record.identifier = text.trim().to_string(),
                            "dc:title" => record.title = text.trim().to_string(),
                            "dc:creator" => record.creators.push(text.trim().to_string()),
                            "dc:date" => record.dates.push(text.trim().to_string()), 
                            "dc:description" => record.description = text.trim().to_string(), 
                            "datestamp" => record.datestamp = text.trim().to_string(), 
                            _ => {}
                        }
                    }
                }
                Event::End(ref e) => {
                    if e.name() == QName(b"record") {
                        records.push(record.clone()); // Ensure we clone the record
                        record = Record {
                            datestamp: String::new(),
                            identifier: String::new(),
                            title: String::new(),
                            creators: Vec::new(),
                            dates: Vec::new(),
                            description: String::new(),
                        };
                    }
                }
                Event::Eof => break,  // End of file reached, break the loop
                _ => {}
            }
        }
        
        // Convert records to BlogArticle
        let mut articles = Vec::new();
        for record in records {
            // Check if the dates vector is not empty before accessing it
            if !record.dates.is_empty() {
                if let Ok(date) = NaiveDate::parse_from_str(&record.dates[0], "%Y-%m-%dT%H:%M:%SZ") {
                    if date >= *since_date && should_include_record(&record) {
                        let creators = record.creators.clone();
                        let authors = if creators.is_empty() {
                            None 
                        } else {
                            Some(EprintFetcher::format_creators(creators)) 
                        };
                        articles.push(BlogArticle {
                            title: record.title,
                            url: record.identifier,
                            date,
                            blog_name: "Eprint".to_string(), 
                            authors: authors, 
                        });
                    }
                }
            }
        }

        Ok(articles)
    }
}