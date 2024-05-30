mod llm_convo;
use llm_convo::LLMConvoMethods;
mod markdown;
use markdown::compile_monodoc::compile_mono_doc;
use markdown::read_file_list::read_file_list;
use reqwest::Client;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use assist_macro::generate_tools_config;

#[generate_tools_config]
/// Forces the AI to make a yes/no choice on the situation at hand.
///
/// # Parameters
/// - `decision`: The answer to the decision, true for yes, false for no.
/// - `confidence`: Floating point confidence values of the decisions.
/// - `answer`: Yes or no string which represents the decision.
/// - `reasoning`: The justification as to why the decision was made.
fn example_function(
    decision: bool,
    confidence: Vec<Vec<f64>>,
    answer: String,
    reasoning: Option<String>,
) -> String {
    "Decision made.".to_string()
}

const CONFIG_FILE_NAME: &str = ".api_key.conf";

// Function to get the path to the configuration file in the user's home directory
fn get_config_file_path() -> PathBuf {
    let mut home_dir = dirs::home_dir().expect("Could not find home directory");
    home_dir.push(CONFIG_FILE_NAME);
    home_dir
}

// Function to read the API key from a configuration file
fn read_api_key_from_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut api_key = String::new();
    file.read_to_string(&mut api_key)?;
    Ok(api_key.trim().to_string()) // Trim any extra whitespace or newlines
}

// Function to save the API key to a configuration file
fn save_api_key_to_file(path: &Path, key: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(key.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <option> [api_key]", args[0]);
        println!("Options:");
        println!("  chat, c            Enter chat mode");
        println!("  key <api_key>      Set the API key");
        println!("  analyze, a         Inspect and compile markdown files");
        return;
    }

    let config_file_path = get_config_file_path();

    match args[1].as_str() {
        "key" | "k" | "-k" | "--k" => {
            if args.len() < 3 {
                eprintln!("Error: Missing API key.");
                return;
            }
            let api_key = args[2].clone();
            if let Err(err) = save_api_key_to_file(&config_file_path, &api_key) {
                eprintln!("Failed to save API key: {}", err);
            } else {
                println!("API key set successfully.");
            }
            return; // Return early to avoid falling through to the "Unknown option" case
        }
        "chat" | "c" | "-c" | "--c" => {
            let mut api_key = String::new();
            // Retrieve the API key from the configuration file
            match read_api_key_from_file(&config_file_path) {
                Ok(key) => api_key = key,
                Err(err) => {
                    eprintln!("API key not set. Please set the API key using the 'key' option.");
                    eprintln!("Error: {}", err);
                    return;
                }
            }

            let client = Client::new();
            let model = "gpt-4o".to_string();

            let mut convo = llm_convo::oai_convo::OAIConvo3_0_0::new(client, api_key, model);

            convo
                .add_system_message("System initialized.".to_string())
                .await;
            println!("System initialized.");

            loop {
                print!("You: ");
                io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");
                let user_input = user_input.trim().to_string(); // Remove any trailing newlines or whitespace

                if user_input.to_lowercase() == "exit" {
                    break;
                }

                convo.add_user_message(user_input).await;

                let response = convo.request_response(true, 2048).await;
                println!("Assistant: {}", response);

                // Optionally, clear messages if needed or add other logic
                // convo.llm_convo.clear_messages();
            }
            println!("Exiting the chat.");
        }
        "analyze" | "a" | "-a" | "--a" => {
            let file_list_path = "./assist/list.md";

            match read_file_list(file_list_path) {
                Ok(file_list) => match compile_mono_doc(file_list) {
                    Ok(output) => println!("{}", output),
                    Err(e) => eprintln!("Error compiling document: {}", e),
                },
                Err(e) => eprintln!("Error reading file list: {}", e),
            }
        }
        _ => {
            println!("Unknown option: {}", args[1]);
            println!("Usage: {} <option> [api_key]", args[0]);
            println!("Options:");
            println!("  chat, c            Enter chat mode");
            println!("  key <api_key>      Set the API key");
            println!("  inspect, i         Inspect and compile markdown files");
        }
    }
}
