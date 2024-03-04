use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the user provides a URL as a command line argument
    if args.len() != 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }

    // Extract the URL from the command line arguments
    let url_str = &args[1];

    // Parse the URL to extract the domain name
    let url = Url::parse(url_str)?;

    // Extract the domain name from the URL
    let domain = url
        .host_str()
        .ok_or("Invalid URL: no host found")?
        .to_string();

    // Create a file name from the domain
    let file_name = domain;

    // Send an HTTP GET request to the specified URL
    let response = reqwest::get(url_str).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Read the HTML content from the response body
        let html_content = response.text().await?;

        let html_folder = "html";
        if !Path::new(html_folder).exists() {
            std::fs::create_dir(html_folder)?;
        }

        // Write the HTML content to a file named after the domain
        // let file_path = format!("{}.html", file_name);
        let file_path = format!("{}/{}.html", html_folder, file_name);
        let mut file = File::create(&file_path)?;
        file.write_all(html_content.as_bytes())?;

        println!("HTML content saved to {}", file_path);
    } else {
        println!("Error: Request failed with status code {}", response.status());
    }

    Ok(())
}
