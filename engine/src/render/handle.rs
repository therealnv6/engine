use std::{any::TypeId, marker::PhantomData};

pub type HandleId = usize;

pub struct Handle<T> {
    id: HandleId,
    phantom: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: HandleId) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }
}
