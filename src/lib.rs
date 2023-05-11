// Copyright (c) 2023 herrsmitty8128
// Distributed under the MIT software license, see the accompanying
// file LICENSE.txt or http://www.opensource.org/licenses/mit-license.php.

use std::cmp::{Ord, Ordering};
use std::error;
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

impl error::Error for Error {}

/// A specialized result type to make error handling simpler.
pub type Result<T> = std::result::Result<T, Error>;

/// An enum used to indicate whether a heap is a minimum or maximum heap.
/// It is your responsibility use the same HeapType when calling different functions for the same heap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapType {
    MinHeap = Ordering::Less as isize,
    MaxHeap = Ordering::Greater as isize,
}

/// Sorts the heap by iterating down the tree starting from index p.
///
/// ## Panics:
///
/// Panics if *index* is out of bounds.
///
/// ## Example:
///
/// ```
/// use rheap::{HeapType, sort_down};
///
/// let mut heap: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
/// let index: usize = 0;
/// // remove the element located at index
/// heap.swap_remove(index);
/// sort_down(&mut heap, HeapType::MinHeap, index);
/// assert!(heap[0] == 1);
/// ```
pub fn sort_down<T>(heap: &mut [T], heap_type: HeapType, mut index: usize)
where
    T: Ord,
{
    let length: usize = heap.len();
    loop {
        let left: usize = (index * 2) + 1;
        let right: usize = left + 1;
        let mut x: usize =
            if left < length && heap[left].cmp(&heap[index]) as isize == heap_type as isize {
                left
            } else {
                index
            };
        if right < length && heap[right].cmp(&heap[x]) as isize == heap_type as isize {
            x = right;
        }
        if x == index {
            break;
        }
        heap.swap(index, x);
        index = x;
    }
}

/// Sorts the heap by iterating up the tree starting from *index*.
///
/// ## Example:
///
/// ```
/// use rheap::{HeapType, sort_up};
///
/// let mut heap: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
/// let index: usize = heap.len();
/// heap.push(5);
/// sort_up(&mut heap, HeapType::MinHeap, index);
/// assert!(heap[0] == 0);
/// ```
pub fn sort_up<T>(heap: &mut [T], heap_type: HeapType, mut index: usize)
where
    T: Ord,
{
    while index > 0 {
        let p: usize = (index - 1) >> 1; // calculate the index of the parent node
        if heap[index].cmp(&heap[p]) as isize == heap_type as isize {
            heap.swap(index, p); // if the child is smaller than the parent, then swap them
        } else {
            break;
        }
        index = p;
    }
}

/// Inserts element into the heap.
///
/// ## Example:
///
/// ```
/// use rheap::{HeapType, insert};
///
/// let mut heap: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
/// insert(&mut heap, HeapType::MinHeap, 5);
/// assert!(heap[0] == 0);
/// ```
pub fn insert<T>(heap: &mut Vec<T>, heap_type: HeapType, element: T)
where
    T: Ord,
{
    let c: usize = heap.len();
    heap.push(element);
    sort_up(heap, heap_type, c)
}

/// Removes the smallest item from the top of the heap. Returns *None* if the heap is empty.
///
/// ## Example:
///
/// ```
/// use rheap::{HeapType, extract};
///
/// let mut heap: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
/// if let Some(smallest) = extract(&mut heap, HeapType::MinHeap) {
///     assert!(smallest == 0);
///     assert!(heap[0] == 2);
/// } else {
///     panic!();
/// }
/// ```
pub fn extract<T>(heap: &mut Vec<T>, heap_type: HeapType) -> Option<T>
where
    T: Ord,
{
    remove(heap, heap_type, 0).ok()
}

/// Performs a linear search to find the index of an element on the heap.
/// Returns *None* if the element was not found.
///
/// ## Example:
///
/// ```
/// use rheap::{HeapType, find};
///
/// let mut heap: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
/// if let Some(index) = find(&mut heap, &6) {
///     assert!(index == 3);
///     assert!(heap[0] == 0);
/// } else {
///     panic!();
/// }
/// ```
pub fn find<T>(heap: &[T], element: &T) -> Option<usize>
where
    T: Ord + Eq,
{
    (0..heap.len()).find(|&i| heap[i] == *element)
}

/// Updates the value of the element at index
/// Returns and error if the element is not found in the heap or the index is out of bounds.
///
/// ## Example:
///
/// ```
/// use rheap::{HeapType, update};
///
/// let mut heap: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
/// if let Ok(old_element) = update(&mut heap, HeapType::MinHeap, 3, 11) {
///     assert!(old_element == 6);
///     assert!(heap[0] == 0);
/// } else {
///     panic!();
/// }
/// ```
pub fn update<T>(heap: &mut [T], heap_type: HeapType, index: usize, new_element: T) -> Result<T>
where
    T: Ord + Copy,
{
    if heap.is_empty() {
        Err(Error::new(
            ErrorKind::EmptyHeap,
            "Can not remove elements from an empty heap.",
        ))
    } else if index >= heap.len() {
        Err(Error::new(
            ErrorKind::InvalidIndex,
            "Index is beyond the end of the heap.",
        ))
    } else {
        let old_element: T = heap[index];
        heap[index] = new_element;
        if new_element.cmp(&old_element) as isize == heap_type as isize {
            sort_up(heap, heap_type, index);
        } else {
            sort_down(heap, heap_type, index);
        }
        Ok(old_element)
    }
}

/// Removes and returns the element at index.
/// Returns an error if the heap is empty or if the index is out of bounds.
///
/// ## Example:
///
/// ```
/// use rheap::{HeapType, remove};
///
/// let mut heap: Vec<usize> = vec![0, 2, 4, 6, 8, 10];
/// if let Ok(old_element) = remove(&mut heap, HeapType::MinHeap, 3) {
///     assert!(old_element == 6);
///     assert!(heap[0] == 0);
/// } else {
///     panic!();
/// }
/// ```
pub fn remove<T>(heap: &mut Vec<T>, heap_type: HeapType, index: usize) -> Result<T>
where
    T: Ord,
{
    if heap.is_empty() {
        Err(Error::new(
            ErrorKind::EmptyHeap,
            "Can not remove elements from an empty heap.",
        ))
    } else if index >= heap.len() {
        Err(Error::new(
            ErrorKind::InvalidIndex,
            "Index is beyond the end of the heap.",
        ))
    } else {
        let removed: T = heap.swap_remove(index);
        if index < heap.len() {
            if heap[index].cmp(&removed) as isize == heap_type as isize {
                sort_up(heap, heap_type, index);
            } else {
                sort_down(heap, heap_type, index);
            }
        }
        Ok(removed)
    }
}

/// Performs an in-place heap sort.
///
/// ## Example:
///
/// ```
/// use rheap::{HeapType, heap_sort};
///
/// let mut heap: Vec<usize> = vec![8, 66, 9, 55, 7, 0, 14, 6, 37, 2];
/// heap_sort(&mut heap, HeapType::MinHeap);
/// assert!(heap[0] == 0);
/// ```
pub fn heap_sort<T>(heap: &mut [T], heap_type: HeapType)
where
    T: Ord,
{
    let len: usize = heap.len();
    if len > 1 {
        let parent: usize = (len - 2) >> 1;
        for i in (0..=parent).rev() {
            sort_down(heap, heap_type, i);
        }
    }
}

/// Trait that describes a minimum or maximum heap.
pub trait Heap<T>
where
    T: Ord + Eq + Copy,
{
    /// Inserts an element into the heap.
    fn insert(&mut self, element: T);

    /// Removes the smallest item from the top of the heap. Returns ```None``` if the heap is empty.
    fn extract(&mut self) -> Option<T>;

    /// Performs a linear search to find the index of ```element``` on the heap.
    /// If ```element``` is found, then it will return its index on the heap.
    /// Otherwise, it will return ```None```.
    fn find(&self, element: &T) -> Option<usize>;

    /// Updates the value of the ```element``` at ```index```.
    /// Returns and error if the element is not found in the heap or the index is out of bounds.
    fn update(&mut self, index: usize, new_element: T) -> Result<T>;

    /// Removes and returns the element at ```index```.
    /// Returns an error if the heap is empty or if the index is out of bounds.
    fn remove(&mut self, index: usize) -> Result<T>;

    /// Returns the number of elements in the heap, also referred to as its 'length'.
    fn len(&self) -> usize;

    /// Clears the heap, removing all elements.
    /// Note that this method has no effect on the allocated capacity of the heap.
    fn clear(&mut self);

    /// Returns true if the heap contains no elements.
    fn is_empty(&self) -> bool;

    /// Returns the underlying array as a slice.
    fn as_slice(&self) -> &[T];

    /// Returns the underlying array as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [T];
}

/// A complete binary tree in which the value of each node in the tree is
/// less than the value of each of its children. As a consequence, the smallest
/// value in the tree is always located at the root of the tree.
#[derive(Debug, Clone)]
pub struct MinHeap<T>
where
    T: Ord + Eq + Copy,
{
    heap: Vec<T>,
}

impl<T> Default for MinHeap<T>
where
    T: Ord + Eq + Copy,
{
    fn default() -> Self {
        Self { heap: Vec::new() }
    }
}

impl<T> MinHeap<T>
where
    T: Ord + Eq + Copy,
{
    /// Creates and returns a new empty minimum heap.
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
}

impl<T> Heap<T> for MinHeap<T>
where
    T: Ord + Eq + Copy,
{
    /// Returns the underlying array as a slice.
    fn as_slice(&self) -> &[T] {
        self.heap.as_slice()
    }

    /// Returns the underlying array as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.heap.as_mut_slice()
    }

    /// Clears the heap, removing all elements.
    /// Note that this method has no effect on the allocated capacity of the heap.
    #[inline]
    fn clear(&mut self) {
        self.heap.clear()
    }

    /// Returns the number of elements in the heap, also referred to as its 'length'.
    #[inline]
    fn len(&self) -> usize {
        self.heap.len()
    }

    /// Removes the smallest item from the top of the heap. Returns ```None``` if the heap is empty.
    #[inline]
    fn extract(&mut self) -> Option<T> {
        extract(&mut self.heap, HeapType::MinHeap)
    }

    /// Performs a linear search to find the index of ```element``` on the heap.
    /// If the element is found, then it will return its index on the heap.
    /// Otherwise, it will return ```None```.
    #[inline]
    fn find(&self, element: &T) -> Option<usize> {
        find(&self.heap, element)
    }

    /// Inserts ```element``` into the heap.
    #[inline]
    fn insert(&mut self, element: T) {
        insert(&mut self.heap, HeapType::MinHeap, element)
    }

    /// Returns true if the heap contains no elements.
    #[inline]
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Removes and returns the element at ```index```.
    /// Returns an error if the heap is empty or if the index is out of bounds.
    #[inline]
    fn remove(&mut self, index: usize) -> Result<T> {
        remove(&mut self.heap, HeapType::MinHeap, index)
    }

    /// Updates the value of the ```element``` at ```index```.
    /// Returns and error if the element is not found in the heap or the index is out of bounds.
    #[inline]
    fn update(&mut self, index: usize, new_element: T) -> Result<T> {
        update(&mut self.heap, HeapType::MinHeap, index, new_element)
    }
}

/// A complete binary tree in which the value of each node in the tree
/// is greater than the value of each of its children. As a consequence,
/// the largest value in the tree is always located at the root of the tree.
#[derive(Debug, Clone)]
pub struct MaxHeap<T>
where
    T: Ord + Eq + Copy,
{
    heap: Vec<T>,
}

impl<T> Default for MaxHeap<T>
where
    T: Ord + Eq + Copy,
{
    fn default() -> Self {
        Self { heap: Vec::new() }
    }
}

impl<T> MaxHeap<T>
where
    T: Ord + Eq + Copy,
{
    /// Creates and returns a new empty maximum heap.
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
}

impl<T> Heap<T> for MaxHeap<T>
where
    T: Ord + Eq + Copy,
{
    /// Returns the underlying array as a slice.
    fn as_slice(&self) -> &[T] {
        self.heap.as_slice()
    }

    /// Returns the underlying array as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.heap.as_mut_slice()
    }

    /// Clears the heap, removing all elements.
    /// Note that this method has no effect on the allocated capacity of the heap.
    #[inline]
    fn clear(&mut self) {
        self.heap.clear()
    }

    /// Returns the number of elements in the heap, also referred to as its 'length'.
    #[inline]
    fn len(&self) -> usize {
        self.heap.len()
    }

    /// Removes the smallest item from the top of the heap. Returns ```None``` if the heap is empty.
    #[inline]
    fn extract(&mut self) -> Option<T> {
        extract(&mut self.heap, HeapType::MaxHeap)
    }

    /// Performs a linear search to find the index of ```element``` on the heap.
    /// If the element is found, then it will return its index on the heap.
    /// Otherwise, it will return ```None```.
    #[inline]
    fn find(&self, element: &T) -> Option<usize> {
        find(&self.heap, element)
    }

    /// Inserts ```element``` into the heap.
    #[inline]
    fn insert(&mut self, element: T) {
        insert(&mut self.heap, HeapType::MaxHeap, element)
    }

    /// Returns true if the heap contains no elements.
    #[inline]
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Removes and returns the element at ```index```.
    /// Returns an error if the heap is empty or if the index is out of bounds.
    #[inline]
    fn remove(&mut self, index: usize) -> Result<T> {
        remove(&mut self.heap, HeapType::MaxHeap, index)
    }

    /// Updates the value of the ```element``` at ```index```.
    /// Returns and error if the element is not found in the heap or the index is out of bounds.
    #[inline]
    fn update(&mut self, index: usize, new_element: T) -> Result<T> {
        update(&mut self.heap, HeapType::MaxHeap, index, new_element)
    }
}
