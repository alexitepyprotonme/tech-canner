use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_technology(domain: &str) -> Option<String> {
    let client = Client::new();
    let url = format!("{}", domain);

    match client.get(&url).send() {
        Ok(response) => {
            if let Ok(body) = response.text() {
                let document = Document::from(body.as_str());
                // Check for certain patterns in the HTML to identify technologies/frameworks
                if document.find(Name("meta")).any(|n| n.attr("name").unwrap_or("").contains("WordPress")) {
                    return Some("WordPress".to_string());
                }
                // Add more checks for other technologies as needed
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch {}: {}", domain, e);
        }
    }
    None
}

fn main() {
    let file = File::open("../domains.txt"); // Assuming domains are stored in a file named "domains.txt"
    
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            for line in reader.lines() {
                if let Ok(domain) = line {
                    if let Some(tech) = get_technology(&domain.trim()) {
                        println!("Domain: {}, Technology: {}", domain, tech);
                    } else {
                        println!("Unable to determine technology for domain: {}", domain);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
