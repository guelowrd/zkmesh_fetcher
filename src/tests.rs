use chrono::NaiveDate;
use mockito::mock;
use crate::{read_blogs_from_file, fetch_substack_blog_articles, fetch_rss_blog_articles, parse_rss_date};

#[test]
fn test_read_blogs_from_file() {
    let content = "TestBlog|https://test.com|Substack\nAnotherBlog|https://another.com|RSS";
    std::fs::write("test_blogs.txt", content).unwrap();
    
    let blogs = read_blogs_from_file("test_blogs.txt").unwrap();
    
    assert_eq!(blogs.len(), 2);
    assert_eq!(blogs[0].name, "TestBlog");
    assert_eq!(blogs[0].domain, "https://test.com");
    assert_eq!(blogs[0].platform, "Substack");
    
    std::fs::remove_file("test_blogs.txt").unwrap();
}

#[test]
fn test_fetch_substack_blog_articles() {
    // Define the mock JSON response
    let mock_body = r#"[
        {
            "id": 1,
            "title": "Test Article",
            "slug": "test-article",
            "post_date": "2024-10-01T00:00:00.000Z"
        }
    ]"#;

    // Set up the mock server
    let _m = mock("GET", "/api/v1/posts?limit=50")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_body)
        .create();

    // Set a date for filtering articles
    let since_date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();

    // Call the function under test with the mock server URL
    let mock_url = mockito::server_url();
    let api_url = format!("{}/api/v1/posts?limit=50", mock_url);
    let articles = fetch_substack_blog_articles(&api_url, &since_date, "TestBlog", "https://test.com").unwrap();

    // Assert the results
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Test Article");
    assert_eq!(articles[0].url, "https://test.com/p/test-article");
    assert_eq!(articles[0].date, NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
}

#[test]
fn test_fetch_rss_blog_articles() {
    // Define the mock RSS response
    let mock_response = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <rss version="2.0">
        <channel>
            <item>
                <title>Test RSS Article</title>
                <link>https://test.com/rss-article</link>
                <pubDate>Tue, 01 Oct 2024 12:00:00 GMT</pubDate>
            </item>
        </channel>
    </rss>
    "#;

    // Set up the mock server
    let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/rss+xml")
        .with_body(mock_response)
        .create();

    // Set a date for filtering articles
    let since_date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();

    // Call the function under test with the mock server URL
    let articles = fetch_rss_blog_articles(&mockito::server_url(), &since_date, "TestRSSBlog").unwrap();

    // Assert the results
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Test RSS Article");
    assert_eq!(articles[0].url, "https://test.com/rss-article");
    assert_eq!(articles[0].date, NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
}

#[test]
fn test_parse_rss_date() {
    let date1 = "Tue, 01 Oct 2024 12:00:00 GMT";
    let date2 = "2024-10-01T12:00:00+00:00";
    let date3 = "2024-10-01";
    
    assert_eq!(parse_rss_date(date1).unwrap(), NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
    assert_eq!(parse_rss_date(date2).unwrap(), NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
    assert_eq!(parse_rss_date(date3).unwrap(), NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
    
    assert!(parse_rss_date("Invalid Date").is_err());
}