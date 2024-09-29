use crate::feed_types::{ArticleFetcher, SubstackFetcher};
use chrono::NaiveDate;
use mockito::mock;
use tokio;

#[tokio::test]
async fn test_fetch_substack_blog_articles() {
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
    let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_body)
        .create();

    // Set a date for filtering articles
    let since_date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();

    // Create a SubstackFetcher instance
    let fetcher = SubstackFetcher;

    // Call the function under test with the mock server URL
    let mock_url = mockito::server_url();
    let articles = fetcher.fetch_articles(&mock_url, &since_date, "TestBlog", None)
        .await
        .expect("Failed to fetch Substack articles");

    // Assert the results
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Test Article");
    assert_eq!(articles[0].url, format!("{}/p/test-article", mock_url));
    assert_eq!(articles[0].date, NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
}