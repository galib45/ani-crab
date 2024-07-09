mod gql;
mod model;

use std::io::{stdin, stdout, Write};
use std::error::Error;
use fuzzypicker::FuzzyPicker;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, REFERER};


fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let stdin = stdin();

    stdout.write(b"Search Anime: ")?;
    stdout.flush()?;

    let mut anime_name = String::new();
    stdin.read_line(&mut anime_name)?;
    anime_name = String::from(anime_name.trim());
    println!("Searching for \"{anime_name}\"...");

    let mut gql_query: gql::GqlQuery;
    gql_query = gql::build_search_query(&anime_name);
    let mut url = gql_query.get_url();    
    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_str("https://allmanga.to")?);
    let client = Client::builder()
	.default_headers(headers)
        .build()?;
    let mut response = client.get(url).send()?;
    let mut text = response.text()?;
    //println!("{:?}", text);
    let json: model::Json = serde_json::from_str(&text)?;
    let anime_list = json.get_anime_list();
    if !anime_list.is_empty() {
	let mut anime_picker = FuzzyPicker::new();
	anime_picker.set_items(&anime_list);
	let selected_anime = anime_picker.pick()?;
	if let Some(anime) = selected_anime {
	    let mut episode_picker = FuzzyPicker::new();
	    episode_picker.set_items(anime.get_episodes_list());
	    let selected_episode = episode_picker.pick()?;
	    if let Some(episode) = selected_episode {
		gql_query = gql::build_episode_query(anime.id, "sub", episode);
		url = gql_query.get_url();
		response = client.get(url).send()?;
		text = response.text()?;
		let json: serde_json::Value = serde_json::from_str(&text)?;
		let sources = json["data"]["episode"]["sourceUrls"];
		for source in sources {
		    let source_name = source["sourceName"];
		    match source_name.as_str() {
			"Luf-mp4" | "Sak" | "Yt-mp4" | "S-mp4" => {
			    println!("{}", source_name);
			    println!("{}", source["sourceUrl"]);
			    println!("");
			},
			_ => {}
		    }
		}
	    } else {
		println!("Selection cancelled.")
	    }
	} else {
	    println!("Selection cancelled.")
	}
    } else {
	println!("No anime found!");
    }
    
    Ok(())
}
