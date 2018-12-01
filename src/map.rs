use aoc_runner_internal::DayPart;
use crate::types::Runner;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::collections::HashMap;

pub(crate) type InnerMap = HashMap<DayPart, Runner>;

#[derive(Debug, Copy, Clone)]
pub enum MapError {
    AlreadyConsumed,
}

pub struct Map {
    inner: RefCell<InnerMap>,
    consumed: Cell<bool>,
}

impl Map {
    pub(crate) fn new() -> Map {
        Map {
            inner: RefCell::new(HashMap::new()),
            consumed: Cell::new(false),
        }
    }

    pub(crate) fn consume(&self) -> Result<Ref<InnerMap>, MapError> {
        if self.consumed.replace(true) {
            Err(MapError::AlreadyConsumed)
        } else {
            Ok(self.inner.borrow())
        }
    }

    pub(crate) fn borrow_mut(&self) -> Result<RefMut<InnerMap>, MapError> {
        if self.consumed.take() {
            Err(MapError::AlreadyConsumed)
        } else {
            Ok(self.inner.borrow_mut())
        }
    }
}
