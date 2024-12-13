use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use base64::{engine::general_purpose, Engine};
use dotenv::dotenv;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

fn main() {
    dotenv().ok();

    // Generate a new APP_KEY
    let key: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
    let base64_key = general_purpose::STANDARD.encode(&key);
    let app_key = format!("base64:{}", base64_key);

    let file_path = Path::new(".env");

    // Read the existing .env file, or create it if it doesn't exist
    let mut content = String::new();
    if file_path.exists() {
        File::open(file_path)
            .expect("Failed to open .env file")
            .read_to_string(&mut content)
            .expect("Failed to read .env file");
    } else {
        println!(".env file not found, Creating new one.");
        File::create(file_path).expect("Failed to create .env file");
    }

    let mut app_key_updated = false;

    let updated_lines: Vec<String> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if line.starts_with("APP_KEY=") {
                app_key_updated = true;
                format!("APP_KEY={}", app_key)
            } else {
                line.to_string()
            }
        })
        .collect();

    let final_lines = if !app_key_updated {
        let mut lines = updated_lines;
        lines.push(format!("APP_KEY={}", app_key));
        lines
    } else {
        updated_lines
    };
    // Rewrite the .env file with updated contents
    let new_content = final_lines.join("\n") + "\n";
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open .env file for writing");

    file.write_all(new_content.as_bytes())
        .expect("Failed to write to .env file");
}
