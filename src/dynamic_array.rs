/// Resizable contiguous collection scaffold.
pub struct DynamicArray<T> {
    _marker: core::marker::PhantomData<T>,
}

impl<T> DynamicArray<T> {
    pub fn new() -> Self {
        todo!("implement DynamicArray::new")
    }
    pub fn insert(&mut self, index: usize, item: T) -> Result<(), T> {
        let _ = (index, item);
        todo!("implement insert")
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        let _ = index;
        todo!("implement get")
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let _ = index;
        todo!("implement get_mut")
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

impl<T> Default for DynamicArray<T> {
    fn default() -> Self {
        Self::new()
    }
}
