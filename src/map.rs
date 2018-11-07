use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use types::{Day, Runner};

pub struct Map {
    inner: RefCell<HashMap<Day, Runner>>,
}

impl Map {
    pub(crate) fn new() -> Map {
        Map {
            inner: RefCell::new(HashMap::new()),
        }
    }

    pub(crate) fn borrow(&self) -> Ref<HashMap<Day, Runner>> {
        self.inner.borrow()
    }

    pub(crate) fn borrow_mut(&self) -> RefMut<HashMap<Day, Runner>> {
        self.inner.borrow_mut()
    }
}
