mod network;
mod model;
mod util;

use std::process::Command;
use std::error::Error;
use fuzzypicker::FuzzyPicker;
use crate::network::Network;
use crate::util::sanitize_filename;

fn main() -> Result<(), Box<dyn Error>> {
    let config = util::parse_args()?;
    if !config.has_task { return Ok(()); }
    
    let anime_name = util::user_input(
	&format!("{}Search Anime: {}", util::COLOR_CYAN_BOLD, util::COLOR_RESET)
    )?;
    println!("Searching for \"{anime_name}\"...");

    let network = Network::new()?;
    let json: model::Json = serde_json::from_str(
	&network.search_anime(&anime_name)?
    )?;
    let anime_list = json.get_anime_list();
    // exit if no anime found
    if anime_list.is_empty() { return Err("No Anime Found.".into()); }
    
    // select anime from the search results
    let mut anime_picker = FuzzyPicker::new(&anime_list);
    let selected_anime = anime_picker.pick()?;
    if selected_anime.is_none() { return Err("No Anime Selected.".into()); }
    let anime = selected_anime.unwrap();

    let mut quality = match config.quality {
	360 => "360",
	480 => "480",
	720 => "720",
	_ => "1080"
    };

    let episode_list = anime.get_episodes_list();
    let mut video_url_list: Vec<String>;
    if config.download {
	let (start, end);
	if config.range_start == 0 {
	    start = String::from(&episode_list[0]);
	} else {
	    start = config.range_start.to_string();
	}
	if config.range_end == 0 || config.range_end as usize > episode_list.len() { 
	    end = String::from(&episode_list[episode_list.len()-1]); 
	} else {
	    end = config.range_end.to_string();
	}
	let start_index = episode_list.iter().position(|s| s.as_str() == start.as_str()).unwrap();
	let end_index = episode_list.iter().position(|s| s.as_str() == end.as_str()).unwrap();

	for episode in &episode_list[start_index..end_index+1] {
	    let video_title = format!("{} - Episode {}", anime.name, episode);
	    println!(
		"{}Downloading {}{}", 
		util::COLOR_GREEN_BOLD, 
		video_title,
		util::COLOR_RESET
	    );
	    let filename = format!("{}.mp4", sanitize_filename(&video_title));
	    video_url_list = get_video_url_list(&network, &anime.id, &episode, &quality)?;
	    let mut cmd = Command::new("yt-dlp")
		.arg(&video_url_list[0])
		.arg("--no-skip-unavailable-fragments")
		.arg("--fragment-retries")
		.arg("infinite")
		.arg("-N")
		.arg("16")
		.arg("-o")
		.arg(&filename)
		.spawn()
		.expect("yt-dlp command failed to start");
	    cmd.wait()?;
	}
	
	return Ok(());
    }
    
    // select episode from the episode list
    let mut episode_picker = FuzzyPicker::new(&episode_list);
    let mut selected_episode = episode_picker.pick()?;
    if selected_episode.is_none() { return Err("No Episode Selected.".into()); }
    let mut episode_no = selected_episode.unwrap();
    
    let total = episode_list.len();
    let mut index = episode_list.iter().position(|s| s.as_str() == episode_no.as_str()).unwrap();
    let commands = vec!["replay", "next", "previous", "select episode", "change quality", "exit"];
    let qualities = vec!["360", "480", "720", "1080"];
    let mut command_picker = FuzzyPicker::new(&commands);
    let mut quality_picker = FuzzyPicker::new(&qualities);
    loop {
	let video_title = format!("{} - Episode {}", anime.name, episode_list[index]);
	video_url_list = get_video_url_list(&network, &anime.id, &episode_list[index], &quality)?;
	//if config.download {
	//    Command::new("yt-dlp")
	//	.arg(&video_url_list[0])
	//}
	Command::new("mpv")
	    .arg(format!("--force-media-title={}", video_title))
	    .arg(&video_url_list[0])
	    .output()
	    .expect("mpv command failed to start");
	let selected_command = command_picker.pick()?;
	if selected_command.is_none() { break; }
	let command = selected_command.unwrap();
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
	    "select episode" => {
		selected_episode = episode_picker.pick()?;
		if selected_episode.is_none() { return Err("No Episode Selected.".into()); }
		episode_no = selected_episode.unwrap();
		index = episode_list.iter().position(|s| s.as_str() == episode_no.as_str()).unwrap();
	    }
	    "change quality" => {
		let selected_quality = quality_picker.pick()?;
		if selected_quality != None {
		    quality = selected_quality.unwrap();
		}
	    },
	    _ => {}
	}
    }
    Ok(())
}

fn get_video_url_list
(network: &Network, id: &str, episode_no: &str, quality: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut video_url_list = Vec::new();
    // get source urls for the selected episode
    let json: serde_json::Value = serde_json::from_str(
	&network.get_sources(id, "sub", episode_no)?
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
    Ok(video_url_list)
}
