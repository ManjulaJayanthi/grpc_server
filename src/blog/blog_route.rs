use axum::{extract::Path, Json};
use tonic::{Request, Response, Status};

use self::blog::{blog_run_time_server::BlogRunTime, BlogRuntimeRequest, BlogRuntimeResponse};

use super::{
    blog_app::{get_blog, write_blog_thumbsdown, write_blog_thumbsup},
    blog_model::{BlogThumbsdownMockRequest, BlogThumbsupMockRequest},
    blog_response::GetBlogMockResponse,
};

pub mod blog {
    tonic::include_proto!("blog");
}

#[derive(Debug, Default)]
pub struct UpdateTheBlogService {}

#[tonic::async_trait]
impl BlogRunTime for UpdateTheBlogService {
    async fn update_blog(
        &self,
        request: Request<BlogRuntimeRequest>,
    ) -> Result<Response<BlogRuntimeResponse>, Status> {
        println!("\n update_blog {:?}", &request);
        let blog_runtime_response = request.into_inner().update_into();
        Ok(Response::new(blog_runtime_response.await.unwrap()))
    }
}

pub async fn get_blog_data(Path(blog): Path<String>) -> GetBlogMockResponse {
    println!("\n get handler {:?}", &blog);
    Ok(get_blog(blog).await?)
}

pub async fn thumbs_up(
    Path(blog): Path<String>,
    Json(request): Json<BlogThumbsupMockRequest>,
) -> GetBlogMockResponse {
    println!("\n thumbs_up handler");
    Ok(write_blog_thumbsup(blog, request).await?)
}

pub async fn thumbs_down(
    Path(blog): Path<String>,
    Json(request): Json<BlogThumbsdownMockRequest>,
) -> GetBlogMockResponse {
    println!("\n thumbs_down handler");
    Ok(write_blog_thumbsdown(blog, request).await?)
}
