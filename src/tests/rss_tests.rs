use crate::feed_types::{ArticleFetcher, RssFetcher};
use chrono::NaiveDate;
use mockito::mock;
use tokio;

#[tokio::test]
async fn test_fetch_rss_blog_articles() {
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

    // Create an RssFetcher instance
    let fetcher = RssFetcher;

    // Call the function under test with the mock server URL
    let articles = fetcher.fetch_articles(&mockito::server_url(), &since_date, "TestRSSBlog", None)
        .await
        .expect("Failed to fetch RSS articles");

    // Assert the results
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Test RSS Article");
    assert_eq!(articles[0].url, "https://test.com/rss-article");
    assert_eq!(articles[0].date, NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
}