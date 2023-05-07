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

/// The error type used by a min and max heap.
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

/// A specialized result type that is used by both heap implementations.
pub type Result<T> = std::result::Result<T, Error>;

/// An enum used to indicate whether a heap is a minimum or maximum heap.
/// Minimum heaps are sorted in (mostly) ascending order.
/// Maximum heaps are sorted in (mostly) descending order.
/// It is your responsibility use the same HeapType when calling different functions for the same heap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapType {
    MinHeap,
    MaxHeap,
}

/// Sorts the heap by iterating down the tree starting from index ```p```.
/// Panics if p is out of bounds.
fn sort_down<T>(heap: &mut [T], heap_type: HeapType, mut p: usize)
where
    T: Ord,
{
    let order: Ordering = if heap_type == HeapType::MaxHeap {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let length: usize = heap.len();
    loop {
        let left: usize = (p * 2) + 1;
        let right: usize = left + 1;
        let mut x: usize = if left < length && heap[left].cmp(&heap[p]) == order {
            left
        } else {
            p
        };
        if right < length && heap[right].cmp(&heap[x]) == order {
            x = right;
        }
        if x == p {
            break;
        }
        heap.swap(p, x);
        p = x;
    }
}

/// Sorts the heap by iterating up the tree starting from index ```c```.
/// Panics if p is out of bounds.
fn sort_up<T>(heap: &mut [T], heap_type: HeapType, mut c: usize)
where
    T: Ord,
{
    let order: Ordering = if heap_type == HeapType::MaxHeap {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    while c > 0 {
        let p: usize = (c - 1) >> 1; // calculate the index of the parent node
        if heap[c].cmp(&heap[p]) == order {
            heap.swap(c, p); // if the child is smaller than the parent, then swap them
        } else {
            break;
        }
        c = p;
    }
}

/// Inserts ```element``` into the heap.
pub fn insert<T>(heap: &mut Vec<T>, heap_type: HeapType, element: T)
where
    T: Ord,
{
    let c: usize = heap.len();
    heap.push(element);
    sort_up(heap, heap_type, c)
}

/// Removes the *smallest* item from the top of the heap. Returns ```None``` if the heap is empty.
pub fn extract<T>(heap: &mut Vec<T>, heap_type: HeapType) -> Option<T>
where
    T: Ord,
{
    remove(heap, heap_type, 0).ok()
}

/// Performs a linear search to find the index of ```element``` on the heap.
/// If the element is found, then it will return its index on the heap.
/// Otherwise, it will return ```None```.
pub fn find<T>(heap: &[T], element: &T) -> Option<usize>
where
    T: Ord + Eq,
{
    (0..heap.len()).find(|&i| heap[i] == *element)
}

/// Updates the value of ```element``` and resorts the heap.
/// Returns ```None``` if the element is not found in the heap.
pub fn update<T>(heap: &mut [T], heap_type: HeapType, index: usize, new_element: &T) -> Result<T>
where
    T: Ord + Clone,
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
        let old_element: T = heap[index].clone();
        heap[index] = new_element.clone();
        heap_sort(heap, heap_type);
        Ok(old_element)
    }
}

/// Removes and returns the element at ```index```.
/// Returns an error if the heap is empty or if the index is out of bounds.
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
        sort_down(heap, heap_type, index);
        Ok(removed)
    }
}

/// Performs an in-place heap sort.
pub fn heap_sort<T>(heap: &mut [T], heap_type: HeapType)
where
    T: Ord,
{
    for i in (1..=(heap.len() - 1)).rev() {
        heap.swap(0, i);
        sort_down(heap, heap_type, 0);
    }
}

pub trait Heap<T>
where
    T: Ord + Eq + Clone,
{
    /// Inserts ```element``` into the heap.
    fn insert(&mut self, element: T);

    /// Removes the *smallest* item from the top of the heap. Returns ```None``` if the heap is empty.
    fn extract(&mut self) -> Option<T>;
    
    /// Performs a linear search to find the index of ```element``` on the heap.
    /// If the element is found, then it will return its index on the heap.
    /// Otherwise, it will return ```None```.
    fn find(&self, element: &T) -> Option<usize>;
    
    /// Replaces the element at ```index``` and resorts the heap.
    /// Returns an error if the heap is empty or the index is out of bounds.
    fn update(&mut self, index: usize, replace_with: &T) -> Result<T>;
    
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
}

#[derive(Debug, Clone)]
pub struct MinHeap<T>
where
    T: Ord + Eq + Clone,
{
    heap: Vec<T>,
}

impl<T> Default for MinHeap<T>
where
    T: Ord + Eq + Clone,
{
    fn default() -> Self {
        Self { heap: Vec::new() }
    }
}

impl<T> MinHeap<T>
where
    T: Ord + Eq + Clone,
{
    /// Creates and returns a new empty minimum heap.
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
}

impl<T> Heap<T> for MinHeap<T>
where
    T: Ord + Eq + Clone,
{
    /// Clears the heap, removing all elements.
    /// Note that this method has no effect on the allocated capacity of the heap.
    fn clear(&mut self) {
        self.heap.clear()
    }

    /// Returns the number of elements in the heap, also referred to as its 'length'.
    fn len(&self) -> usize {
        self.heap.len()
    }

    /// Removes the *smallest* item from the top of the heap. Returns ```None``` if the heap is empty.
    fn extract(&mut self) -> Option<T> {
        extract(&mut self.heap, HeapType::MinHeap)
    }

    /// Performs a linear search to find the index of ```element``` on the heap.
    /// If the element is found, then it will return its index on the heap.
    /// Otherwise, it will return ```None```.
    fn find(&self, element: &T) -> Option<usize> {
        find(&self.heap, element)
    }

    /// Inserts ```element``` into the heap.
    fn insert(&mut self, element: T) {
        insert(&mut self.heap, HeapType::MinHeap, element)
    }

    /// Returns true if the heap contains no elements.
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Removes and returns the element at ```index```.
    /// Returns an error if the heap is empty or if the index is out of bounds.
    fn remove(&mut self, index: usize) -> Result<T> {
        remove(&mut self.heap, HeapType::MinHeap, index)
    }

    /// Replaces the element at ```index``` and resorts the heap.
    /// Returns an error if the heap is empty or the index is out of bounds.
    fn update(&mut self, index: usize, replace_with: &T) -> Result<T> {
        update(&mut self.heap, HeapType::MinHeap, index, replace_with)
    }
}

#[derive(Debug, Clone)]
pub struct MaxHeap<T>
where
    T: Ord + Eq + Clone,
{
    heap: Vec<T>,
}

impl<T> Default for MaxHeap<T>
where
    T: Ord + Eq + Clone,
{
    fn default() -> Self {
        Self { heap: Vec::new() }
    }
}

impl<T> MaxHeap<T>
where
    T: Ord + Eq + Clone,
{
    /// Creates and returns a new empty maximum heap.
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
}

impl<T> Heap<T> for MaxHeap<T>
where
    T: Ord + Eq + Clone,
{
    /// Clears the heap, removing all elements.
    /// Note that this method has no effect on the allocated capacity of the heap.
    fn clear(&mut self) {
        self.heap.clear()
    }

    /// Returns the number of elements in the heap, also referred to as its 'length'.
    fn len(&self) -> usize {
        self.heap.len()
    }

    /// Removes the *smallest* item from the top of the heap. Returns ```None``` if the heap is empty.
    fn extract(&mut self) -> Option<T> {
        extract(&mut self.heap, HeapType::MaxHeap)
    }

    /// Performs a linear search to find the index of ```element``` on the heap.
    /// If the element is found, then it will return its index on the heap.
    /// Otherwise, it will return ```None```.
    fn find(&self, element: &T) -> Option<usize> {
        find(&self.heap, element)
    }

    /// Inserts ```element``` into the heap.
    fn insert(&mut self, element: T) {
        insert(&mut self.heap, HeapType::MaxHeap, element)
    }

    /// Returns true if the heap contains no elements.
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Removes and returns the element at ```index```.
    /// Returns an error if the heap is empty or if the index is out of bounds.
    fn remove(&mut self, index: usize) -> Result<T> {
        remove(&mut self.heap, HeapType::MaxHeap, index)
    }

    /// Replaces the element at ```index``` and resorts the heap.
    /// Returns an error if the heap is empty or the index is out of bounds.
    fn update(&mut self, index: usize, replace_with: &T) -> Result<T> {
        update(&mut self.heap, HeapType::MaxHeap, index, replace_with)
    }
}

