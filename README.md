# YouTube Summarizer Backend

This project is a web service that summarizes YouTube videos. It takes a YouTube video URL as input, extracts the transcript, and then uses a large language model to generate a summary.

## Features

- Summarize YouTube videos by providing a URL.
- Built with Rust and the Axum web framework.
- Uses `yt-transcript-rs` to fetch video transcripts.
- Uses `async-openai` to generate summaries.

## Getting Started

### Prerequisites

- Rust
- An OpenAI API key

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/youtube_summarizer-backend.git
   ```
2. Navigate to the project directory:
   ```bash
   cd youtube_summarizer-backend
   ```
3. Create a `.env` file in the root of the project and add your OpenAI API key:
   ```
   OPENAI_API_KEY=your-api-key
   ```
4. Build the project:
   ```bash
   cargo build --release
   ```
5. Run the server:
   ```bash
   cargo run --release
   ```

The server will start on port 3000 by default. You can change the port by setting the `PORT` environment variable.

## API

### `POST /summarize`

This endpoint takes a YouTube video URL and returns a summary of the video.

#### Request Body

```json
{
  "url": "https://www.youtube.com/watch?v=your_video_id"
}
```

#### Response Body

A string containing the summarized text.

#### Example

##### Request

```bash
curl -X POST http://localhost:3000/summarize \
-H "Content-Type: application/json" \
-d '{"url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"}'
```

##### Response

```
"The video is about a man who is never gonna give you up, never gonna let you down, never gonna run around and desert you. He is also never gonna make you cry, never gonna say goodbye, never gonna tell a lie and hurt you."
```

## Dependencies

- [axum](https://github.com/tokio-rs/axum) - Web framework
- [tokio](https://github.com/tokio-rs/tokio) - Asynchronous runtime
- [serde](https://github.com/serde-rs/serde) - Serialization/deserialization framework
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [dotenvy](https://github.com/all-crates/dotenvy) - Dotenv implementation for Rust
- [yt-transcript-rs](https://github.com/abunsen/yt-transcript-rs) - Fetch YouTube transcripts
- [async-openai](https://github.com/64bit/async-openai) - OpenAI API client