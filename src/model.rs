use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Json {
    data: Data,
}

impl Json {
    pub fn get_anime_list(&self) -> &Vec<Anime> {
        &self.data.shows.edges
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    shows: Shows,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Shows {
    edges: Vec<Anime>,
    //page_info: PageInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Anime {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub english_name: Option<String>,
    pub description: Option<String>,
    pub score: Option<f64>,
    pub rating: Option<String>,
    pub status: Option<String>,
    pub thumbnail: String,
    season: Option<Season>,
    available_episodes: AvailableEpisodes,
    available_episodes_detail: AvailableEpisodesDetail,
    episode_duration: Option<String>,
    last_episode_date: LastEpisodeDate,
}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, " [")?;
        let mut inside_square_brackets = 0;
        let na = "N/A".to_string();
        let season = self.get_season();
        let eps = self.get_available_episodes();
        let duration = self.get_episode_duration();
        if season != na { 
            if inside_square_brackets > 0 { write!(f, ", ")?; }
            write!(f, "{}", season)?;
            inside_square_brackets += 1;
        }
        if inside_square_brackets > 0 { write!(f, ", ")?; }
        write!(f, "{}", eps)?;
        if duration != na { 
            if inside_square_brackets > 0 { write!(f, ", ")?; }
            write!(f, "{}", duration)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Anime {
    pub fn get_season(&self) -> String {
        let mut output = "N/A".to_string();
        if let Some(season) = &self.season {
            output = format!("{} {}", season.quarter, season.year);
        }
        output
    }

    pub fn get_episode_duration(&self) -> String {
        let duration: u64 = self.episode_duration
            .clone().unwrap_or_default().parse().unwrap_or_default();
        if duration == 0 { return "N/A".to_string(); }
        let minutes = duration / 60000;
        if minutes > 60 { return format!("{}hr {}min", minutes/60, minutes%60); }
        format!("{}min", minutes)
    }

    pub fn get_available_episodes(&self) -> String {
        format!(
            "{} eps", 
            self.available_episodes.sub
        )
    }

    pub fn get_episodes_list(&self) -> &Vec<String> {
        &self.available_episodes_detail.sub
    }
}

//fn get_month_name(month: u32) -> &'static str {
//    let months = vec![
//        "January", "February", "March", "April", "May", "June",
//        "July", "August", "September", "October", "November", "December"
//    ];
//    if month > 12 { return months[11] }
//    months[month as usize - 1]
//}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AvailableEpisodes {
    sub: u32,
    dub: u32,
    raw: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AvailableEpisodesDetail {
    sub: Vec<String>,
    dub: Vec<String>,
    raw: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct LastEpisodeDate {
    sub: DateOrEmpty,
    dub: DateOrEmpty ,
    raw: DateOrEmpty,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum DateOrEmpty {
    Date(EpisodeDate),
    Empty(EmptyStruct),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EpisodeDate {
    //hour: u32,
    //minute: u32,
    year: u32,
    month: u32,
    date: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EmptyStruct {}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Season {
    quarter: String,
    year: u32,
}

//#[derive(Debug, Deserialize, Serialize)]
//struct PageInfo {
//    total: u32,
//}

