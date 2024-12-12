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

    let key: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
    let base64_key = general_purpose::STANDARD.encode(&key);
    let app_key = format!("base64:{}", base64_key);

    let file_path = Path::new(".env");

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            // Create a new .env file if it doesn't exist
            let mut file = File::create(file_path).expect("Failed to create .env file");
            file.write_all(b"APP_KEY=\n")
                .expect("Failed to write to .env file");
            file
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read .env file");

    let app_key_exists = content.contains("APP_KEY=");

    // Update APP_KEY in the content
    let new_content = if app_key_exists {
        content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                if line.starts_with("APP_KEY=") {
                    format!("APP_KEY={}\n", app_key)
                } else {
                    line.to_string() + "\n"
                }
            })
            .collect::<String>()
    } else {
        format!("APP_KEY={}\n", app_key)
    };

    // Overwrite the .env file with the updated content
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // Truncate the file before writing
        .open(file_path)
        .expect("Failed to open .env file for writing");

    file.write_all(new_content.as_bytes())
        .expect("Failed to write to .env file");

    println!("New APP_KEY generated and saved to .env file: {}", app_key);
}
