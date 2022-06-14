use std::sync::Mutex;

use domain::users::User;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

use super::repository::UserRepository;

pub struct UserRepoPickle {
    db: Mutex<PickleDb>,
    id_counter: Mutex<i32>,
}

impl UserRepoPickle {
    pub fn new() -> UserRepoPickle {
        // TODO: move to config
        let db_path = "users_pickle.db";

        let db = PickleDb::load(
            db_path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
        .unwrap_or(PickleDb::new(
            db_path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        ));

        UserRepoPickle {
            db: Mutex::new(db),
            id_counter: Mutex::new(0),
        }
    }

    fn next_id(&self) -> i32 {
        let next = *self.id_counter.lock().unwrap() + 1;
        *self.id_counter.lock().unwrap() = next;

        next
    }
}

impl UserRepository for UserRepoPickle {
    // FIXME: check uniq username in user_service
    fn create(&self, login: &str, password: &str, api_key: &str) -> Result<User, String> {
        let user = User {
            id: self.next_id(),
            username: login.to_owned(),
            password: password.to_owned(),
            api_key: api_key.to_owned(),
        };

        self.db
            .lock()
            .unwrap()
            .set(&user.id.to_string(), &user)
            .map(|_| user)
            .map_err(|err| err.to_string())
    }

    fn delete(&self, id: &i32) {
        todo!()
    }

    fn find_all(&self) -> Vec<User> {
        self.db
            .lock()
            .unwrap()
            .iter()
            .map(|u| u.get_value().unwrap())
            .collect()
    }

    fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        todo!()
    }
}
