use crate::feed_types::{ArticleFetcher, AtomFetcher};
use chrono::NaiveDate;
use mockito::mock;

#[test]
fn test_fetch_atom_blog_articles() {
    // Test similar to the RSS and Substack tests
    // but using AtomFetcher and appropriate mock data
    let mock_response = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <feed xmlns="http://www.w3.org/2005/Atom">
        <entry>
            <title>Test Atom Article</title>
            <link href="https://test.com/atom-article"/>
            <updated>2024-10-01T12:00:00Z</updated>
        </entry>
    </feed>
    "#;

    let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/atom+xml")
        .with_body(mock_response)
        .create();

    let since_date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();
    let fetcher = AtomFetcher;
    let articles = fetcher.fetch_articles(&mockito::server_url(), &since_date, "TestAtomBlog").unwrap();

    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Test Atom Article");
    assert_eq!(articles[0].url, "https://test.com/atom-article");
    assert_eq!(articles[0].date, NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
}