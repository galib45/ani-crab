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

#[derive(Debug, Deserialize, Serialize)]
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
    available_episodes: AvailableEpisodes,
    episode_duration: Option<String>,
    last_episode_date: LastEpisodeDate,
}

impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{}, {}, {}, {}", 
            self.name,
            self.status.clone().unwrap_or_default(),
            self.get_last_episode_date(), 
            self.get_episode_duration()
        )
    }
}

impl Anime {
    pub fn get_last_episode_date(&self) -> String {
        let mut output = String::new();
        if let DateOrEmpty::Date(date) = &self.last_episode_date.sub {
            if date.month == 0 {
                output.push_str("N/A");
            } else {
                output = format!(
                    "{} {} {}", date.date, get_month_name(date.month), date.year
                );
            }
        }
        output
    }

    pub fn get_episode_duration(&self) -> String {
        let duration: u64 = self.episode_duration
            .clone().unwrap_or_default().parse().unwrap_or_default();
        let minutes = duration / 60000;
        if minutes > 60 { return format!("{}hr {}min", minutes/60, minutes%60) }
        format!("{}min", minutes)
    }
}

fn get_month_name(month: u32) -> &'static str {
    let months = vec![
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    if month > 12 { return months[11] }
    months[month as usize - 1]
}

#[derive(Debug, Deserialize, Serialize)]
struct AvailableEpisodes {
    sub: u32,
    dub: u32,
    raw: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct LastEpisodeDate {
    sub: DateOrEmpty,
    dub: DateOrEmpty ,
    raw: DateOrEmpty,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum DateOrEmpty {
    Date(EpisodeDate),
    Empty(EmptyStruct),
}

#[derive(Debug, Deserialize, Serialize)]
struct EpisodeDate {
    //hour: u32,
    //minute: u32,
    year: u32,
    month: u32,
    date: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct EmptyStruct {}

//#[derive(Debug, Deserialize, Serialize)]
//struct PageInfo {
//    total: u32,
//}

