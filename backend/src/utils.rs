use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub struct PickleUtils;

impl PickleUtils {
    pub fn load_or_new(db_path: &str) -> PickleDb {
        PickleDb::load(
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
        })
    }
}
