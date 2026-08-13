use rust_data_structures_and_algorithms::{
    binary_search_tree::BinarySearchTree, dynamic_array::DynamicArray, hash_table::HashTable,
    queue::Queue, search, singly_linked_list::SinglyLinkedList, sorting, stack::Stack,
};

#[test]
#[ignore = "enable after Stack implementation"]
fn stack_preserves_lifo_order() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[test]
#[ignore = "enable after Queue implementation"]
fn queue_preserves_fifo_order() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), None);
}

#[test]
#[ignore = "enable after SinglyLinkedList implementation"]
fn linked_list_supports_indexed_operations() {
    let mut list = SinglyLinkedList::new();
    assert_eq!(list.insert(0, 1), Ok(()));
    assert_eq!(list.insert(1, 2), Ok(()));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.remove(0), Some(1));
}

#[test]
#[ignore = "enable after DynamicArray implementation"]
fn dynamic_array_grows_and_returns_owned_removed_values() {
    let mut array = DynamicArray::new();
    assert_eq!(array.insert(0, 1), Ok(()));
    assert_eq!(array.insert(1, 2), Ok(()));
    assert_eq!(array.get(0), Some(&1));
    assert_eq!(array.remove(1), Some(2));
}

#[test]
#[ignore = "enable after HashTable implementation"]
fn hash_table_replaces_equal_keys_and_handles_removal() {
    let mut table = HashTable::new();
    assert_eq!(table.insert("key", 1), None);
    assert_eq!(table.insert("key", 2), Some(1));
    assert_eq!(table.get(&"key"), Some(&2));
    assert_eq!(table.remove(&"key"), Some(2));
}

#[test]
#[ignore = "enable after BinarySearchTree implementation"]
fn binary_search_tree_orders_and_removes_values() {
    let mut tree = BinarySearchTree::new();
    assert_eq!(tree.insert(2), Ok(()));
    assert_eq!(tree.insert(1), Ok(()));
    assert_eq!(tree.insert(3), Ok(()));
    assert_eq!(tree.get(&1), Some(&1));
    assert_eq!(tree.remove(&2), Some(2));
}

#[test]
#[ignore = "enable after linear_search implementation"]
fn linear_search_finds_first_match_and_preserves_absence() {
    assert_eq!(search::linear_search(&[3, 1, 1], &1), Some(1));
    assert_eq!(search::linear_search(&[3, 1, 1], &4), None);
}

#[test]
#[ignore = "enable after binary_search implementation"]
fn binary_search_requires_sorted_input_and_finds_values() {
    assert_eq!(search::binary_search(&[1, 2, 3], &2), Some(1));
    assert_eq!(search::binary_search(&[1, 2, 3], &4), None);
}

#[test]
#[ignore = "enable after sorting implementations"]
fn sorting_algorithms_order_values() {
    let mut values = [3, 1, 2];
    sorting::quick_sort(&mut values);
    assert_eq!(values, [1, 2, 3]);
}
