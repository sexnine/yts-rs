use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Movie {
    pub id: u32,
    pub url: String,
    pub imdb_code: Option<String>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_long: String,
    pub slug: Option<String>,
    pub year: u32,
    pub rating: f32,
    pub runtime: u32,
    pub genres: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description_full: Option<String>,
    pub synopsis: Option<String>,
    pub yt_trailer_code: Option<String>,
    pub language: String,
    pub torrents: Vec<Torrent>,
    // TODO: figure out Option stuff, date_uploaded, date_uploaded_unix
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
    // TODO: date_uploaded, date_uploaded_unix
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
