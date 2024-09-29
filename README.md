# zkMesh Fetcher

zkMesh monthly newsletter sharing the latest in decentralised privacy-preserving technologies, privacy protocol development and zero-knowledge systems – you can check it out and subscribe [here](https://zkmesh.substack.com/).

zkMesh Fetcher is a Rust-based tool designed to fetch and aggregate blog articles from various sources, including (for now) Substack pages, RSS and Atom feeds, custom HTML pages, and ePrint.

## How it Works

1. **Input**: The program reads a list of blogs from a configuration file (default: `./config/blogs.json`). Each entry in this file should contain information about a blog / feed.

2. **Feed Types**: The program supports five types of feeds:
   - Substack
   - RSS
   - Atom
   - CustomHTML
   - ePrint

3. **Fetching Articles**: For each blog in the input file, the program:
   - Determines the appropriate fetcher based on the feed type.
   - Fetches articles published since a specified date.
   - Parses the fetched data and extracts relevant information (title, URL, publication date).

4. **ePrint Search**: For ePrint articles, the program:
   - Loads the ePrint configuration containing keywords and authors of interest.
   - Parses the ePrint XML feed to extract article metadata (title, authors, description, subject).
   - Filters articles based on the presence of specified keywords in the title, description, or subjects.
   - For each matching article, extracts the publication date and ensures it is after the specified date.
   - Converts the extracted information, including the paper's authors.

5. **Output**: The program generates an HTML output file located at `./output/index.html`, which contains the fetched articles and any errors encountered during the fetching process.

## Usage

Run the program with the following command:

```
cargo run <blogs_json> <since_date>
```

- `blogs_file`: Path to the file containing blog information
- `since_date`: Fetch articles published since this date (format: YYYY-MM-DD)

Both arguments can take default values (`./config/blogs.json` for blogs_json and first day of the current month for since_date). So this will run:

```bash
cargo run
```

## Code Structure

- `main.rs`: Contains the main program logic and CLI interface.
- `feed_types/`: Module containing implementations for different feed types.
- `errors.rs`: Custom error types for the application.
- `utils.rs`: Utility functions for parsing dates and reading blog information.
- `config.rs`: Functions for reading blog configurations from a file.
- `models.rs`: Data structures used in the application.

## Testing

The project includes a test suite covering various components:
- Mock server tests for each feed type
- Error handling tests
- Utility function tests

Run the tests using:
`bash
cargo test
`

## License

This project is licensed under the MIT License. See the LICENSE file for details.