use tonic::{Request, Response, Status};

use crate::grpc_blog::grpc_blog_model::get_into;

use self::blog::{
    blog_run_time_server::BlogRunTime, BlogRuntimeRequest, BlogRuntimeResponse, GetBlogRequest,
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

    async fn get_blogg(
        &self,
        request: Request<GetBlogRequest>,
    ) -> Result<Response<BlogRuntimeResponse>, Status>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        println!("\n get_blogg {:?} ", request);
        Ok(Response::new(
            get_into(request.into_inner().blog_id).await.unwrap(),
        ))
    }
}
