use crate::run_with_args;
use std::fs::File;
use tempfile::NamedTempFile;
use mockito::mock;
use tokio;
use serde_json;

#[tokio::test]
async fn test_main_function() {
    use mockito::mock;

    // Set up mock servers for different feed types
    let substack_mock = mock("GET", "/api/v1/posts/?limit=50")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"title":"Test Substack Article","slug":"test-article","post_date":"2024-10-01T00:00:00.000Z"}]"#)
        .create();

    let rss_mock = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/rss+xml")
        .with_body(r#"
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
        "#)
        .create();

    let atom_mock = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/atom+xml")
        .with_body(r#"
            <?xml version="1.0" encoding="utf-8"?>
            <feed xmlns="http://www.w3.org/2005/Atom">
                <entry>
                    <title>Test Atom Article</title>
                    <link href="https://test.com/atom-article"/>
                    <updated>2024-10-01T12:00:00Z</updated>
                </entry>
            </feed>
        "#)
        .create();

    let custom_html_mock = mock("GET", "/custom")
        .with_status(200)
        .with_header("content-type", "text/html")
        .with_body(r#"
            <html>
                <body>
                    <div class="post-feed">
                        <article>
                            <h2 class="post-card-title"><a href="https://test.com/custom-article">Test Custom HTML Article</a></h2>
                            <time class="post-card-meta-date">October 1, 2024</time>
                        </article>
                    </div>
                </body>
            </html>
        "#)
        .create();

    // Create a temporary file with test blog data
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let path = temp_file.path().to_str().expect("Failed to get path as string");

    let file = File::create(path).expect("Failed to create file");
    let json_content = serde_json::json!([
        {
            "name": "TestSubstack",
            "domain": format!("{}/api/v1/posts/?limit=50", mockito::server_url()),
            "feed_type": "Substack"
        },
        {
            "name": "TestRSS",
            "domain": mockito::server_url(),
            "feed_type": "RSS"
        },
        {
            "name": "TestAtom",
            "domain": mockito::server_url(),
            "feed_type": "Atom"
        },
        {
            "name": "TestCustomHTML",
            "domain": format!("{}/custom", mockito::server_url()),
            "feed_type": "CustomHTML",
            "custom_selectors": {
                "article_selector": "div.post-feed article",
                "title_selector": "h2.post-card-title a",
                "article_item_selector": "div.post-feed article",
                "url_selector": "h2.post-card-title a[href]",
                "date_selector": "time.post-card-meta-date",
                "date_format": "%B %d, %Y"
            }
        }
    ]);
    serde_json::to_writer_pretty(file, &json_content).expect("Failed to write JSON to file");

    // Run the main function with test arguments
    let args = vec![
        "program_name".to_string(),
        path.to_string(),
        "2024-09-01".to_string(),
    ];
    let result = run_with_args(args).await;
    assert!(result.is_ok());

    // Assert that the mocks were called
    substack_mock.assert();
    rss_mock.assert();
    atom_mock.assert();
    custom_html_mock.assert();
}

#[tokio::test]
async fn test_run_with_args() {
    // Set up mock servers for different feed types
    let substack_mock = mock("GET", "/api/v1/posts/?limit=50")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"title":"Test Substack Article","slug":"test-article","post_date":"2024-10-01T00:00:00.000Z"}]"#)
        .create();

    let rss_mock = mock("GET", "/rss")
        .with_status(200)
        .with_header("content-type", "application/rss+xml")
        .with_body(r#"
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
        "#)
        .create();

    let atom_mock = mock("GET", "/atom")
        .with_status(200)
        .with_header("content-type", "application/atom+xml")
        .with_body(r#"
            <?xml version="1.0" encoding="utf-8"?>
            <feed xmlns="http://www.w3.org/2005/Atom">
                <entry>
                    <title>Test Atom Article</title>
                    <link href="https://test.com/atom-article"/>
                    <updated>2024-10-01T12:00:00Z</updated>
                </entry>
            </feed>
        "#)
        .create();

    let custom_html_mock = mock("GET", "/custom")
        .with_status(200)
        .with_header("content-type", "text/html")
        .with_body(r#"
            <html>
                <body>
                    <div class="post-feed">
                        <article>
                            <h2 class="post-card-title"><a href="https://test.com/custom-article">Test Custom HTML Article</a></h2>
                            <time class="post-card-meta-date">October 1, 2024</time>
                        </article>
                    </div>
                </body>
            </html>
        "#)
        .create();

    // Create a temporary file with test blog data
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let path = temp_file.path().to_str().expect("Failed to get path as string");

    let file = File::create(path).expect("Failed to create file");
    let json_content = serde_json::json!([
        {
            "name": "TestSubstack",
            "domain": format!("{}/api/v1/posts/?limit=50", mockito::server_url()),
            "feed_type": "Substack"
        },
        {
            "name": "TestRSS",
            "domain": format!("{}/rss", mockito::server_url()),
            "feed_type": "RSS"
        },
        {
            "name": "TestAtom",
            "domain": format!("{}/atom", mockito::server_url()),
            "feed_type": "Atom"
        },
        {
            "name": "TestCustomHTML",
            "domain": format!("{}/custom", mockito::server_url()),
            "feed_type": "CustomHTML",
            "custom_selectors": {
                "article_selector": "div.post-feed article",
                "article_item_selector": "div.post-feed article",
                "title_selector": "h2.post-card-title a",
                "url_selector": "h2.post-card-title a[href]",
                "date_selector": "time.post-card-meta-date",
                "date_format": "%B %d, %Y"
            }
        }
    ]);
    serde_json::to_writer_pretty(file, &json_content).expect("Failed to write JSON to file");

    // Run the main function with test arguments
    let args = vec![
        "program_name".to_string(),
        path.to_string(),
        "2024-09-01".to_string(),
    ];
    let result = run_with_args(args).await;
    assert!(result.is_ok());

    // Assert that the mocks were called
    substack_mock.assert();
    rss_mock.assert();
    atom_mock.assert();
    custom_html_mock.assert();
}

#[tokio::test]
async fn test_run_with_args_invalid_date() {
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let path = temp_file.path().to_str().expect("Failed to get path as string");
    
    let file = File::create(path).expect("Failed to create file");
    let json_content = serde_json::json!([
        {
            "name": "TestBlog",
            "domain": "https://test.com",
            "feed_type": "Substack"
        }
    ]);
    serde_json::to_writer_pretty(file, &json_content).expect("Failed to write JSON to file");

    let args = vec![
        "program_name".to_string(),
        path.to_string(),
        "invalid_date".to_string(),
    ];
    let result = run_with_args(args).await;
    assert!(result.is_err());
}