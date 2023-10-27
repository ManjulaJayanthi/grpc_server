use crate::{
    blog::{blog_model::BlogRuntimeMockResponse, blog_response::BlogError},
    grpc_blog::grpc_blog_app::{grpc_blog_thumbsdown, grpc_blog_thumbsup},
};

use super::{
    grpc_blog_app::grpc_get_blog,
    grpc_blog_route::blog::{
        blog_runtime_request::BlogUpdateComparision, BlogRuntimeRequest, BlogRuntimeResponse,
        BlogThumbsdownRequest, BlogThumbsupRequest,
    },
};

impl BlogThumbsupRequest {
    pub fn update_thumbsup(self, mock: BlogRuntimeMockResponse) -> BlogRuntimeMockResponse {
        println!(
            "\n\n\n  BlogThumbsupRequest self {:?}, mock {:?}",
            self.clone().thumps_up,
            mock.clone().thumps_up
        );
        BlogRuntimeMockResponse {
            blog_id: self.blog_id,
            thumps_down: mock.clone().thumps_down,
            thumps_up: self.thumps_up + mock.thumps_up,
            who: self.who,
        }
    }
}

impl BlogThumbsdownRequest {
    pub fn update_thumbsdown(self, mock: BlogRuntimeMockResponse) -> BlogRuntimeMockResponse {
        println!(
            "\n\n\n  self {:?}, mock {:?}",
            self.clone().thumps_down,
            mock.clone().thumps_down
        );
        BlogRuntimeMockResponse {
            blog_id: self.blog_id,
            thumps_down: self.thumps_down + mock.clone().thumps_down,
            thumps_up: mock.thumps_up,
            who: self.who,
        }
    }
}

impl BlogRuntimeRequest {
    pub async fn update_into(self) -> Result<BlogRuntimeResponse, BlogError> {
        println!("\n update_into {:?}", &self);
        let blog_update = match self.blog_update_comparision {
            Some(blog_update_comparision) => blog_update_comparision,
            None => Err("No update found").unwrap(),
        };
        match blog_update {
            BlogUpdateComparision::BlogThumbsdownRequest(req) => {
                Ok(grpc_blog_thumbsdown(req).await?)
            }
            BlogUpdateComparision::BlogThumbsupRequest(req) => Ok(grpc_blog_thumbsup(req).await?),
        }
    }
}

impl BlogRuntimeMockResponse {
    pub fn into_grpc_blog_reponse(self) -> BlogRuntimeResponse {
        BlogRuntimeResponse {
            blog_id: self.blog_id,
            who: self.who,
            thumbs_down: self.thumps_down,
            thumbs_up: self.thumps_up,
        }
    }
}

pub async fn get_into(blog_id: String) -> Result<BlogRuntimeResponse, BlogError> {
    // println!("\n get_into {:?}", &blog_id);
    Ok(grpc_get_blog(blog_id).await.unwrap())
}
