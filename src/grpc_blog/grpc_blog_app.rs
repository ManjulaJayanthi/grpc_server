use std::{
    env,
    fs::{self, File},
    path::Path,
};

use crate::blog::{blog_model::BlogRuntimeMockResponse, blog_response::BlogError};

use super::grpc_blog_route::blog::{
    BlogRuntimeResponse, BlogThumbsdownRequest, BlogThumbsupRequest,
};

pub async fn grpc_get_blog(blog_id: String) -> Result<BlogRuntimeResponse, BlogError> {
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
                Some(data) => Ok(data.to_owned().into_grpc_blog_reponse()),
                None => Err(BlogError::NotFound),
            }
        }
    }
}

pub async fn grpc_blog_thumbsup(
    blog: BlogThumbsupRequest,
) -> Result<BlogRuntimeResponse, BlogError> {
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

            let data_vec: Vec<BlogRuntimeMockResponse> = serde_json::from_str(&data_str)
                .expect("Unable to parse the MOCK_DATA_FILE content");

            let cur_value_index = data_vec
                .clone()
                .iter()
                .position(|r| r.blog_id == blog.blog_id)
                .unwrap();

            let data_option: Vec<BlogRuntimeMockResponse> = data_vec
                .clone()
                .iter()
                .map(|c| {
                    let x = if c.blog_id == blog.blog_id {
                        blog.clone().update_thumbsup(c.clone())
                    } else {
                        c.to_owned()
                    };
                    x
                })
                .collect::<Vec<BlogRuntimeMockResponse>>();
            fs::write(n_path, serde_json::to_string(&data_option).unwrap())?;

            Ok(data_vec[cur_value_index]
                .to_owned()
                .into_grpc_blog_reponse())
            // Ok(Json(data_vec[cur_value_index].clone()))
        }
    }
}

pub async fn grpc_blog_thumbsdown(
    blog: BlogThumbsdownRequest,
) -> Result<BlogRuntimeResponse, BlogError> {
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

            let data_vec: Vec<BlogRuntimeMockResponse> = serde_json::from_str(&data_str)
                .expect("Unable to parse the MOCK_DATA_FILE content");

            let cur_value_index = data_vec
                .clone()
                .iter()
                .position(|r| r.blog_id == blog.blog_id)
                .unwrap();

            let data_option: Vec<BlogRuntimeMockResponse> = data_vec
                .clone()
                .iter()
                .map(|c| {
                    let x = if c.blog_id == blog.blog_id {
                        blog.clone().update_thumbsdown(c.clone())
                    } else {
                        c.to_owned()
                    };
                    x
                })
                .collect::<Vec<BlogRuntimeMockResponse>>();
            fs::write(n_path, serde_json::to_string(&data_option).unwrap())?;

            Ok(data_vec[cur_value_index]
                .to_owned()
                .into_grpc_blog_reponse())
        }
    }
}
