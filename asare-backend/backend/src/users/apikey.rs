struct ApiKey;

trait ApiKeyService {
    fn new() -> ApiKey;
    fn find_by_user_id(user_id: &i32); // TODO: move to user module?
}
