use std::sync::Mutex;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

use super::mapper::ApiKeyMapper;

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
        .unwrap_or_else(|_| {
            PickleDb::new(
                db_path,
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            )
        });

        ApiKeyMapperPickleDb { db: Mutex::new(db) }
    }
}

impl ApiKeyMapper for ApiKeyMapperPickleDb {
    fn find_api_key(&self, user_id: u64) -> Option<String> {
        self.db.lock().unwrap().get(&user_id.to_string())
    }

    // FIXME: return Result instead Unit
    fn create(&self, user_id: u64, api_key: &str) {
        self.db
            .lock()
            .unwrap()
            .set(&user_id.to_string(), &api_key.to_string())
            .unwrap();
    }
}
