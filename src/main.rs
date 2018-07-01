#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
#[macro_use]
extern crate rouille;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod wandbox;
use wandbox::wandbox::{send_code, CompiledResult};
mod command;
use command::command::Command;
mod kokoroio;
use kokoroio::kokoroio::{KokoroClient, KokoroRequestBody};

use std::env;
use std::collections::HashMap;

fn main() {
    let access_token = env::var_os("KOKORO_IO_ACCESS_TOKEN").expect("Required KOKORO_IO_ACCESS_TOKEN").to_str().unwrap().to_string();
    let callback_secret = env::var_os("KOKORO_IO_CALLBACK_SECRET").expect("Requried KOKORO_IO_CALLBACK_SECRET");
    
    let kokoro_client = KokoroClient { access_token };

    let port = match env::var_os("KOKOROIO_WANDBOX_BRIDGE_SERVER_PORT") {
        Some(value) => value.to_str().unwrap().to_string(),
        None => "55301".to_string(),
    };
    rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::text("hi")
            },
            (POST) (/) => {
                match &request.header("Authorization") {
                    Some(x) if x == &callback_secret.to_str().unwrap() => {
                        let body: KokoroRequestBody = rouille::input::json_input(&request).unwrap();
                        match Command::parse(&body.raw_content) {
                            Some(command) => {
                                let result: CompiledResult = send_code(&command).unwrap();
                                let mut payload: HashMap<String, String> = HashMap::new();
                                let message = match &result.status as &str {
                                    "0" => format!("```\n{}```", &result.program_message),
                                    x => format!("status code: {}", x),
                                };
                                payload.insert("message".to_string(), message.to_string());
                                let resp = &kokoro_client.post(&body.channel, &payload);
                                println!("{:?}", &resp);
                                rouille::Response::text("done")
                            },
                            None => rouille::Response::text("cannot parse."),
                        }
                    },
                    _ => rouille::Response::text("Invalid callback secret."),
                }
            },
            _ => rouille::Response::empty_404(),
        )
    });
}