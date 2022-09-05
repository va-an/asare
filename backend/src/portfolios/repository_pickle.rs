use std::sync::Mutex;

use async_trait::async_trait;
use domain::utils::ChainingExt;
use pickledb::PickleDb;

use crate::utils::PickleUtils;

use super::{repository::PortfolioRepository, service::UserPortfolio};

#[allow(dead_code)]
pub struct PortfolioRepoPickle {
    db: Mutex<PickleDb>,
    id_counter: Mutex<i32>,
}

#[allow(dead_code)]
impl PortfolioRepoPickle {
    fn new() -> PortfolioRepoPickle {
        let db_path = "portfolios_pickle.db";
        let db = PickleUtils::load_or_new(db_path).pipe(Mutex::new);

        let id_counter = Mutex::new(db.lock().unwrap().total_keys() as i32);

        PortfolioRepoPickle { db, id_counter }
    }
}

#[async_trait]
impl PortfolioRepository for PortfolioRepoPickle {
    async fn create(&self, _portfolio: &UserPortfolio) -> UserPortfolio {
        todo!()
    }

    async fn find_by_id(&self, _id: &i32) -> UserPortfolio {
        todo!()
    }

    async fn find_by_user(&self, user_id: &i32) -> Vec<UserPortfolio> {
        self.db
            .lock()
            .unwrap()
            .iter()
            .map(|v| v.get_value::<UserPortfolio>().unwrap())
            .filter(|port| &port.user_id == user_id)
            .collect()
    }

    async fn delete_by_id(&self, _id: &i32) {
        todo!()
    }
}
