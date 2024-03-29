use std::{
    env,
    fs::{self, File},
    path::Path,
};

use axum::Json;

use crate::blog::blog_response::BlogError;

use super::{
    blog_model::{BlogRuntimeMockResponse, BlogThumbsdownMockRequest, BlogThumbsupMockRequest},
    blog_response::GetBlogMockResponse,
};

pub async fn get_blog(blog_id: String) -> GetBlogMockResponse {
    let mut n_path = Path::new(".").join("src");
    n_path.push(env::var("MOCK_DATA_FILE").expect("MOCK_DATA_FILE missing"));
    dbg!(&n_path);
    match File::open(n_path.clone()) {
        Err(why) => panic!("couldn't open MOCK_DATA_FILE {}", why),
        Ok(file) => {
            println!(">>> {:?}", &file);
            let data_str = fs::read_to_string(n_path).expect("Unable to read file MOCK_DATA_FILE");

            let data_vec: Vec<BlogRuntimeMockResponse> = serde_json::from_str(&data_str)
                .expect("Unable to parse the MOCK_DATA_FILE content");

            println!("{:?}", data_vec);

            let data_option = data_vec.iter().find(|c| c.blog_id == blog_id);
            match data_option {
                Some(data) => Ok(Json(data.to_owned())),
                None => Err(BlogError::NotFound),
            }
        }
    }
}

pub async fn write_blog_thumbsup(
    blog_id: String,
    blog: BlogThumbsupMockRequest,
) -> GetBlogMockResponse {
    println!("\n blog : {:?}", &blog);

    let mut n_path = Path::new(".").join("src");
    n_path.push(env::var("MOCK_DATA_FILE").expect("MOCK_DATA_FILE missing"));
    dbg!(&n_path);
    match File::open(n_path.clone()) {
        Err(why) => panic!("couldn't open MOCK_DATA_FILE {}", why),
        Ok(file) => {
            println!(">>> {:?}", &file);
            let data_str =
                fs::read_to_string(n_path.clone()).expect("Unable to read file MOCK_DATA_FILE");

            let mut data_vec: Vec<BlogRuntimeMockResponse> = serde_json::from_str(&data_str)
                .expect("Unable to parse the MOCK_DATA_FILE content");

            let cur_value_index = data_vec.iter().position(|r| r.blog_id == blog_id).unwrap();

            data_vec[cur_value_index] = blog
                .clone()
                .update_thumbsup(data_vec[cur_value_index].clone());

            fs::write(n_path, serde_json::to_string(&data_vec).unwrap())?;
            Ok(Json(data_vec[cur_value_index].clone()))
        }
    }
}

pub async fn write_blog_thumbsdown(
    blog_id: String,
    blog: BlogThumbsdownMockRequest,
) -> GetBlogMockResponse {
    let mut n_path = Path::new(".").join("src");
    n_path.push(env::var("MOCK_DATA_FILE").expect("MOCK_DATA_FILE missing"));
    dbg!(&n_path);
    match File::open(n_path.clone()) {
        Err(why) => panic!("couldn't open MOCK_DATA_FILE {}", why),
        Ok(file) => {
            println!(">>> {:?}", &file);
            let data_str =
                fs::read_to_string(n_path.clone()).expect("Unable to read file MOCK_DATA_FILE");

            let data_vec: Vec<BlogRuntimeMockResponse> = serde_json::from_str(&data_str)
                .expect("Unable to parse the MOCK_DATA_FILE content");

            let cur_value_index = data_vec.iter().position(|r| r.blog_id == blog_id).unwrap();

            let data_option: Vec<BlogRuntimeMockResponse> = data_vec
                .iter()
                .map(|c| {
                    let x = if c.blog_id == blog_id {
                        blog.clone().update_thumbsdown(c.clone())
                    } else {
                        c.to_owned()
                    };
                    x
                })
                .collect::<Vec<BlogRuntimeMockResponse>>();

            fs::write(n_path, serde_json::to_string(&data_option).unwrap())?;
            Ok(Json(data_vec[cur_value_index].clone()))
        }
    }
}
