use crate::error;

/// Parse an input URL, cleaning and fetching title
/// TODO: Can I change `input_url` to `&str`?
pub async fn parse(input_url: String) -> error::Result<(String, String)> {
    // Fetch page info
    let webpage = tokio::task::spawn_blocking(move || {
        let webpage_options = webpage::WebpageOptions {
            useragent: std::env::var("USER_AGENT").unwrap_or("fact-checker/0.1".to_string()),
            ..webpage::WebpageOptions::default()
        };
        webpage::Webpage::from_url(&input_url, webpage_options)
    })
    .await
    .map_err(|_| error::Error::Internal)?
    .map_err(|_| error::Error::NotFound)?;

    // Use canonical url if provided, otherwise effective url after redirections
    let url = webpage.html.url.unwrap_or(webpage.http.url);

    // Use title if provided, otherwise use the url
    let title = webpage.html.title.unwrap_or_else(|| url.clone());

    Ok((url, title))
}
