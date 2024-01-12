extern crate reqwest;
use std::collections::HashMap;
use serde_json::{json, Value};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

pub struct Zlibrary {
    email: Option<String>,
    name: Option<String>,
    remix_userid: Option<String>,
    remix_userkey: Option<String>,
    domain: String,
    img_download_domains: Vec<String>,
    logged: bool,
    headers: HeaderMap,
    cookies: HashMap<String, String>,
}

impl Zlibrary {
    pub fn new(email: Option<String>, password: Option<String>, remix_userid: Option<String>, remix_userkey: Option<String>) -> Zlibrary {
        let mut zlibrary = Zlibrary {
            email: None,
            name: None,
            remix_userid: None,
            remix_userkey: None,
            domain: "singlelogin.se".to_string(),
            img_download_domains: vec!["z-library.se".to_string(), "zlibrary-in.se".to_string(), "zlibrary-africa.se".to_string()],
            logged: false,
            headers: HeaderMap::new(),
            cookies: HashMap::new(),
        };

        zlibrary.headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
        zlibrary.headers.insert("accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
        zlibrary.headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.9"));
        zlibrary.headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"));

        zlibrary.cookies.insert("siteLanguageV2".to_string(), "en".to_string());

        // if let Some(email) = email {
        //     if let Some(password) = password {
        //         zlibrary.login(email, password);
        //     }
        // } else if let Some(remix_userid) = remix_userid {
        //     if let Some(remix_userkey) = remix_userkey {
        //         zlibrary.login_with_token(remix_userid, remix_userkey);
        //     }
        // }

        zlibrary
    }

    // pub async fn login(&mut self, email: String, password: String) -> Result<Value, reqwest::Error> {
    //     let mut data = HashMap::new();
    //     data.insert("email", email);
    //     data.insert("password", password);
    //     let response: <Result<_, _> as Try>::Output = self.make_post_request("/eapi/user/login", data, true).await?;
    //     Ok(self.set_values(response))
    // }

    // pub async fn login_with_token(&mut self, remix_userid: String, remix_userkey: String) -> Result<Value, reqwest::Error> {
    //     let mut data = HashMap::new();
    //     data.insert("remix_userid", remix_userid);
    //     data.insert("remix_userkey", remix_userkey);
    //     let response = self.make_post_request("/eapi/user/login", data, true).await?;
    //     Ok(self.set_values(response))
    // }

    async fn make_post_request(&self, url: &str, data: HashMap<&str, &str>, is_override: bool) -> Result<Value, reqwest::Error> {
        if !self.logged && !is_override {
            println!("Not logged in");
            return Ok(Value::Null);
        }

        let client = reqwest::Client::new();
        let res = client.post(&format!("https://{}{}", self.domain, url))
            .headers(self.headers.clone())
            .form(&data)
            .send()
            .await?;

        let json: Value = res.json().await?;
        Ok(json)
    }

    // pub fn set_values(&mut self, response: Value) -> Value {
    //     if !response["success"].as_bool().unwrap_or(false) {
    //         return response;
    //     }
    //     self.email = Some(response["user"]["email"].as_str().unwrap_or("").to_string());
    //     self.name = Some(response["user"]["name"].as_str().unwrap_or("").to_string());
    //     self.remix_userid = Some(response["user"]["id"].as_str().unwrap_or("").to_string());
    //     self.remix_userkey = Some(response["user"]["remix_userkey"].as_str().unwrap_or("").to_string());
    //     self.cookies.insert("remix_userid".to_string(), self.remix_userid.clone().unwrap_or_default());
    //     self.cookies.insert("remix_userkey".to_string(), self.remix_userkey.clone().unwrap_or_default());
    //     self.logged = true;
    //     response
    // }


    // async fn make_get_request(&self, url: &str, is_override: bool) -> Result<Value, reqwest::Error> {
    //     if !self.logged && !is_override {
    //         println!("Not logged in");
    //         return Ok(Value::Null);
    //     }

    //     let client = reqwest::Client::new();
    //     let res = client.get(&format!("https://{}{}", self.domain, url))
    //         .headers(self.headers.clone())
    //         .send()
    //         .await?;

    //     let json: Value = res.json().await?;
    //     Ok(json)
    // }

    // pub async fn search(&self, query: String, page: u32) -> Result<Value, reqwest::Error> {
    //     let url = format!("/eapi/search/search?query={}&page={}", query, page);
    //     self.make_get_request(&url, false).await
    // }

    // pub async fn get_book_info(&self, book_id: String) -> Result<Value, reqwest::Error> {
    //     let url = format!("/eapi/book/info?md5={}", book_id);
    //     self.make_get_request(&url, false).await
    // }

    // pub async fn get_image(&self, book: HashMap<String, String>) -> Result<Vec<u8>, reqwest::Error> {
    //     self.get_image_data(book.get("cover").unwrap_or(&"".to_string())).await
    // }

    // async fn get_book_file(&self, book_id: String, hash_id: String) -> Result<(String, Vec<u8>), reqwest::Error> {
    //     let url = format!("/eapi/book/{}/{}file", book_id, hash_id);
    //     let response = self.make_get_request(&url, false).await?;
    //     let mut filename = response["file"]["description"].as_str().unwrap_or("").to_string();
    //     if let Some(author) = response["file"]["author"].as_str() {
    //         filename += &format!(" ({})", author);
    //     }
    //     filename += &format!(".{}", response["file"]["extension"].as_str().unwrap_or(""));
    //     let download_link = response["file"]["downloadLink"].as_str().unwrap_or("");
    //     let client = reqwest::Client::new();
    //     let res = client.get(download_link)
    //         .headers(self.headers.clone())
    //         .send()
    //         .await?;
    //     if res.status().is_success() {
    //         Ok((filename, res.bytes().await?))
    //     } else {
    //         println!("Download failed");
    //         Ok((filename, vec![]))
    //     }
    // }

    // pub async fn download_book(&self, book: HashMap<String, String>) -> Result<(String, Vec<u8>), reqwest::Error> {
    //     self.get_book_file(book.get("id").unwrap_or(&"".to_string()).to_string(), book.get("hash").unwrap_or(&"".to_string()).to_string()).await
    // }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
