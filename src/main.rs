mod network;
mod model;
mod util;

use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::error::Error;
use fuzzypicker::FuzzyPicker;
//use reqwest::blocking::Client;
//use reqwest::header::{HeaderMap, HeaderValue, REFERER};
use crate::network::Network;


fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let stdin = stdin();

    stdout.write(b"Search Anime: ")?;
    stdout.flush()?;

    let mut anime_name = String::new();
    stdin.read_line(&mut anime_name)?;
    anime_name = String::from(anime_name.trim());
    println!("Searching for \"{anime_name}\"...");

    let network = Network::new()?;
    let json: model::Json = serde_json::from_str(
	&network.search_anime(&anime_name)?
    )?;
    let anime_list = json.get_anime_list();
    let mut video_url_list;
    if !anime_list.is_empty() {
	// select anime from the search results
	let mut anime_picker = FuzzyPicker::new(&anime_list);
	let selected_anime = anime_picker.pick()?;
	if let Some(anime) = selected_anime {
	    // select episode from the episode list
	    let episode_list = anime.get_episodes_list();
	    let mut episode_picker = FuzzyPicker::new(&episode_list);
	    let selected_episode = episode_picker.pick()?;
	    if let Some(episode_no) = selected_episode {
		let total = episode_list.len();
		let mut index = episode_list.iter().position(|s| s.as_str() == episode_no.as_str()).unwrap();
		let mut quality = "360";
		let commands = vec!["replay", "next", "previous", "change quality", "exit"];
		let qualities = vec!["360", "480", "720", "1080"];
		let mut command_picker = FuzzyPicker::new(&commands);
		loop {
		    video_url_list = Vec::new();
		    // get source urls for the selected episode
		    let json: serde_json::Value = serde_json::from_str(
			&network.get_sources(&anime.id, "sub", &episode_list[index])?
		    )?;	
		    let sources = json["data"]["episode"]["sourceUrls"].as_array().unwrap();
		    for source in sources {
			let source_name = source["sourceName"].as_str().unwrap();
			match source_name {
			    "Luf-mp4" => { //| "Sak" | "Kir" | "S-mp4"
				let source_url = &source["sourceUrl"].as_str().unwrap()[2..];
				let provider_id = util::decode_provider_id(source_url);
				let json: serde_json::Value = serde_json::from_str(
				    &network.get_links(&provider_id)?
				)?;
				let links = json["links"].as_array().unwrap();
				for link in links {
				    let video_url = link["link"].as_str().unwrap();
				    if video_url.ends_with(".m3u8") {
					video_url_list.push(video_url.replace(".m3u8", format!(".{}.m3u8", quality).as_str()));
				    }
				}
			    },
			    _ => {}
			}
		    }
		    Command::new("mpv")
			.arg(&video_url_list[0])
			.output()
			.expect("mpv command failed to start");
		    let selected_command = command_picker.pick()?;
		    if let Some(command) = selected_command {
			match command {
			    "exit" => break,
			    "next" => {
				if index < total-1 { index += 1; }
				else { break; }
			    },
			    "previous" => {
				if index > 0 { index -= 1; }
				else { break; }
			    },
			    "change quality" => {
				let mut quality_picker = FuzzyPicker::new(&qualities);
				let selected_quality = quality_picker.pick()?;
				if selected_quality != None {
				    quality = selected_quality.unwrap();
				}
			    },
			    _ => {}
			}
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
