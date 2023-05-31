pub mod graphql;
use reqwest::{
    blocking::Response,
    header::{ACCEPT, CONTENT_TYPE, USER_AGENT},
    StatusCode,
};
use serde::Serialize;
use serde_json::Value;
use std::{
    format,
    fs::File,
    io::{Read, Write},
    println,
};

pub struct ObjectLoader {
    stream_id: String,
    object_id: String,
    token: String,
    client: reqwest::blocking::Client,
}
#[derive(Serialize)]
struct RequestObject {
    objects: String,
}

impl ObjectLoader {
    pub fn new(stream_id: &str, object_id: &str, token: &str) -> Self {
        ObjectLoader {
            stream_id: stream_id.to_string(),
            object_id: object_id.to_string(),
            token: token.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }
    fn get_raw_root_object(&self) -> Option<Response> {
        println!("Fetching root object.");
        match self
            .client
            .get(format!(
                "https://speckle.xyz/objects/{}/{}/single",
                &self.stream_id, &self.object_id
            ))
            .bearer_auth(&self.token)
            .header(reqwest::header::ACCEPT, "text/plain")
            .send()
        {
            Ok(res) => {
                println!("Url: {}", res.url());
                println!("Status: {}", res.status());
                match res.status() {
                    StatusCode::OK => Some(res),
                    _ => None,
                }
            }
            Err(_) => None,
        }
    }
    pub fn get_raw_object_iterator(&self) -> Option<Vec<String>> {
        let mut root_obj_json = String::new();

        match self.get_raw_root_object() {
            Some(mut res) => match res.status() {
                StatusCode::OK => res
                    .read_to_string(&mut root_obj_json)
                    .expect("Response contained invalid UTF-8."),
                _ => return None,
            },
            None => return None,
        };
        // debug write
        let mut output = File::create("root.json").unwrap();

        write!(output, "{}", &root_obj_json).unwrap();
        let root_obj: Value = serde_json::from_str(&root_obj_json).unwrap();

        // Return the ids as a list of strings
        let id_list = root_obj["__closure"]
            .as_object()?
            .keys()
            .map(|key| key.to_string())
            .collect();
        Some(id_list)
    }

    pub fn fetch_objects(&self, object_iterator: Vec<String>) -> Result<Response, reqwest::Error> {
        let raw_objects_json = serde_json::to_string(&object_iterator).unwrap();
        println!("Fetching all child objects: \n{}", &raw_objects_json);

        let request_object = RequestObject {
            objects: raw_objects_json,
        };

        let request_body = serde_json::to_string(&request_object).unwrap();
        let res = self
            .client
            .post(format!(
                "https://speckle.xyz/api/getobjects/{}",
                self.stream_id
            ))
            .header(USER_AGENT, "mememe") // fails without user agent
            .header(ACCEPT, "application.json")
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&self.token)
            .body(request_body)
            .send()?;

        println!("Url: {}", res.url());
        println!("Status: {}", res.status());

        Ok(res)
    }
    pub fn store_response(&self, mut res: Response) -> Result<(), std::io::Error> {
        let save_path: &str = "response.json";
        println!("Storing response in ./{}", save_path);

        let mut body = String::new();
        res.read_to_string(&mut body)?;
        let mut output = File::create(save_path)?;

        write!(output, "{}", body)?;
        Ok(())
    }
}
