mod gql;
mod model;

use std::io::{stdin, stdout, Write};
use std::error::Error;
//use fuzzypicker::FuzzyPicker;
use form_urlencoded::byte_serialize;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, REFERER};

fn urlencode(input: &str) -> String {
    byte_serialize(input.as_bytes()).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let stdin = stdin();

    stdout.write(b"Search Anime: ")?;
    stdout.flush()?;

    let mut anime_name = String::new();
    stdin.read_line(&mut anime_name)?;
    anime_name = String::from(anime_name.trim());
    println!("Searching for \"{anime_name}\"...");

    let base_url = "https://api.allanime.day/api";
    let (variables, query) = gql::build_query(&urlencode(anime_name.as_str()));
    let url = format!(
	"{}?variables={}&query={}",
	base_url,
	urlencode(&variables), 
	urlencode(&query)
    );
    //println!("{url}");
    
    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_str("https://allmanga.to")?);
    let client = Client::builder()
	.default_headers(headers)
        .build()?;
    let response = client.get(url).send()?;
    let text = response.text()?;
    //println!("{:?}", text);
    let json: model::Json = serde_json::from_str(&text)?;
    //let json: serde_json::Value = serde_json::from_str(&text)?;
    let anime_list = json.get_anime_list();
    //println!("{:#?}", anime_list);
    for anime in anime_list {
	println!("{anime}");
    }
    Ok(())
}
