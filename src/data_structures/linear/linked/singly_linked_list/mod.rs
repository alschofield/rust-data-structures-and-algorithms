/// Head-only singly linked list scaffold.
pub struct SinglyLinkedList<T> {
    _marker: core::marker::PhantomData<T>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        todo!("implement SinglyLinkedList::new")
    }
    pub fn insert(&mut self, index: usize, item: T) -> Result<(), T> {
        let _ = (index, item);
        todo!("implement insert")
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        let _ = index;
        todo!("implement get")
    }
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let _ = index;
        todo!("implement remove")
    }
    pub fn len(&self) -> usize {
        todo!("implement len")
    }
    pub fn is_empty(&self) -> bool {
        todo!("implement is_empty")
    }
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
