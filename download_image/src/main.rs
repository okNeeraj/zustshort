use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a URL is provided as an argument
    if args.len() != 2 {
        eprintln!("Usage: cargo run <URL>");
        std::process::exit(1);
    }

    let url = &args[1];
    println!("Fetching HTML content from: {}", url);

    // Fetch HTML content
    let response = get(url)?;
    let body = response.text()?;
    let document = Html::parse_document(&body);

    // Extract domain name
    let base_url = Url::parse(url)?;
    let domain_name = base_url.host_str().unwrap_or("unknown_domain");

    println!("Domain name: {}", domain_name);

    // Create a directory to save images
    let dir_path = format!("html/images/{}", domain_name);
    fs::create_dir_all(&dir_path)?;

    println!("Saving images to: {}", dir_path);

    // CSS selector to find image elements
    let img_selector = Selector::parse("img").unwrap();

    // Iterate over image elements and download them
    for img in document.select(&img_selector) {
        let img_src = img.value().attr("src").unwrap_or("");
        let img_url = base_url.join(img_src)?;

        println!("Downloading image from: {}", img_url);

        // Extract image name from URL
        let img_name = match img_url.path_segments() {
            Some(segments) => segments.last().unwrap_or("image"),
            None => "image",
        };

        println!("Image name: {}", img_name);

        // Download and save the image
        let img_response = get(img_url.as_str())?;
        let img_bytes = img_response.bytes()?;
        let img_path = format!("{}/{}", dir_path, img_name);
        let mut img_file = fs::File::create(&img_path)?;

        img_file.write_all(&img_bytes)?;
        println!("Saved image: {}", img_path);
    }

    println!("All images downloaded successfully!");
    Ok(())
}
