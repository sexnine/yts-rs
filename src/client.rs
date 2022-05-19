use std::fmt::Display;

use reqwest::{Client as ReqwestClient, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::models::{GetMovieResponse, GetMoviesResponse, Movie, Response, YTSResponse};

static API_BASE: &str = "https://yts.mx/api/v2";

fn endpoint(path: impl Display) -> String {
    format!("{}{}.json", API_BASE, path)
}

#[derive(Debug)]
pub enum YTSError {
    RequestError(String),
    DeserializationError(String),
    BadResponse,
}

pub struct Client {
    pub http_client: ReqwestClient,
    api_key: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            http_client: ReqwestClient::new(),
            api_key: None,
        }
    }

    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    async fn handle_request<A: 'static>(&self, request: RequestBuilder) -> Result<A, YTSError>
    where
        A: Response + DeserializeOwned,
    {
        match request.send().await {
            Ok(x) => match x.json::<YTSResponse<A>>().await {
                Ok(x) => match x.data {
                    Some(x) => Ok(x),
                    _ => Err(YTSError::BadResponse),
                },
                Err(e) => return Err(YTSError::DeserializationError(e.to_string())),
            },
            Err(e) => return Err(YTSError::RequestError(e.to_string())),
        }
    }

    pub async fn get_movies(
        &self,
        options: crate::models::GetMoviesRequest,
    ) -> Result<GetMoviesResponse, YTSError> {
        self.handle_request::<GetMoviesResponse>(
            self.http_client
                .get(endpoint("/list_movies"))
                .query(&options),
        )
        .await
    }

    pub async fn get_movie_by_id(&self, movie_id: u32) -> Result<Movie, YTSError> {
        self.handle_request::<GetMovieResponse>(
            self.http_client
                .get(endpoint("/movie_details"))
                .query(&[("movie_id", movie_id)]),
        )
        .await
        .map(|x| x.movie)
    }

    pub async fn get_movie_by_imdb_id(&self, movie_id: u32) -> Result<Movie, YTSError> {
        self.handle_request::<GetMovieResponse>(
            self.http_client
                .get(endpoint("/movie_details"))
                .query(&[("imdb_id", movie_id)]),
        )
        .await
        .map(|x| x.movie)
    }
}
