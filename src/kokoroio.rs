pub mod kokoroio {
    #[derive(Deserialize)]
    pub struct Channel {
        pub id: String,
    }

    #[derive(Deserialize)]
    pub struct KokoroRequestBody {
        pub raw_content: String,
        pub channel: Channel,
    }

    pub struct KokoroClient {
        pub access_token: String,
    }

    use reqwest::header::ContentType;
    use std::collections::HashMap;
    use reqwest::{Result, Response, Client};

    header! { (XAccessToken, "X-Access-Token") => [String] }

    impl KokoroClient {
        pub fn post(&self, channel: &Channel, payload: &HashMap<String, String>) -> Result<Response> {
            let client = Client::new();
            client.post(&format!("https://kokoro.io/api/v1/bot/channels/{}/messages", channel.id))
                .header(XAccessToken(self.access_token.to_string()))
                .header(ContentType::json())
                .json(payload)
                .send()
        }
    }
}