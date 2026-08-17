/// FIFO collection scaffold. Implement a growing circular buffer.
pub struct Queue<T> {
    _marker: core::marker::PhantomData<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        todo!("implement Queue::new")
    }
    pub fn enqueue(&mut self, item: T) {
        let _ = item;
        todo!("implement Queue::enqueue")
    }
    pub fn dequeue(&mut self) -> Option<T> {
        todo!("implement Queue::dequeue")
    }
    pub fn len(&self) -> usize {
        todo!("implement Queue::len")
    }
    pub fn is_empty(&self) -> bool {
        todo!("implement Queue::is_empty")
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}
