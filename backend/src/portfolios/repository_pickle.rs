use std::sync::Mutex;

use domain::utils::ChainingExt;
use pickledb::PickleDb;

use crate::utils::PickleUtils;

use super::{repository::PortfolioRepository, service::UserPortfolio};

pub struct PortfolioRepoPickle {
    db: Mutex<PickleDb>,
    id_counter: Mutex<i32>,
}

impl PortfolioRepoPickle {
    fn new() -> PortfolioRepoPickle {
        let db_path = "portfolios_pickle.db";
        let db = PickleUtils::load_or_new(db_path).pipe(Mutex::new);

        let id_counter = Mutex::new(db.lock().unwrap().total_keys() as i32);

        PortfolioRepoPickle { db, id_counter }
    }
}

impl PortfolioRepository for PortfolioRepoPickle {
    fn create(&self, _portfolio: &UserPortfolio) -> UserPortfolio {
        todo!()
    }

    fn find_by_id(&self, _id: &i32) -> UserPortfolio {
        todo!()
    }

    fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio> {
        self.db
            .lock()
            .unwrap()
            .iter()
            .map(|v| v.get_value::<UserPortfolio>().unwrap())
            .filter(|port| &port.user_id == user_id)
            .collect()
    }

    fn delete_by_id(&self, id: &i32) {
        todo!()
    }
}
