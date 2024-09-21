use crate::feed_types::{ArticleFetcher, CustomHtmlFetcher};
use chrono::NaiveDate;
use mockito::mock;
use tokio;

#[tokio::test]
async fn test_fetch_custom_html_articles() {
    let mock_response = r#"
    <html>
        <body>
            <div class="blog-list_wrapper w-dyn-list">
                <div class="blog-list_item-wrapper w-dyn-item">
                    <a href="/blog/test-article" class="blog-list_item w-inline-block">
                        <h3 class="blog-list_heading">Test Polygon Article</h3>
                        <div class="text-size-tiny text-style-label text-style-allcaps text-color-grey6">October 1, 2024</div>
                    </a>
                </div>
            </div>
        </body>
    </html>
    "#;

    let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "text/html")
        .with_body(mock_response)
        .create();

    let since_date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();
    let fetcher = CustomHtmlFetcher {
        article_selector: ".blog-list_wrapper.w-dyn-list".to_string(),
        article_item_selector: ".blog-list_item-wrapper.w-dyn-item".to_string(),
        title_selector: ".blog-list_heading".to_string(),
        url_selector: ".blog-list_item.w-inline-block".to_string(),
        date_selector: ".text-size-tiny.text-style-label.text-style-allcaps.text-color-grey6".to_string(),
        date_format: "%B %d, %Y".to_string()
    };

    let articles = fetcher.fetch_articles(&mockito::server_url(), &since_date, "TestPolygonBlog")
        .await
        .expect("Failed to fetch articles");

    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Test Polygon Article");
    assert_eq!(articles[0].url, format!("{}{}", &mockito::server_url(), "/blog/test-article"));
    assert_eq!(articles[0].date, NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
}