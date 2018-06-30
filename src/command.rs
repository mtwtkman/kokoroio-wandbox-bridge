pub mod command {
    #[derive(Debug)]
    pub struct Command {
        compiler: String,
        code: String,
    }

    use wandbox::wandbox::fetch_compilers;
    use regex::Regex;
    use std::collections::HashMap;

    impl Command {
        pub fn parse(s: &str) -> Option<Command> {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^/wandbox\s+(?P<language>\w+)\s*\n+(?P<code>(?s).+)$").unwrap();
            }        
            RE.captures(s).and_then(|matched| {
                let compilers: HashMap<String, String> = fetch_compilers().unwrap();
                if let Some(compiler) = &compilers.get(&matched["language"].to_lowercase()) {
                    return Some(Command {
                        compiler: compiler.to_string(),
                        code: matched["code"].to_string(),
                    })
                }
                None
            })
        }
        pub fn as_payload(&self) -> HashMap<String, String> {
            let mut result: HashMap<String, String> = HashMap::new();
            result.insert("compiler".to_string(), self.compiler.to_string());
            result.insert("code".to_string(), self.code.to_string());
            result
        }
    }
}