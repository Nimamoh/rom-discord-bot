/**
Attempt to cache future. Replace with outdatable
*/

use std::time::{Duration, Instant};

pub trait Cache {
    type Item;

    fn cached_or<F>(&mut self, f: F) -> Self::Item
        where F: Fn() -> Self::Item;

    fn set_cache(&mut self, item: Self::Item) -> Self::Item;
}

pub struct TtlCache<T> {
    ttl: u64,
    last: Instant,

    cached: Option<T>,
}

impl <T> TtlCache<T> {
    fn new(ttl: u64) -> TtlCache<T> {
        TtlCache { ttl: ttl, last: Instant::now(), cached: None }
    }
}

impl <T> Cache for TtlCache<T> {
    type Item = T;

    fn cached_or<F>(&mut self, f: F) -> Self::Item where F: Fn() -> Self::Item {

        //Refresh the cache
        if self.cached == None || self.last.elapsed().as_secs() > ttl {
            self.cached = f();
            self.last = Instant::now();
        }

        self.cached
    }

    fn set_cache(&mut self, item: Self::Item) -> Self::Item {
        self.cached = item;
        self.last = Instant::now();

        self.cached
    }
}
