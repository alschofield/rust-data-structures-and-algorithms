/// Unbalanced ordered collection scaffold.
pub struct BinarySearchTree<T> {
    _marker: core::marker::PhantomData<T>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        todo!("implement BinarySearchTree::new")
    }
    pub fn insert(&mut self, item: T) -> Result<(), T> {
        let _ = item;
        todo!("implement insert")
    }
    pub fn get(&self, key: &T) -> Option<&T> {
        let _ = key;
        todo!("implement get")
    }
    pub fn remove(&mut self, key: &T) -> Option<T> {
        let _ = key;
        todo!("implement remove")
    }
    pub fn contains(&self, key: &T) -> bool {
        let _ = key;
        todo!("implement contains")
    }
    pub fn len(&self) -> usize {
        todo!("implement len")
    }
    pub fn is_empty(&self) -> bool {
        todo!("implement is_empty")
    }
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
