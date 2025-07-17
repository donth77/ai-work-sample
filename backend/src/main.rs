use std::{env, sync::Arc};
use std::net::SocketAddr;

use axum::{
    extract::{rejection::JsonRejection, State},
    http::{Method, StatusCode},
    routing::{get, post},
    serve, Json, Router,
};
use dotenvy::dotenv;
use isolang::Language;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
enum Command {
    Paraphrase,
    Summarize,
    Translate,
}

#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
}

// test endpoint
async fn root() -> &'static str {
    "Hello, World!"
}

// convert language code to name using isolang
fn get_language_name(code: &str) -> String {
    // attempt to parse language code
    match Language::from_639_1(code).or_else(|| Language::from_639_3(code)) {
        Some(lang) => lang.to_name().to_string(),
        None => {
            // handle special cases or return fallback
            match code.to_lowercase().as_str() {
                "zh-cn" => "Chinese (Simplified)".to_string(),
                "zh-tw" => "Chinese (Traditional)".to_string(),
                _ => format!("the language with code '{}'", code),
            }
        }
    }
}

fn is_valid_language_code(code: &str) -> bool {
    // check valid ISO code
    Language::from_639_1(code).is_some() 
        || Language::from_639_3(code).is_some()
        || matches!(code.to_lowercase().as_str(), "zh-cn" | "zh-tw") // special cases
}

#[derive(Deserialize)]
struct AiRequest {
    command: String, 
    text: String,
    model: Option<String>,
    lang: Option<String>, // language code for translation
}

#[derive(Serialize)]
struct AiResponse {
    result: String,
    model: String,
    lang: String,
}

async fn parse_ai_request_body(payload: Result<Json<AiRequest>, JsonRejection>) -> Result<AiRequest, (StatusCode, String)> {
    match payload {
        Ok(Json(req)) => Ok(req),
        Err(rejection) => {
            Err((StatusCode::BAD_REQUEST, rejection.to_string()))
        }
    }
}

/// POST /api/ai
async fn ai_handler(
    State(state): State<AppState>,
    payload: Result<Json<AiRequest>, JsonRejection>,
) -> Result<Json<AiResponse>, (StatusCode, String)> {
    let request = parse_ai_request_body(payload).await?;
    
    // check command param
    let command = match request.command.as_str() {
        "paraphrase" => Command::Paraphrase,
        "summarize" => Command::Summarize,
        "translate" => Command::Translate,
        invalid_command => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid command '{}'. Valid commands: 'paraphrase', 'summarize', 'translate'", invalid_command)
            ));
        }
    };

    // translate command - check lang parameter exists
    if matches!(command, Command::Translate) && request.lang.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            "lang parameter is required for translate command. Provide a language code (e.g., 'es', 'fra').".to_string()
        ));
    }

    // validate language code if lang param
    if let Some(lang_code) = &request.lang {
        if !is_valid_language_code(lang_code) {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid language code '{}'. Provide a valid ISO 639-1 (e.g., 'es', 'fr') or ISO 639-3 (e.g., 'spa', 'fra') language code.", lang_code)
            ));
        }
    }

    let language = request.lang.as_deref().unwrap_or("en");
    let language_name = get_language_name(language);

    // check model param
    let model = match request.model.as_deref() {
        Some("gpt-3.5-turbo") => "gpt-3.5-turbo",
        Some("gpt-4.1-mini") => "gpt-4.1-mini", 
        Some(invalid_model) => {
            return Err((
                StatusCode::BAD_REQUEST, 
                format!("Invalid model '{}'. Supported models: 'gpt-3.5-turbo', 'gpt-4.1-mini'", invalid_model)
            ));
        },
        None => "gpt-4.1-mini", // default model
    };
    
    let system_prompt = match command {
        Command::Paraphrase => {
            &format!(
                "You are an expert writing assistant specialized in paraphrasing text. \
                Your task is to rewrite the given text while preserving its original meaning in the {} language, \
                but using different words, sentence structures, and phrasing. \
                Make the paraphrased text clear, natural, and well-written. \
                If necessary, translate the paraphrased text to the {} language. Only return the paraphrased text, nothing else.",
                language_name,
                language_name
            )
        },
        Command::Summarize => {
            &format!(
                "You are an expert text summarizer. Your task is to create a concise, \
                accurate summary of the given text that captures the main points and key information in the {} language. \
                Focus on the most important ideas while maintaining clarity and coherence. \
                The summary should be significantly shorter than the original text. \
                If necessary, translate the summary to the {} language. Only return the summary, nothing else.",
                language_name,
                language_name
            )
        },
        Command::Translate => {
            &format!(
                "You are an expert translator. Your task is to translate the given text to {}. \
                 Provide an accurate, natural-sounding translation that preserves the meaning and tone of the original text. \
                 Consider cultural context and idiomatic expressions when appropriate. \
                 Only return the translated text, nothing else.", 
                language_name
            )
        }
    };

    let api_key =
        env::var("OPENAI_API_KEY").map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "OPENAI_API_KEY not set".into()))?;

    let body = serde_json::json!({
        "model": model,
        "messages": [
            { "role": "system",
              "content": system_prompt
            },
            { "role": "user", "content": request.text }
        ]
    });

    let resp = state
        .client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    let value: serde_json::Value =
        resp.json().await.map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    let result = value["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_owned();

    Ok(Json(AiResponse { result, model: model.to_string(), lang: language.to_string() }))
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let state = AppState {
        client: Arc::new(Client::builder().build()?),
    };

    let app = Router::new()
        .route("/hello", get(root))
        .route("/api/ai", post(ai_handler))
        .with_state(state)
        .layer(cors());

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;
    Ok(())
}
