// Copyright (c) 2023 herrsmitty8128
// Distributed under the MIT software license, see the accompanying
// file LICENSE.txt or http://www.opensource.org/licenses/mit-license.php.

use std::cmp::{Ord, Ordering};
use std::fmt::Display;

/// An enum containing the types of errors that a heap might encounter.
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    InvalidIndex,
    EmptyHeap,
}

impl Display for ErrorKind {
    /// Displays the text string associated with an ErrorKind.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorKind::InvalidIndex => f.write_str("Index out of bounds."),
            ErrorKind::EmptyHeap => f.write_str("Heap is empty."),
        }
    }
}

/// The error type used by a heap.
#[derive(Debug, Copy, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: &'static str,
}

impl Display for Error {
    /// Displays both the text string associated with an ErrorKind and the error's message string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.kind, self.message))
    }
}

impl Error {
    /// Creates and returns a new Error object containing the ErrorKind and message string.
    pub fn new(kind: ErrorKind, message: &'static str) -> Self {
        Error { kind, message }
    }
}

impl std::error::Error for Error {}

/// A specialized result type to make error handling simpler.
pub type Result<T> = std::result::Result<T, Error>;

/// A complete binary tree in which the value of each node in the tree is
/// less than the value of each of its children. As a consequence, the smallest
/// value in the tree is always located at the root of the tree.
#[derive(Debug, Clone)]
pub struct Heap<T, const MAX_HEAP: bool, const BRANCHES: usize = 2>
where
    T: Ord + Eq + Copy,
{
    heap: Vec<T>,
    sort_order: Ordering,
}

impl<T, const MAX_HEAP: bool, const BRANCHES: usize> From<&[T]> for Heap<T, MAX_HEAP, BRANCHES>
where
    T: Ord + Eq + Copy,
{
    fn from(arr: &[T]) -> Self {
        let mut heap: Vec<T> = Vec::from(arr);
        let sort_order: Ordering = if MAX_HEAP {
            Ordering::Greater
        } else {
            Ordering::Less
        };
        Self::heap_sort(&mut heap, sort_order);
        Self { heap, sort_order }
    }
}

impl<T, const MAX_HEAP: bool, const BRANCHES: usize> Heap<T, MAX_HEAP, BRANCHES>
where
    T: Ord + Eq + Copy,
{
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            sort_order: if MAX_HEAP == MAX_HEAP {
                Ordering::Greater
            } else {
                Ordering::Less
            },
        }
    }

    /// Clears the heap, removing all elements.
    /// Note that this method has no effect on the allocated capacity of the heap.
    pub fn clear(&mut self) {
        self.heap.clear()
    }

    /// Performs a linear search to find the index of an element on the heap.
    /// Returns *None* if the element was not found.
    ///
    /// ## Example:
    ///
    /// ```
    /// use rheap::Heap;
    ///
    /// let mut v: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
    /// let mut heap: Heap<usize, false, 2> = Heap::from(&v[..]);
    ///
    /// if let Some(index) = heap.find(&6) {
    ///     assert!(index == 3);
    /// } else {
    ///     panic!();
    /// }
    /// ```
    pub fn find(&self, element: &T) -> Option<usize> {
        (0..self.heap.len()).find(|&i| self.heap[i] == *element)
    }

    /// Inserts an element into the heap.
    ///
    /// ## Example:
    ///
    /// ```
    /// use rheap::Heap;
    /// use std::cmp::Ordering;
    ///
    /// let mut v: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
    /// let mut heap: Heap<usize, false, 2> = Heap::from(&v[..]);
    /// println!("{:?}", heap);
    /// heap.insert(5);
    /// if let Some(x) = heap.peek() {
    ///     assert!(*x == 0)
    /// } else {
    ///     panic!()
    /// }
    /// ```
    pub fn insert(&mut self, element: T) {
        let index: usize = self.heap.len();
        self.heap.push(element);
        Self::sort_up(&mut self.heap, self.sort_order, index)
    }

    /// Returns true if the heap contains no elements.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns the number of elements in the heap, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Returns a reference to the element on top of the heap without removing it.
    pub fn peek(&self) -> Option<&T> {
        if self.heap.is_empty() {
            None
        } else {
            Some(&self.heap[0])
        }
    }

    /// Removes and returns the element at *index*.
    /// Returns an error if the heap is empty or if the index is out of bounds.
    ///
    /// ## Example:
    ///
    /// ```
    /// use rheap::Heap;
    /// use std::cmp::Ordering;
    ///
    /// let mut v: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
    /// let mut heap: Heap<usize, false, 2> = Heap::from(&v[..]);
    /// if let Ok(old_element) = heap.remove(3) {
    ///     assert!(old_element == 6);
    /// } else {
    ///     panic!();
    /// }
    /// ```
    pub fn remove(&mut self, index: usize) -> Result<T> {
        if self.heap.is_empty() {
            Err(Error::new(
                ErrorKind::EmptyHeap,
                "Can not remove elements from an empty heap.",
            ))
        } else if index >= self.heap.len() {
            Err(Error::new(
                ErrorKind::InvalidIndex,
                "Index is beyond the end of the heap.",
            ))
        } else {
            let removed: T = self.heap.swap_remove(index);
            if index < self.heap.len() {
                if self.heap[index].cmp(&removed) == self.sort_order {
                    Self::sort_up(&mut self.heap, self.sort_order, index);
                } else {
                    Self::sort_down(&mut self.heap, self.sort_order, index);
                }
            }
            Ok(removed)
        }
    }

    /// Removes and returns the element from the top of the heap. Returns *None* if the heap is empty.
    ///
    /// ## Example:
    ///
    /// ```
    /// use rheap::Heap;
    /// use std::cmp::Ordering;
    ///
    /// let mut v: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
    /// let mut heap: Heap<usize, false, 2> = Heap::from(&v[..]);
    /// if let Some(smallest) = heap.top() {
    ///     assert!(smallest == 0);
    /// } else {
    ///     panic!();
    /// }
    /// ```
    pub fn top(&mut self) -> Option<T> {
        self.remove(0).ok()
    }

    /// Updates the value of the element at *index*.
    /// Returns and error if the element is not found in the heap or the index is out of bounds.
    ///
    /// ## Example:
    ///
    /// ```
    /// use rheap::Heap;
    /// use std::cmp::Ordering;
    ///
    /// let mut v: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
    /// let mut heap: Heap<usize, false> = Heap::from(&v[..]);
    /// if heap.update(3, |x| *x = 11).is_err() {
    ///     panic!();
    /// }
    /// ```
    #[inline]
    pub fn update<F>(&mut self, index: usize, update_func: F) -> Result<()>
    where
        F: Fn(&mut T),
    {
        if self.heap.is_empty() {
            Err(Error::new(
                ErrorKind::EmptyHeap,
                "Can not remove elements from an empty heap.",
            ))
        } else if index >= self.heap.len() {
            Err(Error::new(
                ErrorKind::InvalidIndex,
                "Index is beyond the end of the heap.",
            ))
        } else {
            update_func(&mut self.heap[index]);
            if index == 0
                || self.heap[index].cmp(&self.heap[(index - 1) / BRANCHES]) != self.sort_order
            {
                Self::sort_down(&mut self.heap, self.sort_order, index);
            } else {
                Self::sort_up(&mut self.heap, self.sort_order, index);
            }
            Ok(())
        }
    }

    /// Sorts the heap by iterating down the tree starting at *index*.
    ///
    /// ## Panics:
    ///
    /// Panics if *index* is out of bounds.
    ///
    /// ## Example:
    ///
    /// ```
    /// use rheap::Heap;
    /// use std::cmp::Ordering;
    ///
    /// let mut heap: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
    /// let index: usize = 0;
    /// // remove the element located at index
    /// heap.swap_remove(index);
    /// Heap::<usize, false>::sort_down(&mut heap, Ordering::Less, index);
    /// assert!(heap[0] == 1);
    /// ```
    pub fn sort_down(heap: &mut [T], sort_order: Ordering, mut index: usize)
    where
        T: Ord,
    {
        let length: usize = heap.len();
        loop {
            let first_child: usize = (index * BRANCHES) + 1;
            let last_child: usize = first_child + BRANCHES;
            let mut priority: usize = index;
            for i in first_child..last_child.min(length) {
                priority = if heap[priority].cmp(&heap[i]) == sort_order {
                    priority
                } else {
                    i
                }
            }
            if priority == index {
                break;
            }
            heap.swap(priority, index);
            index = priority
        }
    }

    /// Sorts the heap by iterating up the tree starting at *index*.
    ///
    /// ## Panics
    ///
    /// Panics if *index* is out of bounds.
    ///
    /// ## Example:
    ///
    /// ```
    /// use rheap::Heap;
    /// use std::cmp::Ordering;
    ///
    /// let mut heap: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
    /// let index: usize = heap.len();
    /// heap.push(5);
    /// Heap::<usize, false>::sort_up(&mut heap, Ordering::Less, index);
    /// assert!(heap[0] == 0);
    /// ```
    pub fn sort_up(heap: &mut [T], sort_order: Ordering, mut index: usize)
    where
        T: Ord,
    {
        while index > 0 {
            let p: usize = (index - 1) / BRANCHES; // calculate the index of the parent node
            if heap[index].cmp(&heap[p]) == sort_order {
                heap.swap(index, p); // if the child is smaller than the parent, then swap them
            } else {
                break;
            }
            index = p;
        }
    }

    /// Performs an in-place heap sort.
    ///
    /// ## Example:
    ///
    /// ```
    /// use rheap::Heap;
    /// use std::cmp::Ordering;
    ///
    /// let mut heap: Vec<usize> = vec![8, 66, 9, 55, 7, 0, 14, 6, 37, 2];
    /// Heap::<usize, false>::heap_sort(&mut heap, Ordering::Less);
    /// assert!(heap[0] == 0);
    /// ```
    pub fn heap_sort(heap: &mut [T], sort_order: Ordering)
    where
        T: Ord,
    {
        let len: usize = heap.len();
        if len > 1 {
            let parent: usize = (len - 2) / BRANCHES;
            for index in (0..=parent).rev() {
                Self::sort_down(heap, sort_order, index);
            }
        }
    }

    /// This function is intended for use during testing.
    ///
    /// ## Example:
    ///
    /// ```
    /// use rheap::Heap;
    /// use std::cmp::Ordering;
    ///
    /// let mut v: Vec<usize> = Vec::new();
    /// let mut heap: Heap<usize, false> = Heap::from(&v[..]);
    /// assert!(heap.is_valid());
    /// ```
    #[doc(hidden)]
    pub fn is_valid(&self) -> bool {
        for i in 1..self.heap.len() {
            if self.heap[0].cmp(&self.heap[i]) != self.sort_order {
                return false;
            }
        }
        true
    }
}
