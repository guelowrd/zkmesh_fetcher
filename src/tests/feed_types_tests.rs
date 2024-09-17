use crate::feed_types::FeedType;
use std::str::FromStr;

#[test]
fn test_feed_type_from_str() {
    assert_eq!(FeedType::from_str("Substack").unwrap(), FeedType::Substack);
    assert_eq!(FeedType::from_str("RSS").unwrap(), FeedType::RSS);
    assert_eq!(FeedType::from_str("Atom").unwrap(), FeedType::Atom);
    assert!(FeedType::from_str("Invalid").is_err());
}