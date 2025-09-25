// wikipedia_scraper.rs
// scrapes the paragraph elements in a wikipedia article and prints them

mod wikipedia_scraper; 
use std::io::{self, Write}; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
     loop {
        print!("scraper> "); 
        io::stdout().flush()?; // flush prompt 
        let mut input = String::new(); 
        io::stdin().read_line(&mut input)?; 

        let input = input.trim(); 
        if input.eq_ignore_ascii_case("EXIT") {
            break;
        }

        // call scraper 
        wikipedia_scraper::wikipedia_scraper(input)?; 
        println!("saved paragraphs.txt and links.txt for {}", input);  
     }

    Ok(())

}


