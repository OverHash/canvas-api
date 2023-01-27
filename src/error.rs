use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to create a header for an http request {header}")]
    CreatingHeader { header: InvalidHeaderValue },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
