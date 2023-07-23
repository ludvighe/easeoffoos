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
            Err(_) => 0,
        },
        None => 0,
    }
    .min(n_words);

    let use_html = match params.html {
        Some(val) => val == "true" || val == "1",
        None => false,
    };

    // Get Lorem words
    let lorem_words = data::get_words();

    let lorem_len = lorem_words.len();
    let lorem_max_i = lorem_len - 1;

    let mut result: Vec<String> = Vec::new();
    for i in 0..n_words {
        result.push(lorem_words[i % lorem_max_i].clone());
    }

    let mut result_str = result.join(" ");

    // Chunk paragraphs if requested
    if n_paragraphs != 0 {
        let paragraph_len = result.len() / n_paragraphs;
        let mut result_p: Vec<String> = Vec::new();

        let paragraphs: Vec<&[String]> = result.chunks(paragraph_len).collect();

        for i in 0..n_paragraphs {
            let mut res = paragraphs[i].join(" ");
            let first_char = res.get(0..1).unwrap().to_uppercase().to_string(); // TODO: Handle unwrap
            res.replace_range(0..1, &first_char);

            let last_char = res
                .get(res.len() - 1..)
                .unwrap()
                .to_string()
                .replace(",", "");

            res.replace_range(res.len() - 1.., &last_char);
            if last_char != "?" {
                res.push('.');
            }

            result_p.push(res);
        }

        if use_html {
            result_str = result_p.join("</p><p>");
        } else {
            result_str = result_p.join("\n\n");
        }
    }

    if use_html {
        result_str.insert_str(0, "<p>");
        result_str.push_str("</p>");
    }
    return Html(result_str);
}
