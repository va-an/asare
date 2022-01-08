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

pub struct UserPasswordGenerator;

impl UserPasswordGenerator {
    pub fn new() -> String {
        PASSWORD_GENERATOR
            .generate_one()
            .expect("Error generating password user")
    }
}
