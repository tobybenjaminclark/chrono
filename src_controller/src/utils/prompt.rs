use crate::types::Event;
use serde::{Deserialize, Serialize};
use serde_json::json;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    println!("API key: {}", api_key);
}


#[derive(Serialize, Deserialize)]
struct EventWithNameDescription {
    name: String,
    description: String,
}

pub async fn get_name_and_description(mut event: Event) -> Result<Event, Box<dyn std::error::Error>> {
    dotenv().ok();
    // Prepare the fields for the prompt
    let before = if event.before.is_empty() {
        "".to_string()
    } else {
        event.before.join(", ")
    };

    let characters = if event.characters.is_empty() {
        "no characters".to_string()
    } else {
        event
            .characters
            .iter()
            .map(|c| c.name.clone()) // assuming Character has a `name` field
            .collect::<Vec<_>>()
            .join(", ")
    };

    let mut prompt = format!(
        "Generate a name and description for a medieval-fantasy event. \
        The player is a timelord who schedules events in the past. \
        This event is a {}. \
        The event occurs before {}. \
        The event involves {}.",
        event._type, before, characters
    );

    if event.effects.iter().any(|e| e.to_lowercase() == "death") && !event.characters.is_empty() {
        let character_name = &event.characters[0].name; // Pick the first character affected
        prompt.push_str(&format!(" The {} should cause {} to die.", event._type, character_name));
    }

    prompt.push_str(" Write the description as a request from a member of the kingdom to the player, in a concise, fun, medieval tone (1–2 sentences). Return the result as JSON in the format {\"name\": ..., \"description\": ... }.");

    // Prepare the request to OpenAI API
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = reqwest::Client::new();
    let body = json!({
        "model": "gpt-4",
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.7
    });

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;

    let res_json: serde_json::Value = res.json().await?;
    let content = &res_json["choices"][0]["message"]["content"];
    let content_str = content.as_str().ok_or("Missing response content")?;

    // Parse the JSON returned by GPT
    let name_description: EventWithNameDescription = serde_json::from_str(content_str)?;

    // Add name and description to the event
    event.name = name_description.name;
    event.description = name_description.description;

    Ok(event)
}
