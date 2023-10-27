use serde::{Deserialize, Serialize};

// use crate::blog::blog_route::blog::blog_runtime_request::BlogUpdateComparision;

// use super::{
//     blog_response::BlogError,
//     blog_route::blog::{
//         BlogRuntimeRequest, BlogRuntimeResponse, BlogThumbsdownRequest, BlogThumbsupRequest,
//     },
// };

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
