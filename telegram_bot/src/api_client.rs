use domain::users::CreateUserRequest;

static USERS_URL: &str = "http://localhost:8072/v1/users/"; // FIXME: localhost

pub async fn create_user(login: &str) -> reqwest::Response {
    let create_user_request = CreateUserRequest {
        username: login.to_string(),
        password: None,
    };

    let client = reqwest::Client::new(); // TODO: create and share?

    let response = client
        .post(USERS_URL)
        .json(&create_user_request)
        .send()
        .await
        .expect("error with create user request");

    response
}
