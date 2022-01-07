use passwords::PasswordGenerator;
use serde::Serialize;

static API_KEY_GENERATOR: PasswordGenerator = PasswordGenerator {
    length: 32,
    numbers: true,
    lowercase_letters: true,
    uppercase_letters: true,
    symbols: false,
    spaces: false,
    exclude_similar_characters: false,
    strict: true,
};

#[derive(PartialEq, Debug, Serialize, Clone)]

pub struct ApiKey {
    api_key: String,
}

impl ApiKey {
    pub fn new() -> String {
        let api_key = API_KEY_GENERATOR
            .generate_one()
            .expect("Error generating api_key");

        api_key
    }
}
