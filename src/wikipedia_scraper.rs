// wikipedia_scraper.rs 
// scrapes wikipedia articles and parses paragraphs 

use scraper::{Html, Selector};
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn wikipedia_scraper(title: &str) -> Result<(), Box<dyn std::error::Error>> { 
    // Download target HTML doc
    let url = format!("https://wikipedia.org/wiki/{}", title);  
    let response = reqwest::blocking::get(&url)?;
    let html_content: String = response.text()?;
    let document = Html::parse_document(&html_content);

    // Create paragraph.txt 
    let paragraph_file = File::create("paragraphs.txt").unwrap();
    let mut paragraph_writer = BufWriter::new(paragraph_file);

    // Select paragraph elements and write them to the file 
    let selector = Selector::parse("p").unwrap();
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        paragraph_writer.write_all(text.as_bytes()).unwrap();
        paragraph_writer.write_all(b"\n").unwrap();
    }

    // Create links.txt 
    let links_file = File::create("links.txt").unwrap();
    let mut link_writer = BufWriter::new(links_file);

    // Select links and writes them to the file 
    let selector = Selector::parse("a").unwrap();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href"){
           link_writer.write_all(href.as_bytes()).unwrap();
           link_writer.write_all(b"\n").unwrap(); 
        }
    }
    Ok(())

}
