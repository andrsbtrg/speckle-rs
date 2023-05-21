use reqwest::header::{ACCEPT, CONTENT_TYPE, USER_AGENT};
use serde::Serialize;
use serde_json::Value;
use std::{format, io::Read, println};

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
    pub fn get_raw_root_object(&self) -> String {
        let mut res = self
            .client
            // .get("https://speckle.xyz/api/getobjects/0bacfc3aa6")
            .get(format!(
                "https://speckle.xyz/objects/{}/{}/single",
                &self.stream_id, &self.object_id
            ))
            .bearer_auth(&self.token)
            .header(reqwest::header::ACCEPT, "text/plain")
            .send()
            .unwrap();

        println!("Url: {}", res.url());
        let mut body = String::new();

        res.read_to_string(&mut body).unwrap();

        // debug out
        // println!("Status: {}", res.status());
        // println!("Headers:\n{:#?}", res.headers());
        // println!("Body:\n{}", body);

        body
    }
    pub fn get_raw_object_iterator(&self) -> Vec<String> {
        let root_obj_json = self.get_raw_root_object();
        let root_obj: Value = serde_json::from_str(&root_obj_json).unwrap();

        // Return the ids as a list of strings
        root_obj["__closure"]
            .as_object()
            .unwrap()
            .keys()
            .map(|key| key.to_string())
            .collect()
    }

    pub fn fetch_objects(&self, object_iterator: Vec<String>) {
        let raw_objects_json =
            serde_json::to_string(&object_iterator.iter().take(5).cloned().collect::<Vec<_>>())
                .unwrap();
        println!("{}", &raw_objects_json);

        let request_object = RequestObject {
            objects: raw_objects_json,
        };

        let request_body = serde_json::to_string(&request_object).unwrap();
        println!("{}", request_body);
        let mut res = self
            .client
            .post(format!(
                "https://speckle.xyz/api/getobjects/{}",
                self.stream_id
            ))
            .header(USER_AGENT, "mememe")
            .header(ACCEPT, "text/plain")
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&self.token)
            .body(request_body)
            .send()
            .unwrap();

        println!("{}", res.url());
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        println!("Objects: \n{}", body);
    }
}
