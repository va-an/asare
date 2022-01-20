use std::{collections::HashMap, sync::Mutex};

use chrono::Utc;

use crate::{Price, Ticker};

use super::repository::{PriceCache, PriceProviderRepo};

pub struct RepoInMemory {
    pub data: Mutex<HashMap<Ticker, PriceCache>>,
}

impl PriceProviderRepo for RepoInMemory {
    fn save(&self, ticker: &Ticker, price: &Price) {
        let price_cache = PriceCache {
            price: price.to_owned(),
            saved_timestamp: Utc::now().timestamp(),
        };

        self.data
            .lock()
            .unwrap()
            .insert(ticker.to_owned(), price_cache);
    }

    fn find(&self, ticker: &Ticker) -> Option<PriceCache> {
        self.data
            .lock()
            .unwrap()
            .get(ticker)
            .map(|price_cache| PriceCache {
                price: price_cache.price,
                saved_timestamp: price_cache.saved_timestamp,
            })
    }
}
