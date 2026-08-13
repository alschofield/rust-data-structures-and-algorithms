/// LIFO collection scaffold. Implement without delegating to `Vec` methods.
pub struct Stack<T> {
    _marker: core::marker::PhantomData<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!("implement Stack::new")
    }
    pub fn push(&mut self, item: T) {
        let _ = item;
        todo!("implement Stack::push")
    }
    pub fn pop(&mut self) -> Option<T> {
        todo!("implement Stack::pop")
    }
    pub fn len(&self) -> usize {
        todo!("implement Stack::len")
    }
    pub fn is_empty(&self) -> bool {
        todo!("implement Stack::is_empty")
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
