use pickledb::{PickleDbDumpPolicy, SerializationMethod, PickleDb};

pub struct PickleUtils;

impl PickleUtils {
    pub fn load_or_new(db_path: &str) -> PickleDb {
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

        db
    }
}
