use passwords::PasswordGenerator;

static PASSWORD_GENERATOR: PasswordGenerator = PasswordGenerator {
    length: 12,
    numbers: true,
    lowercase_letters: true,
    uppercase_letters: true,
    symbols: true,
    spaces: false,
    exclude_similar_characters: true,
    strict: true,
};

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

pub struct UserPasswordGenerator;

impl UserPasswordGenerator {
    pub fn generate() -> String {
        PASSWORD_GENERATOR
            .generate_one()
            .expect("Error generating password user")
    }
}

pub struct ApiKeyGenerator;

impl ApiKeyGenerator {
    pub fn generate() -> String {
        let api_key = API_KEY_GENERATOR
            .generate_one()
            .expect("Error generating api_key");

        api_key
    }
}
