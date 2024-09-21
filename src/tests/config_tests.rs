use crate::config::read_blogs_from_file;
use crate::feed_types::FeedType;
use std::fs::File;
use tempfile::NamedTempFile;
use serde_json;

#[test]
fn test_read_blogs_from_file() {
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let path = temp_file.path().to_str().expect("Failed to get path as string");
    
    let file = File::create(path).expect("Failed to create file");
    let json_content = serde_json::json!([
        {
            "name": "TestBlog",
            "domain": "https://test.com",
            "feed_type": "Substack"
        },
        {
            "name": "AnotherBlog",
            "domain": "https://another.com",
            "feed_type": "RSS"
        },
        {
            "name": "CustomBlog",
            "domain": "https://custom.com",
            "feed_type": "CustomHTML",
            "custom_selectors": {
                "article_selector": ".article",
                "article_item_selector": ".article-item",
                "title_selector": ".title",
                "url_selector": ".url",
                "date_selector": ".date",
                "date_format": "%Y-%m-%d"
            }
        }
    ]);
    serde_json::to_writer_pretty(file, &json_content).expect("Failed to write JSON to file");
    
    let blogs = read_blogs_from_file(path).unwrap();
    
    assert_eq!(blogs.len(), 3);
    assert_eq!(blogs[0].name, "TestBlog");
    assert_eq!(blogs[0].domain, "https://test.com");
    assert_eq!(blogs[0].feed_type, FeedType::Substack);
    assert_eq!(blogs[1].name, "AnotherBlog");
    assert_eq!(blogs[1].domain, "https://another.com");
    assert_eq!(blogs[1].feed_type, FeedType::RSS);
    assert_eq!(blogs[2].name, "CustomBlog");
    assert_eq!(blogs[2].domain, "https://custom.com");
    assert_eq!(blogs[2].feed_type, FeedType::CustomHTML);
    assert!(blogs[2].custom_selectors.is_some());
    let custom_selectors = blogs[2].custom_selectors.as_ref().unwrap();
    assert_eq!(custom_selectors.article_selector, ".article");
    assert_eq!(custom_selectors.article_item_selector, ".article-item");
    assert_eq!(custom_selectors.title_selector, ".title");
    assert_eq!(custom_selectors.url_selector, ".url");
    assert_eq!(custom_selectors.date_selector, ".date");
    assert_eq!(custom_selectors.date_format, "%Y-%m-%d");
}