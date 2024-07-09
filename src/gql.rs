use std::fmt::Display;
use form_urlencoded::byte_serialize;

const BASE_URL: &str = "https://api.allanime.day/api";

fn urlencode<T: AsRef<[u8]>>(input: T) -> String {
    byte_serialize(input.as_ref()).collect()
}

pub struct GqlQuery {
    pub variables: String,
    pub query: String,
}

impl GqlQuery {
    pub fn get_url(&self) -> String {
        format!(
            "{}?variables={}&query={}",
            BASE_URL,
            urlencode(&self.variables), 
            urlencode(&self.query)
        )
    }
}

pub fn build_search_query<T: Display>(anime_name: T) -> GqlQuery {
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

pub fn build_episode_query
<A: Display, B: Display, C: Display>
(id: A, mode: B, ep_no: C) -> GqlQuery {
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
