use serde::Serialize;

use super::{Quality, SortBy};

#[derive(Debug, Serialize, Clone)]
pub struct GetMoviesRequest {
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub quality: Option<Quality>,
    pub minimum_rating: Option<u8>,
    #[serde(rename = "query_term")]
    pub term: Option<String>,
    pub genre: Option<String>,
    #[serde(flatten)]
    pub sort: Option<SortBy>,
}

impl GetMoviesRequest {
    pub fn new() -> Self {
        GetMoviesRequest::default()
    }

    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page(mut self, value: u32) -> Self {
        self.page = Some(value);
        self
    }

    pub fn quality(mut self, value: Quality) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn minimum_rating(mut self, value: u8) -> Self {
        self.minimum_rating = Some(value);
        self
    }

    pub fn term(mut self, value: String) -> Self {
        self.term = Some(value);
        self
    }

    pub fn genre(mut self, value: String) -> Self {
        self.genre = Some(value);
        self
    }

    pub fn sort(mut self, value: SortBy) -> Self {
        self.sort = Some(value);
        self
    }
}

impl Default for GetMoviesRequest {
    fn default() -> Self {
        GetMoviesRequest {
            limit: None,
            page: None,
            quality: None,
            minimum_rating: None,
            term: None,
            genre: None,
            sort: None,
        }
    }
}
