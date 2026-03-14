/*
 *
 *  * Created: 2026-3-8 2:22:35
 *  * File: lib.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use thiserror::Error;

pub mod player_uuid;
pub mod auth;
#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Timed out")]
    TimeoutError,
    #[error("Other error: {0}")]
    Other(String),

}