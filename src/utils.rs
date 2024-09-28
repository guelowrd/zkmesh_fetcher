use chrono::{NaiveDate, Datelike};
use std::env;
use std::fs::File;
use std::io::Write;
use crate::errors::AppError;

pub fn parse_rss_date(date_str: &str) -> Result<NaiveDate, AppError> {
    let formats = [
        "%a, %d %b %Y %H:%M:%S %Z",
        "%a, %d %b %Y %H:%M:%S GMT",
        "%Y-%m-%dT%H:%M:%S%:z",
        "%Y-%m-%d",
        "%Y-%m-%dT%H:%M:%SZ",  
    ];

    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Ok(date);
        }
    }

    Err(AppError::ParseError(format!("Unable to parse date: {}", date_str)))
}

pub fn capitalize_title(title: &str) -> String {
    let words = title.split_whitespace().collect::<Vec<&str>>();
    let mut capitalized_title = Vec::new();

    for (i, &word) in words.iter().enumerate() {
        let is_first_or_last = i == 0 || i == words.len() - 1;
        let is_preposition_or_conjunction = matches!(word.to_lowercase().as_str(), 
            "and" | "but" | "or" | "of" | "for" | "nor" | "so" | "yet" | "to" | "the" | "at" | "a" | "an");

        let is_untouched = word.starts_with('(') || 
                           (word.chars().all(|c| c.is_uppercase()) && word.len() > 1) || 
                           word.chars().filter(|c| c.is_uppercase()).count() > 2 ||
                           (word.chars().filter(|c| c.is_uppercase()).count() > 1 && word.len() > 2);

        let capitalize_next = if i > 0 && words[i - 1].ends_with(':') {
            true
        } else {
            false
        };

        let capitalized_word = if word.eq_ignore_ascii_case("zksync") {
            "ZKsync".to_string()
        } else if is_untouched {
            word.to_string()
        } else if capitalize_next || is_first_or_last || word.len() > 3 || !is_preposition_or_conjunction {
            let mut c = word.to_lowercase();
            c.get_mut(0..1).map(|s| s.make_ascii_uppercase());
            c
        } else {
            word.to_lowercase()
        };

        capitalized_title.push(capitalized_word);
    }

    capitalized_title.join(" ")
}

pub fn parse_args() -> Result<(String, NaiveDate), AppError> {
    let args: Vec<String> = env::args().collect();
    let blogs_file = if args.len() > 1 {
        args[1].clone()
    } else {
        "./config/blogs.json".to_string()
    };

    let since_date = if args.len() > 2 {
        NaiveDate::parse_from_str(&args[2], "%Y-%m-%d")?
    } else {
        let today = chrono::Local::now();
        NaiveDate::from_ymd_opt(today.year(), today.month(), 1).expect("Invalid date provided")
    };

    Ok((blogs_file, since_date))
}

pub fn write_output(html_output: &str) -> Result<(), AppError> {
    std::fs::create_dir_all("./output")?;
    let mut file = File::create("./output/index.html")?;
    file.write_all(html_output.as_bytes())?;
    Ok(())
}