# rheap

`rheap` is a Rust library containing an implementation of a minimum, maximum, d-way heap.

It supports:
 
- Maximum heaps
- Minimum heaps, without relying on [`core::cmp::Reverse`] or a custom [`std::cmp::Ord`] implementation
- Binary and d-way heaps. Any number of branches up to (usize::MAX - 1) / d are allowed, so use good judgement!
  
Use the [`Heap::update`] method to modify the value of an element on the heap in such
a way that the element's ordering relative to other elements is changed. Modifying 
an element's value through other means may result in a inconsistencies, logic errors,
panics, or other unintended consequences.

## License

*rheap is licensed under the MIT License. Please see the included LICENSE.txt file.

