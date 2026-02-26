use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aish")]
#[command(about = "AI Shell helper - ask AI about command output or directly ask questions")]
struct Cli {
    #[arg(long)]
    api_key: Option<String>,

    #[arg(long, default_value = "https://api.z.ai/api/coding/paas/v4")]
    api_base: String,

    #[arg(short, long, default_value = "glm-4.7")]
    model: String,

    #[arg(short = 'H', long)]
    history: bool,

    question: Option<String>,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

fn get_shell_history() -> Result<String, String> {
    let home = env::var("HOME").map_err(|e| e.to_string())?;
    
    let history_files = vec![
        PathBuf::from(&home).join(".bash_history"),
        PathBuf::from(&home).join(".zsh_history"),
        PathBuf::from(&home).join(".zhistory"),
        PathBuf::from(&home).join(".local/share/fish/fish_history"),
    ];

    for path in history_files {
        if path.exists() {
            let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read {:?}: {}", path, e))?;
            
            let shell = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("history");
            
            let mut history_lines: Vec<String> = content.lines()
                .filter(|line| !line.trim().is_empty())
                .filter(|line| !line.starts_with('#'))
                .filter(|line| {
                    let line = line.trim();
                    !line.starts_with(':') || line.contains(';')
                })
                .map(|line| {
                    if line.starts_with(':') {
                        line.split(';')
                            .nth(1)
                            .unwrap_or("")
                            .trim()
                            .to_string()
                    } else {
                        line.to_string()
                    }
                })
                .collect();

            let limit = 50;
            if history_lines.len() > limit {
                history_lines = history_lines.into_iter().rev().take(limit).rev().collect();
            }

            let summary = format!(
                "Shell: {}\nRecent commands:\n{}",
                shell,
                history_lines.join("\n")
            );
            
            return Ok(summary);
        }
    }

    Err("No shell history file found".to_string())
}

async fn ask_ai(
    question: &str,
    context: Option<&str>,
    api_key: &str,
    api_base: &str,
    model: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut messages = vec![Message {
        role: "system".to_string(),
        content: "You are a helpful assistant that helps with command-line tasks. Answer concisely and accurately.".to_string(),
    }];

    if let Some(ctx) = context {
        messages.push(Message {
            role: "user".to_string(),
            content: format!("Context (command output or history):\n{}", ctx),
        });
    }

    messages.push(Message {
        role: "user".to_string(),
        content: question.to_string(),
    });

    let request = ChatRequest {
        model: model.to_string(),
        messages,
    };

    let response = client
        .post(&format!("{}/chat/completions", api_base))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("API request failed: {}", error_text).into());
    }

    let chat_response: ChatResponse = response.json().await?;
    Ok(chat_response.choices[0].message.content.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let api_key = cli.api_key
        .or_else(|| env::var("OPENAI_API_KEY").ok())
        .or_else(|| env::var("AI_API_KEY").ok())
        .ok_or("API key not found. Set OPENAI_API_KEY or AI_API_KEY environment variable or use --api-key")?;

    let mut stdin_content = String::new();
    let mut has_stdin = false;

    if atty::isnt(atty::Stream::Stdin) {
        io::stdin().read_to_string(&mut stdin_content)?;
        has_stdin = !stdin_content.trim().is_empty();
    }

    let context = if cli.history {
        Some(get_shell_history().unwrap_or_else(|e| {
            eprintln!("Warning: {}", e);
            "No history available".to_string()
        }))
    } else if has_stdin {
        Some(stdin_content)
    } else {
        None
    };

    let question = cli.question.unwrap_or_else(|| {
        if context.is_some() {
            "Analyze the above content and provide helpful insights or suggestions.".to_string()
        } else if cli.history {
            "Analyze my shell history and provide insights or suggestions.".to_string()
        } else {
            eprintln!("No question provided. Usage: ai <question>");
            eprintln!("Or pipe command: some-command | ai <question>");
            eprintln!("Or use history: ai --history");
            std::process::exit(1);
        }
    });

    let response = ask_ai(&question, context.as_deref(), &api_key, &cli.api_base, &cli.model).await?;
    println!("{}", response);

    Ok(())
}
