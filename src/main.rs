use clap::Parser;
use colored::*;
use rand::prelude::*;
use std::{collections::HashMap, fs, path::Path};

#[derive(Parser)]
#[command(name = "mood-msg")]
#[command(about = "Generate commit messages based on your mood")]
struct Args {
    /// Mood type (e.g. tired, happy, chaotic)
    mood: String,
}

const OLLAMA_URL: &str = "http://localhost:11434/v1/chat/completions";

fn main() {
    let args = Args::parse();
    let mood = args.mood.to_lowercase();

    let msg = match generate_commit_with_ollama(&mood) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{} {}", "⚠️ Ollama failed:".red(), e);
            fallback_local(&mood)
        }
    };

    println!("{}", msg.bright_magenta().bold());
}

fn fallback_local(mood: &str) -> String {
    let path = Path::new("moods.json");

    let content = if path.exists() {
        fs::read_to_string(path).expect("Failed to read local moods.json")
    } else {
        return format!(
            "🤷 mood '{}' not found and no fallback DB available.",
            mood
        );
    };

    let moods: HashMap<String, Vec<String>> =
        serde_json::from_str(&content).expect("Invalid JSON in moods.json");

    if let Some(messages) = moods.get(mood) {
        let mut rng = rand::thread_rng();
        messages
            .choose(&mut rng)
            .unwrap_or(&"mood not found".to_string())
            .clone()
    } else {
        format!("🤷 unknown mood '{}'. try: tired, happy, sad, etc.", mood)
    }
}

fn generate_commit_with_ollama(mood: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let prompt = format!(
        "Give me a short, witty, sarcastic git commit message written by a developer who is feeling {}. Only output the commit message.",
        mood
    );

    let body = serde_json::json!({
        "model": "mistral",
        "messages": [
            { "role": "user", "content": prompt }
        ],
        "temperature": 0.9,
        "max_tokens": 60
    });

    let res = client.post(OLLAMA_URL).json(&body).send()?;
    let json: serde_json::Value = res.json()?;

    if let Some(err) = json.get("error") {
        return Err(format!(
            "Ollama responded with an error: {}",
            err["message"].as_str().unwrap_or("unknown error")
        )
        .into());
    }

    let msg = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("🧠 Ollama was speechless.")
        .trim()
        .to_string();

    Ok(msg)
}
