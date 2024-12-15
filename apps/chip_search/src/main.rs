use dotenv::dotenv;
use ngyn::prelude::*;
use reqwest::{self, header::ACCESS_CONTROL_ALLOW_ORIGIN};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Serialize, Deserialize)]
struct SearchResponse {
    total_count: u32,
    items: Vec<MarkdownFile>,
}

#[derive(Query)]
struct SearchQuery {
    query: String,
    limit: u8,
}

#[derive(Serialize, Deserialize)]
struct MarkdownFile {
    name: String,
    path: String,
    html_url: String,
    repository: Repository,
    score: f32,
}

#[derive(Serialize, Deserialize)]
struct Repository {
    full_name: String,
}

struct GitHubSearchClient {
    client: reqwest::Client,
    token: Option<String>,
}

impl GitHubSearchClient {
    fn new() -> Self {
        let token = env::var("GITHUB_TOKEN").ok();

        GitHubSearchClient {
            client: reqwest::Client::new(),
            token,
        }
    }

    async fn search_files(
        &self,
        repository: &str,
        query: &str,
        per_page: u8,
        page: u8,
    ) -> Result<SearchResponse, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(token) = &self.token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("token {}", token).parse().unwrap(),
            );
        }

        let full_query = format!("{} language:markdown repo:{}", query, repository);

        let response = self
            .client
            .get("https://api.github.com/search/code")
            .headers(headers)
            .query(&[
                ("q", &full_query),
                ("per_page", &per_page.to_string()),
                ("page", &page.to_string()),
            ])
            .header("User-Agent", "rust-github-markdown-search")
            .send()
            .await?
            .json::<SearchResponse>()
            .await?;

        Ok(response)
    }
}

struct CorsMiddleware;

impl NgynMiddleware for CorsMiddleware {
    async fn handle(cx: &mut NgynContext<'_>) {
        cx.response()
            .headers_mut()
            .append(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    }
}

#[handler]
async fn handle_search(SearchQuery { query, limit }: SearchQuery) -> JsonResult {
    let repository = "ngyn-rs/ngyn";
    let client = GitHubSearchClient::new();
    println!(
        "\nSearching for '{}' in markdown files of {}",
        query, repository
    );

    match client.search_files(repository, &query, limit, 1).await {
        Ok(response) => Ok(json!(response)),
        Err(error) => Err(json!({
            "status": error.status().map(|code| code.as_u16()).unwrap_or(580)
        })),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut app = HyperApplication::default();

    app.use_middleware(CorsMiddleware {});

    app.get("/search", async_wrap(handle_search));

    println!("Starting server on http://127.0.0.1:8000");
    let _ = app.listen("0.0.0.0:8000").await;
}
