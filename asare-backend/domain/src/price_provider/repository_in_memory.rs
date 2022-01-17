use std::{collections::HashMap, sync::Mutex};

use crate::{Price, Ticker};

use super::repository::PriceProviderRepo;

pub struct RepoInMemory {
    pub data: Mutex<HashMap<Ticker, Price>>,
}

impl PriceProviderRepo for RepoInMemory {
    fn save(&self, ticker: &Ticker, price: &Price) {
        self.data
            .lock()
            .unwrap()
            .insert(ticker.to_owned(), price.to_owned());
    }

    fn find(&self, ticker: &Ticker) -> Option<Price> {
        self.data.lock().unwrap().get(ticker).map(|p| p.to_owned())
    }
}
