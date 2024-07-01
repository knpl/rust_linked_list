use std::{error::Error, fmt};

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug, PartialEq)]
pub struct InsertionError {}

impl Error for InsertionError {}

impl fmt::Display for InsertionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to insert an item into a list")
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        return List { head: None };
    }

    pub fn append(&mut self, item: T) {
        let new_node = Some(Box::new(Node { elem: item, next: None }));

        if let None = self.head {
            // The list is empty.
            self.head = new_node;
            return;
        }

        let mut current = self.head.as_mut().unwrap();
        while let Some(ref mut v) = current.next {
            current = v;
        }
        current.next = new_node;
    }

    pub fn prepend(&mut self, item: T) {
        self.head = Some(Box::new(Node {
            elem: item,
            next: self.head.take(),
        }));
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        let mut current = &self.head;
        for _ in 0..index {
            match current {
                None => return None,
                Some(v) => current = &v.next,
            }
        }
        match current {
            None => None,
            Some(v) => Some(&v.elem),
        }
    }

    pub fn insert(&mut self, index: u32, item: T) -> Result<(), InsertionError> {
        if index == 0 {
            // Same as `prepend`.
            self.head = Some(Box::new(Node {
                elem: item,
                next: self.head.take(),
            }));
            return Ok(());
        }

        if let None = self.head {
            return Err(InsertionError {});
        }

        let mut current = self.head.as_mut().unwrap();
        for _ in 1..index {
            if let None = current.next {
                return Err(InsertionError {});
            }
            current = current.next.as_mut().unwrap();
        }

        current.next = match current.next.take() {
            None => Some(Box::new(Node { elem: item, next: None })),
            Some(to_push) => Some(Box::new(Node {
                elem: item,
                next: Some(to_push),
            })),
        };
        Ok(())
    }

    pub fn remove(&mut self, index: u32) -> Option<T> {
        if let None = self.head {
            // The list is empty.
            return None;
        }

        if index == 0 {
            // Remove the first element.
            let to_remove = self.head.take().unwrap();
            self.head = to_remove.next;
            return Some(to_remove.elem);
        }

        // Find the node 'before' the node to remove.
        let mut current = self.head.as_mut().unwrap();
        for _ in 1..index {
            if let None = current.next {
                // The list is too short.
                return None;
            }
            current = current.next.as_mut().unwrap();
        }

        match current.next.take() {
            None => None,
            Some(to_remove) => {
                current.next = to_remove.next;
                Some(to_remove.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_three_elems_keep_adding_at_the_end() {
        // Arrange
        let mut list: List<i32> = List::new();

        // Act
        for i in 1..=3 {
            list.append(i)
        }

        // Assert
        let first_node = list.head.expect("List empty");
        let second_node = first_node.next.expect("No second node");
        let third_node = second_node.next.expect("No third node");
        assert_eq!(first_node.elem, 1);
        assert_eq!(second_node.elem, 2);
        assert_eq!(third_node.elem, 3);
        assert!(third_node.next.is_none());
    }

    #[test]
    fn prepend_three_elems_keep_adding_at_beginning() {
        // Arrange
        let mut list: List<i32> = List::new();

        // Act
        for i in 1..=3 {
            list.prepend(i);
        }

        // Assert
        let first_node = list.head.expect("List empty");
        let second_node = first_node.next.expect("No second node");
        let third_node = second_node.next.expect("No third node");
        assert_eq!(first_node.elem, 3);
        assert_eq!(second_node.elem, 2);
        assert_eq!(third_node.elem, 1);
        assert!(third_node.next.is_none());
    }

    #[test]
    fn get_from_empty_list_returns_none() {
        // Arrange
        let list: List<i32> = List::new();

        // Act, Assert
        assert!(list.get(0).is_none());
        assert!(list.get(42).is_none());
    }

    #[test]
    fn get_index_out_of_bounds_returns_none() {
        // Arrange
        let mut list: List<i32> = List::new();
        for i in 1..=3 {
            list.append(i);
        }

        // Act, Assert
        assert!(list.get(3).is_none());
        assert!(list.get(42).is_none());
    }

    #[test]
    fn get_index_in_bounds_returns_correct_item() {
        // Arrange
        let mut list: List<i32> = List::new();
        for i in 1..=3 {
            list.append(i);
        }

        // Act
        for i in 0..3 {
            assert_eq!(list.get(i), Some(i as i32 + 1).as_ref());
        }
    }

    #[test]
    fn insert_at_zero_is_the_same_as_prepend() {
        // Arrange
        let mut list: List<i32> = List::new();

        // Act
        for i in 1..=3 {
            list.insert(0, i).expect("Insertion error");
        }

        // Assert
        let first_node = list.head.expect("List empty");
        let second_node = first_node.next.expect("No second node");
        let third_node = second_node.next.expect("No third node");
        assert_eq!(first_node.elem, 3);
        assert_eq!(second_node.elem, 2);
        assert_eq!(third_node.elem, 1);
        assert!(third_node.next.is_none());
    }

    #[test]
    fn insert_at_end_is_the_same_as_append() {
        // Arrange
        let mut list: List<i32> = List::new();

        // Act
        for i in 0..3 {
            list.insert(i, i as i32 + 1).expect("Insertion error");
        }

        // Assert
        let first_node = list.head.expect("List empty");
        let second_node = first_node.next.expect("No second node");
        let third_node = second_node.next.expect("No third node");
        assert_eq!(first_node.elem, 1);
        assert_eq!(second_node.elem, 2);
        assert_eq!(third_node.elem, 3);
        assert!(third_node.next.is_none());
    }

    #[test]
    fn insert_out_of_bounds_in_empty_list_returns_error_result() {
        // Arrange
        let mut list: List<i32> = List::new();

        // Act, Assert
        assert_eq!(list.insert(1, 42).expect_err("No error"), InsertionError {});
        assert_eq!(list.insert(42, 43).expect_err("No error"), InsertionError {});
    }

    #[test]
    fn insert_out_of_bounds_in_non_empty_list_returns_error_result() {
        // Arrange
        let mut list: List<i32> = List::new();
        for i in 1..=3 {
            list.append(i);
        }

        // Act, Assert
        assert_eq!(list.insert(4, 42).expect_err("No error"), InsertionError {});
        assert_eq!(list.insert(42, 43).expect_err("No error"), InsertionError {});
    }

    #[test]
    fn insert_intermediate_item() -> Result<(), InsertionError> {
        // Arrange
        let mut list: List<i32> = List::new();
        list.append(1);
        list.append(3);

        // Act
        list.insert(1, 2)?;

        // Assert
        let first_node = list.head.expect("Empty list");
        let second_node = first_node.next.expect("No second element");
        let third_node = second_node.next.expect("No third element");
        assert_eq!(first_node.elem, 1);
        assert_eq!(second_node.elem, 2);
        assert_eq!(third_node.elem, 3);
        assert!(third_node.next.is_none());
        Ok(())
    }

    #[test]
    fn insert_before_last() -> Result<(), InsertionError> {
        // Arrange
        let mut list: List<i32> = List::new();
        for i in 1..=10 {
            list.append(i);
        }

        // Act
        list.insert(9, 42)?;

        // Assert
        assert_eq!(*list.get(9).unwrap(), 42);
        assert_eq!(*list.get(10).unwrap(), 10);
        Ok(())
    }

    #[test]
    fn remove_on_empty_list_returns_none() {
        // Arrange
        let mut list: List<i32> = List::new();

        // Act, Assert
        assert!(list.remove(0).is_none());
        assert!(list.remove(42).is_none());
    }

    #[test]
    fn remove_index_out_of_bounds_returns_none() {
        // Arrange
        let mut list: List<i32> = List::new();
        for i in 1..=3 {
            list.append(i);
        }

        // Act, Assert
        assert!(list.remove(3).is_none());
        assert!(list.remove(42).is_none());
    }

    #[test]
    fn remove_first_on_list_with_one_element_makes_empty_list() {
        // Arrange
        let mut list: List<i32> = List::new();
        list.append(42);

        // Act
        let result = list.remove(0).expect("Empty list");

        // Assert
        assert_eq!(result, 42);
        assert!(list.head.is_none());
    }

    #[test]
    fn remove_first_on_list_with_two_elements_make_list_with_one_element() {
        // Arrange
        let mut list: List<i32> = List::new();
        list.append(1);
        list.append(2);

        // Act
        let result = list.remove(0).expect("No first element");

        // Assert
        assert_eq!(result, 1);
        assert!(list.head.expect("Empty list").next.is_none());
    }

    #[test]
    fn remove_second_on_list_with_two_elements_make_list_with_one_element() {
        // Arrange
        let mut list: List<i32> = List::new();
        list.append(1);
        list.append(2);

        // Act
        let result = list.remove(1).expect("No second element");

        // Assert
        assert_eq!(result, 2);
        assert!(list.head.expect("Empty list").next.is_none());
    }

    #[test]
    fn remove_intermediate_item_on_list_link_prev_to_next() {
        // Arrange
        let mut list: List<i32> = List::new();
        for i in 1..=3 {
            list.append(i);
        }

        // Act
        let result = list.remove(1).expect("No second element");

        // Assert
        assert_eq!(result, 2);
        let first_node = list.head.expect("Empty list");
        let second_node = first_node.next.expect("No second element");
        assert_eq!(first_node.elem, 1);
        assert_eq!(second_node.elem, 3);
        assert!(second_node.next.is_none());
    }
}
