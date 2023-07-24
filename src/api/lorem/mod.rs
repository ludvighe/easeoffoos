use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

mod data;

pub fn router() -> Router {
    Router::new().route("/lorem/:words", get(get_lorem))
}

#[derive(Deserialize)]
struct LoremParams {
    /// Number of paragraphs
    p: Option<String>,
    /// Determines if returned result should be with HTML-tags
    html: Option<String>,
}

async fn get_lorem(
    word_count: Path<String>,
    Query(params): Query<LoremParams>,
) -> impl IntoResponse {
    // Parse params
    let n_words = match word_count.0.parse::<usize>() {
        Ok(val) => val,
        Err(_) => usize::MIN,
    }
    .min(1_000_000);

    let n_paragraphs = match params.p {
        Some(n) => match n.parse::<usize>() {
            Ok(val) => val,
            Err(_) => usize::MIN,
        },
        None => usize::MIN,
    }
    .min(n_words);

    let use_html = match params.html {
        Some(val) => val == "true" || val == "1",
        None => false,
    };

    // Get Lorem words
    let lorem_words = data::get_n_words(n_words);
    let mut result_str = lorem_words.join(" ");

    // Chunk paragraphs if requested
    if n_paragraphs != 0 {
        let sep = match use_html {
            true => "</p><p>",
            false => "\n\n",
        };

        let paragraphs = data::words_to_n_paragraphs(lorem_words, n_paragraphs);
        result_str = paragraphs.join(sep);
    }

    if use_html {
        result_str.insert_str(0, "<p>");
        result_str.push_str("</p>");
    }
    return Html(result_str);
}
