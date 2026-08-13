/// Returns the first equal index in arbitrary input.
pub fn linear_search<T: PartialEq>(items: &[T], key: &T) -> Option<usize> {
    let _ = (items, key);
    todo!("implement linear_search")
}

/// Returns an equal index in ascending `Ord` input.
pub fn binary_search<T: Ord>(items: &[T], key: &T) -> Option<usize> {
    let _ = (items, key);
    todo!("implement binary_search")
}
