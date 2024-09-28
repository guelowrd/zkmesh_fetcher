use crate::feed_types::{ArticleFetcher, EprintFetcher};
use chrono::NaiveDate;
use mockito::mock;
use tokio;

#[tokio::test]
async fn test_fetch_eprint_articles() {
    let mock_response = r#"
    <?xml version="1.0" encoding="UTF-8"?> 
    <?xml-stylesheet type="text/xsl" href="/css/oai2.xsl" ?>
    <OAI-PMH xmlns="http://www.openarchives.org/OAI/2.0/" 
             xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
             xsi:schemaLocation="http://www.openarchives.org/OAI/2.0/
             http://www.openarchives.org/OAI/2.0/OAI-PMH.xsd">
      <responseDate>2024-09-27T19:53:51Z</responseDate>
      
      <request verb="ListRecords" metadataPrefix="oai_dc">https://eprint.iacr.org/oai</request>
      <ListRecords>
        <record> 
          <header>
            <identifier>oai:eprint.iacr.org:2024/1431</identifier> 
            <datestamp>2024-09-18T07:46:25Z</datestamp>
          </header>
          <metadata>
            <oai_dc:dc 
                xmlns:oai_dc="http://www.openarchives.org/OAI/2.0/oai_dc/" 
                xmlns:dc="http://purl.org/dc/elements/1.1/" 
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" 
                xsi:schemaLocation="http://www.openarchives.org/OAI/2.0/oai_dc/ 
                                    http://www.openarchives.org/OAI/2.0/oai_dc.xsd">
              <dc:identifier>https://eprint.iacr.org/2024/1431</dc:identifier>
              <dc:title>Interactive Line-Point Zero-Knowledge with Sublinear Communication and Linear Computation</dc:title> 
              <dc:creator>Fuchun Lin</dc:creator>
              <dc:creator>Chaoping Xing</dc:creator>
              <dc:creator>Yizhou Yao</dc:creator>
              <dc:date>2024-09-13T02:36:04Z</dc:date>
              <dc:date>2024-09-14T02:36:04Z</dc:date>
              <dc:description>Studies of vector oblivious linear evaluation (VOLE)-based zero-knowledge (ZK) protocols...</dc:description> 
            </oai_dc:dc>
          </metadata>
        </record>
        <record> 
          <header>
            <identifier>oai:eprint.iacr.org:2024/9999</identifier> 
            <datestamp>2024-09-19T07:46:25Z</datestamp>
          </header>
          <metadata>
            <oai_dc:dc 
                xmlns:oai_dc="http://www.openarchives.org/OAI/2.0/oai_dc/" 
                xmlns:dc="http://purl.org/dc/elements/1.1/" 
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" 
                xsi:schemaLocation="http://www.openarchives.org/OAI/2.0/oai_dc/ 
                                    http://www.openarchives.org/OAI/2.0/oai_dc.xsd">
              <dc:identifier>https://eprint.iacr.org/2024/9999</dc:identifier>
              <dc:title>Another</dc:title> 
              <dc:creator>Author 1</dc:creator>
              <dc:creator>Author 2</dc:creator>
              <dc:date>2024-09-19T02:36:04Z</dc:date>
              <dc:description>Something STARK related</dc:description> 
            </oai_dc:dc>
          </metadata>
        </record>
      </ListRecords>
    </OAI-PMH>
    "#;

    let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(mock_response)
        .create();

    // Set a date for filtering articles using from_ymd_opt
    let since_date = NaiveDate::from_ymd_opt(2024, 9, 1)
        .expect("Invalid date provided"); // Handle the case where the date is invalid

    let fetcher = EprintFetcher;
    let articles = fetcher.fetch_articles(&mockito::server_url(), &since_date, "TestEprintBlog")
        .await
        .expect("Failed to fetch Eprint articles");

    assert_eq!(articles.len(), 2);
    assert_eq!(articles[0].title, "Interactive Line-Point Zero-Knowledge with Sublinear Communication and Linear Computation");
    assert_eq!(articles[0].url, "https://eprint.iacr.org/2024/1431");
    assert_eq!(articles[0].blog_name, "Eprint");
    assert_eq!(articles[0].authors, Some("Fuchun Lin, Chaoping Xing and Yizhou Yao".to_string()));
    assert_eq!(articles[1].title, "Another");
    assert_eq!(articles[1].url, "https://eprint.iacr.org/2024/9999");
    assert_eq!(articles[1].blog_name, "Eprint");
    assert_eq!(articles[1].authors, Some("Author 1 and Author 2".to_string()));
}

#[tokio::test]
async fn test_fetch_eprint_articles_with_filtering() {
    let mock_response = r#"
    <?xml version="1.0" encoding="UTF-8"?> 
    <OAI-PMH xmlns="http://www.openarchives.org/OAI/2.0/">
      <ListRecords>
        <record> 
          <header>
            <identifier>oai:eprint.iacr.org:2024/1431</identifier> 
            <datestamp>2024-09-18T07:46:25Z</datestamp>
          </header>
          <metadata>
            <oai_dc:dc>
              <dc:title>Test Title</dc:title> 
              <dc:identifier>https://eprint.iacr.org/2024/1431</dc:identifier>              
              <dc:creator>Dan Boneh</dc:creator>
              <dc:date>2024-09-18T07:46:25Z</dc:date>
              <dc:description>This paper discusses zero-knowledge proofs.</dc:description> 
            </oai_dc:dc>
          </metadata>
        </record>
        <record> 
          <header>
            <identifier>oai:eprint.iacr.org:2024/9999</identifier> 
            <datestamp>2024-09-19T02:36:04Z</datestamp>
          </header>
          <metadata>
            <oai_dc:dc>
              <dc:title>Another Title</dc:title> 
              <dc:identifier>https://eprint.iacr.org/2024/9999</dc:identifier>     
              <dc:creator>Some Author</dc:creator>
              <dc:date>2024-09-19T02:36:04Z</dc:date>
              <dc:description>This paper discusses unrelated topics.</dc:description> 
            </oai_dc:dc>
          </metadata>
        </record>
        <record> 
          <header>
            <identifier>oai:eprint.iacr.org:2024/1234</identifier> 
            <datestamp>2024-09-19T02:36:04Z</datestamp>
          </header>
          <metadata>
            <oai_dc:dc>
              <dc:title>Another Title</dc:title> 
              <dc:identifier>https://eprint.iacr.org/2024/9999</dc:identifier>     
              <dc:creator>Some Author</dc:creator>
              <dc:date>2024-06-19T02:36:04Z</dc:date>
              <dc:date>2024-09-19T02:36:04Z</dc:date>
              <dc:description>This paper discusses ZK but it's an old one, just recently updated.</dc:description> 
            </oai_dc:dc>
          </metadata>
        </record>        
      </ListRecords>
    </OAI-PMH>
    "#;

    let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(mock_response)
        .create();

    let since_date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();
    let fetcher = EprintFetcher;
    let articles = fetcher.fetch_articles(&mockito::server_url(), &since_date, "TestEprintBlog")
        .await
        .expect("Failed to fetch Eprint articles");

    assert_eq!(articles.len(), 1); // Only one article should be included
    assert_eq!(articles[0].title, "Test Title");
    assert_eq!(articles[0].url, "https://eprint.iacr.org/2024/1431");
    assert_eq!(articles[0].authors, Some("Dan Boneh".to_string()));
}