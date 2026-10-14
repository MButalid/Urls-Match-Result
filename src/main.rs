use url;

fn main() -> Result<(), std::io::Error> {
    let file_contents = std::fs::read_to_string("src/data/content.txt")?;
    println!("File opened and read");

    let my_contents =  file_contents;

    for line in my_contents.lines() {
        match parse_url(line) {
            Some(url) => {
                println!("Is URL: {url}");
            },
            None =>  {
                println!("Not an URL");
            }
        }

    }

    Ok(())
}


// Write a function that takes (input: &str) and parses each line and returns Option<url::Url>  
fn parse_url(input: &str) -> Option<url::Url> {
    match url::Url::parse(input) {
        Ok(url) => Some(url),
        Err(_e) => None,
    }
}


#[test]
fn good_url() {
    assert!(parse_url("https://example.com").is_some())
}

#[test]
fn bad_url() {
    assert!(parse_url("asdasdwd").is_none())
}