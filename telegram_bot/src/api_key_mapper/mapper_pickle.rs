use std::sync::Mutex;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

use super::mapper::{ApiKeyMapper, UserId};

pub struct ApiKeyMapperPickleDb {
    db: Mutex<PickleDb>,
}

impl ApiKeyMapperPickleDb {
    pub fn new() -> Self {
        // TODO: move to config
        let db_path = "api_key_map_pickle.db";

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

        ApiKeyMapperPickleDb { db: Mutex::new(db) }
    }
}

impl ApiKeyMapper for ApiKeyMapperPickleDb {
    fn find_api_key(&self, user_id: UserId) -> Option<String> {
        self.db.lock().unwrap().get(&user_id.to_string())
    }

    // FIXME: return Result instead Unit
    fn create(&self, user_id: UserId, api_key: &str) {
        self.db
            .lock()
            .unwrap()
            .set(&user_id.to_string(), &api_key.to_string())
            .unwrap();
    }
}
