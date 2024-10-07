use std::convert::TryInto;

const MAX_SIZE: usize = 10_usize.pow(6) + 1;
struct MyHashMap {
    store: Box<[Option<i32>; MAX_SIZE]>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            store: vec![None; MAX_SIZE].into_boxed_slice().try_into().unwrap()
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(val) = self.store.get_mut(key as usize) {
            *val = Some(value);
        }
    }

    fn get(&self, key: i32) -> i32 {
        self.store
            .get(key as usize)
            .unwrap_or(&Some(-1))
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        if let Some(val) = self.store.get_mut(key as usize) {
            *val = None;
        }
    }
}
