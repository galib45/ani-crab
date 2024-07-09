
pub fn build_query(anime_name: &str) -> (String, String) {
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
	    episodeDuration
	    lastEpisodeDate
        }
	pageInfo {
	    total
	}
    }
}"#.to_string(); 
    (variables, query)
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
