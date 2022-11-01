use std::sync::Mutex;

use async_trait::async_trait;
use domain::{users::User, utils::ChainingExt};
use pickledb::PickleDb;

use crate::utils::PickleUtils;

use super::repository::UserRepository;

pub struct UserRepoPickle {
    db: Mutex<PickleDb>,
    id_counter: Mutex<i32>,
}

impl UserRepoPickle {
    pub fn new() -> UserRepoPickle {
        // TODO: move to config
        let db_path = "users_pickle.db";
        let db = PickleUtils::load_or_new(db_path).pipe(Mutex::new);

        let id_counter = Mutex::new(db.lock().unwrap().total_keys() as i32);

        UserRepoPickle { db, id_counter }
    }

    fn next_id(&self) -> i32 {
        let next = *self.id_counter.lock().unwrap() + 1;
        *self.id_counter.lock().unwrap() = next;

        next
    }
}

#[async_trait]
impl UserRepository for UserRepoPickle {
    async fn create(&self, username: &str, password: &str, api_key: &str) -> Result<User, String> {
        let user = User {
            id: self.next_id(),
            username: username.to_owned(),
            password: password.to_owned(),
            api_key: api_key.to_owned(),
        };

        self.db
            .lock()
            .unwrap()
            .set(&user.username, &user)
            .map(|_| user)
            .map_err(|err| err.to_string())
    }

    async fn delete(&self, username: &str) -> Result<(), String> {
        match self.db.lock().unwrap().rem(username) {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("{}", err);
                Err(err.to_string())
            }
        }
    }

    async fn find_all(&self) -> Vec<User> {
        self.db
            .lock()
            .unwrap()
            .iter()
            .map(|v| v.get_value().unwrap())
            .collect()
    }

    async fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        self.db
            .lock()
            .unwrap()
            .iter()
            .map(|v| v.get_value::<User>().unwrap())
            .find(|user| user.api_key == api_key)
    }

    async fn find_all_usernames(&self) -> std::collections::HashSet<String> {
        self.db
            .lock()
            .unwrap()
            .get_all()
            .iter()
            .map(|k| k.to_owned())
            .collect()
    }
}
