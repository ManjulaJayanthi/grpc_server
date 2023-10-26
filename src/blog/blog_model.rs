use axum::Json;
use serde::{Deserialize, Serialize};

use crate::blog::{
    blog_app::grpc_blog_thumbsup, blog_route::blog::blog_runtime_request::BlogUpdateComparision,
};

use super::{
    blog_app::return_mock,
    blog_response::BlogError,
    blog_route::blog::{
        BlogRuntimeRequest, BlogRuntimeResponse, BlogThumbsdownRequest, BlogThumbsupRequest,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogRuntimeMockResponse {
    pub blog_id: String,
    pub thumps_down: i32,
    pub thumps_up: i32,
    pub who: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BlogRuntimeMockRequest {
    BlogThumbsupMockRequest,
    BlogThumbsdownMockRequest,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlogThumbsupMockRequest {
    pub blog_id: String,
    pub thumps_up: i32,
    pub who: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlogThumbsdownMockRequest {
    pub blog_id: String,
    pub thumps_down: i32,
    pub who: String,
}

impl BlogThumbsdownMockRequest {
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

impl BlogThumbsupMockRequest {
    pub fn update_thumbsup(self, mock: BlogRuntimeMockResponse) -> BlogRuntimeMockResponse {
        println!(
            "\n\n\n  self {:?}, mock {:?}",
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

impl BlogRuntimeMockResponse {
    pub fn into_grpc_blog_reponse(self) -> BlogRuntimeResponse {
        BlogRuntimeResponse {
            blog_id: self.blog_id,
            who: self.who,
            thumpsdown: self.thumps_down,
            thumpsup: self.thumps_up,
        }
    }
}

impl BlogRuntimeResponse {
    pub fn into_blog_reponse(self) -> BlogRuntimeMockResponse {
        BlogRuntimeMockResponse {
            blog_id: self.blog_id,
            thumps_down: self.thumpsdown,
            thumps_up: self.thumpsup,
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

        let mock = return_mock();
        match blog_update {
            BlogUpdateComparision::BlogThumbsdownRequest(req) => {
                // Ok(Json(grpc_blog_thumbsup(req).await?))
                Ok(BlogRuntimeResponse {
                    blog_id: mock.clone().blog_id,
                    thumpsdown: mock.clone().thumpsdown + req.thumps_down,
                    thumpsup: mock.clone().thumpsup,
                    who: req.who,
                })
            }
            BlogUpdateComparision::BlogThumbsupRequest(req) => {
                Ok(grpc_blog_thumbsup(req).await?)
            }
            // BlogRuntimeResponse {
            //     blog_id: mock.clone().blog_id,
            //     thumpsdown: mock.clone().thumpsdown,
            //     thumpsup: mock.clone().thumpsup + req.thumps_up,
            //     who: req.who,
            // },
        }
    }
}
