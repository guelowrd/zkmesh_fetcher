use crate::{read_blogs_from_file, parse_rss_date, FeedType};
use chrono::NaiveDate;

#[test]
fn test_read_blogs_from_file() {
    let blogs = read_blogs_from_file("./src/tests/test_blogs.txt").unwrap();
    
    assert_eq!(blogs.len(), 2);
    assert_eq!(blogs[0].name, "TestBlog");
    assert_eq!(blogs[0].domain, "https://test.com");
    assert_eq!(blogs[0].feed_type, FeedType::Substack);
}

#[test]
fn test_parse_rss_date() {
    let date1 = "Tue, 01 Oct 2024 12:00:00 GMT";
    let date2 = "2024-10-01T12:00:00+00:00";
    let date3 = "2024-10-01";
    
    assert_eq!(
        parse_rss_date(date1).expect("Failed to parse date1"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );
    assert_eq!(
        parse_rss_date(date2).expect("Failed to parse date2"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );assert_eq!(
        parse_rss_date(date3).expect("Failed to parse date3"),
        NaiveDate::from_ymd_opt(2024, 10, 1).expect("Failed to create NaiveDate")
    );
    
    assert!(parse_rss_date("Invalid Date").is_err());
}

// #[test]
// fn test_main_functionality() {
//     // Create a temporary file with test data
//     let temp_file = "test_blogs.txt";
//     let content = "TestBlog|http://localhost:1234|Substack\nAnotherBlog|http://localhost:5678|RSS\n";
//     File::create(temp_file).unwrap().write_all(content.as_bytes()).unwrap();

//     // Mock the Substack API response
//     let _m1 = mock("GET", "/api/v1/posts/?limit=50")
//         .with_status(200)
//         .with_header("content-type", "application/json")
//         .with_body(r#"[{"title":"Test Article","slug":"test-article","post_date":"2024-10-01T00:00:00Z"}]"#)
//         .create();

//     // Mock the RSS feed response
//     let _m2 = mock("GET", "/")
//         .with_status(200)
//         .with_header("content-type", "application/rss+xml")
//         .with_body(r#"<?xml version="1.0" encoding="UTF-8"?>
//         <rss version="2.0">
//             <channel>
//                 <item>
//                     <title>Test RSS Article</title>
//                     <link>https://test.com/rss-article</link>
//                     <pubDate>Tue, 01 Oct 2024 12:00:00 GMT</pubDate>
//                 </item>
//             </channel>
//         </rss>"#)
//         .create();

//     // Run the main function
//     let blogs = read_blogs_from_file(temp_file).unwrap();
//     let since_date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();

//     for blog in blogs {
//         let fetcher: Box<dyn ArticleFetcher> = match blog.feed_type {
//             FeedType::Substack => Box::new(SubstackFetcher),
//             FeedType::RSS => Box::new(RssFetcher),
//             FeedType::Atom => Box::new(AtomFetcher),
//         };

//         let articles = fetcher.fetch_articles(&blog.domain, &since_date, &blog.name).unwrap();

//         assert!(!articles.is_empty());
//         for article in articles {
//             assert!(!article.title.is_empty());
//             assert!(!article.url.is_empty());
//             assert!(article.date >= since_date);
//         }
//     }

//     // Clean up
//     std::fs::remove_file(temp_file).unwrap();
// }