use std::error::Error;
use form_urlencoded::byte_serialize;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, REFERER};

const BASE_URL: &str = "https://allanime.day";
const API_BASE_URL: &str = "https://api.allanime.day/api";

pub struct Network {
    pub client: Client,
}

impl Network {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(REFERER, HeaderValue::from_str("https://allmanga.to")?);
        let client = Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self{ client })
    }

    pub fn search_anime(&self, anime_name: &str) -> Result<String, Box<dyn Error>> {
        let query = build_search_query(&anime_name);
        let url = query.get_api_url();
        let response = self.client.get(url).send()?;
        let text = response.text()?;
        Ok(text)
    }

    pub fn get_sources(&self, anime_id: &str, translation_mode: &str, episode_no: &str) -> Result<String, Box<dyn Error>> {
        let query = build_sources_query(anime_id, translation_mode, episode_no);
	let url = query.get_api_url();
	let response = self.client.get(url).send()?;
	let text = response.text()?;
        Ok(text)
    }

    pub fn get_links(&self, provider_id: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}{}", BASE_URL, provider_id);
        let response = self.client.get(url).send()?;
	let text = response.text()?;
        Ok(text)
    }
}

fn urlencode<T: AsRef<[u8]>>(input: T) -> String {
    byte_serialize(input.as_ref()).collect()
}

pub struct GqlQuery {
    pub variables: String,
    pub query: String,
}

impl GqlQuery {
    pub fn get_api_url(&self) -> String {
        format!(
            "{}?variables={}&query={}",
            API_BASE_URL,
            urlencode(&self.variables), 
            urlencode(&self.query)
        )
    }
}

pub fn build_search_query(anime_name: &str) -> GqlQuery {
    let variables = format!(r#"{{  
    "search": {{    
        "allowAdult": false,    
        "allowUnknown": false,    
        "query": "{anime_name}"  
    }},  
    "limit": 40,  
    "page": 1,  
    "translationType": "sub",  
    "countryOrigin": "ALL"
}}"#);
    let query = r#"query (
    $search: SearchInput
    $limit: Int
    $page: Int
    $translationType: VaildTranslationTypeEnumType
    $countryOrigin: VaildCountryOriginEnumType
) {
    shows (
        search: $search
        limit: $limit
        page: $page
        translationType: $translationType
        countryOrigin: $countryOrigin
    ) {
        edges {
            _id
            name
	    englishName
	    description
	    score
	    rating
	    status
	    thumbnail
            season
            availableEpisodes
            availableEpisodesDetail
	    episodeDuration
	    lastEpisodeDate
        }
	pageInfo {
	    total
	}
    }
}"#.to_string(); 
    GqlQuery { variables, query }
}

pub fn build_sources_query(id: &str, mode: &str, ep_no: &str) -> GqlQuery {
    let variables = format!(r#"{{
    "showId": "{id}",
    "translationType":"{mode}",
    "episodeString":"{ep_no}"
}}"#);
    let query = r#"query (
    $showId: String!, 
    $translationType: VaildTranslationTypeEnumType!, 
    $episodeString: String!
) {    
    episode(
        showId: $showId        
        translationType: $translationType        
        episodeString: $episodeString    
    ) {        
        episodeString 
        sourceUrls    
    }
}"#.to_string();
    GqlQuery { variables, query }
}

//pub const VAR_QUERY_POPULAR: &str = r#"{
//    "type":"anime",
//    "size":20,
//    "dateRange":1,
//    "page":1,
//}"#;
//
//pub const QUERY_POPULAR: &str = r#"query popularAnime (
//    $type: VaildPopularTypeEnumType!
//    $size: Int!
//    $dateRange: Int
//    $page: Int
//) {
//    queryPopular (
//       type: $type
//       size: $size
//       dateRange: $dateRange
//       page: $page
//    ) {
//        recommendations {
//            anyCard {
//	    	_id
//                name
//	        englishName
//	        description
//	        score
//	        rating
//	        status
//	        thumbnail
//                availableEpisodes
//	        episodeDuration
//	        lastEpisodeDate
//	    }
//        }
//    }
//}"#;
