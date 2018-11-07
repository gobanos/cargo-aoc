use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use types::{Day, Part, Runner};

pub struct Map {
    inner: RefCell<HashMap<(Day, Part), Runner>>,
}

impl Map {
    pub(crate) fn new() -> Map {
        Map {
            inner: RefCell::new(HashMap::new()),
        }
    }

    pub(crate) fn borrow(&self) -> Ref<HashMap<(Day, Part), Runner>> {
        self.inner.borrow()
    }

    pub(crate) fn borrow_mut(&self) -> RefMut<HashMap<(Day, Part), Runner>> {
        self.inner.borrow_mut()
    }
}
