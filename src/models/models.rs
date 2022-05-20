use serde::{Deserialize, Serialize};

// TODO: This is dumb as fuck so ill figure it out later
// fn get_false() -> bool {
//     false
// }

#[derive(Deserialize, Clone, Debug)]
pub struct Movie {
    // #[serde(default = "get_false")]
    // fetched_cast: bool,
    pub id: u32,
    pub url: String,
    pub imdb_code: String,
    pub title: String,
    pub title_english: String,
    pub title_long: String,
    pub slug: String,
    pub year: u32,
    pub rating: f32,
    pub runtime: u32,
    pub genres: Vec<String>,
    pub summary: Option<String>,
    pub description_intro: Option<String>,
    pub description_full: String,
    pub synopsis: String,
    pub yt_trailer_code: Option<String>,
    pub mpa_rating: Option<String>,
    pub language: String,
    pub torrents: Option<Vec<Torrent>>,
    pub date_uploaded: Option<String>,
    pub date_uploaded_unix: Option<u64>,
    pub cast: Option<Vec<Actor>>, // TODO: Images, Implement fetching cast
}

#[derive(Deserialize, Clone, Debug)]
pub struct Actor {
    pub name: String,
    pub character_name: String,
    pub imdb_code: String,
    #[serde(rename = "url_small_image")]
    pub image_url: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Torrent {
    pub url: String,
    pub hash: String,
    pub quality: Quality,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub seeds: u32,
    pub peers: u32,
    #[serde(rename = "size_bytes")]
    pub size: u64,
    #[serde(rename = "size")]
    pub size_str: String,
    pub date_uploaded: String,
    pub date_uploaded_unix: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Quality {
    #[serde(rename = "720p")]
    Q720p,
    #[serde(rename = "1080p")]
    Q1080p,
    #[serde(rename = "2160p")]
    Q2160p,
    #[serde(rename = "3D")]
    Q3D,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum MediaType {
    #[serde(rename = "bluray")]
    Bluray,
    #[serde(rename = "web")]
    Web,
}
