use std::io::Error as IoError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use super::blog_model::BlogRuntimeMockResponse;

#[derive(Debug)]
pub enum BlogError {
    IoError(IoError),
    NotFound
}

impl From<IoError> for BlogError {
    fn from(err: IoError) -> Self {
        BlogError::IoError(err)
    }
}

pub type BlogResult<T> = Result<T, BlogError>;
pub type GetBlogResponse = BlogResult<Json<BlogRuntimeMockResponse>>;
pub type GetAllBlogResponse = BlogResult<Json<Vec<BlogRuntimeMockResponse>>>;

impl IntoResponse for BlogError {
    fn into_response(self) -> Response {
        println!("error :{:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}
