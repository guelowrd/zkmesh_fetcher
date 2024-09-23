# zkMesh Fetcher

zkMesh monthly newsletter sharing the latest in decentralised privacy-preserving technologies, privacy protocol development and zero-knowledge systems – you can check it out and subscribe [here](https://zkmesh.substack.com/).

zkMesh Fetcher is a Rust-based tool designed to fetch and aggregate blog articles from various sources, including (for now) Substack, RSS, Atom feeds, and custom HTML pages.

## How it Works

1. **Input**: The program reads a list of blogs from a file (default: `blogs.txt`). Each line in this file contains information about a blog in the format: `BlogName|FeedURL|FeedType`.

2. **Feed Types**: The program supports four types of feeds:
   - Substack
   - RSS
   - Atom
   - CustomHTML

3. **Fetching Articles**: For each blog in the input file, the program:
   - Determines the appropriate fetcher based on the feed type.
   - Fetches articles published since a specified date.
   - Parses the fetched data and extracts relevant information (title, URL, publication date).

4. **Output**: The program prints the fetched articles to the console in a formatted manner.

## Usage

Run the program with the following command:
`bash
cargo run <blogs_file> <since_date>
`

- `blogs_file`: Path to the file containing blog information
- `since_date`: Fetch articles published since this date (format: YYYY-MM-DD)

Both arguments are required. For example:

```bash
cargo run blogs.txt 2024-01-01
```

## Code Structure

- `main.rs`: Contains the main program logic and CLI interface.
- `feed_types/`: Module containing implementations for different feed types.
- `errors.rs`: Custom error types for the application.
- `utils.rs`: Utility functions for parsing dates and reading blog information.
- `config.rs`: Functions for reading blog configurations from a file.
- `models.rs`: Data structures used in the application.

## Testing

The project includes a comprehensive test suite covering various components:
- Mock server tests for each feed type
- Error handling tests
- Utility function tests

Run the tests using:
`bash
cargo test
`

## License

This project is licensed under the MIT License. See the LICENSE file for details.