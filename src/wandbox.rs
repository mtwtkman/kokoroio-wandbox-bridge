pub mod wandbox {
    const API: &'static str = "https://wandbox.org/api/";

    #[derive(Deserialize, Debug)]
    struct Compiler {
        pub name: String,
        pub language: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct CompiledResult {
        pub status: String,
        pub compiler_message: Option<String>,
        pub program_message: String,
    }

    use reqwest::{Result, get, Client};
    use std::collections::HashMap;
    use Command;

    pub fn fetch_compilers() -> Result<HashMap<String, String>> {
        let json: Vec<Compiler> = get(&format!("{}list.json", API))?.json()?;
        let mut lang_map: HashMap<String, String> = HashMap::new();
        for row in json.into_iter() {
            let lang = &row.language.to_lowercase();
            if !lang_map.contains_key(lang) {
                lang_map.insert(lang.to_string(), row.name);
            }
        }
        Ok(lang_map)
    }

    pub fn send_code(command: &Command) -> Result<CompiledResult> {
        let client = Client::new();
        client.post(&format!("{}compile.json", API))
            .json(&command.as_payload())
            .send()?
            .json()
    }
}