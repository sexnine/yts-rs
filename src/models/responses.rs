use serde::Deserialize;

use super::{GetMoviesRequest, Movie, Request};

pub(crate) trait Response {}

#[derive(Debug, Deserialize, Clone)]
pub struct YTSResponse<T> {
    pub data: Option<T>,
    pub status_message: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetMoviesResponse {
    pub movies: Vec<Movie>,
    pub movie_count: u32,
    pub page_number: u32,
}

impl Response for GetMoviesResponse {}

#[derive(Debug, Deserialize, Clone)]
pub struct GetMovieResponse {
    pub movie: Movie,
}

impl Response for GetMovieResponse {}
