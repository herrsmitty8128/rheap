#[cfg(test)]
pub mod test {

    use rand::prelude::*;
    use rheap::{extract, find, heap_sort, insert, remove, update, HeapType};

    const COUNT: usize = 10000;

    #[test]
    pub fn test_min_heap() {
        test_heap(HeapType::MinHeap);
    }

    #[test]
    pub fn test_max_heap() {
        test_heap(HeapType::MaxHeap);
    }

    pub fn top_has_correct_value(heap: &Vec<usize>, heap_type: HeapType) -> bool {
        if heap_type == HeapType::MinHeap {
            for i in 1..heap.len() {
                if heap[0] >= heap[i] {
                    return false;
                }
            }
        } else {
            for i in 1..heap.len() {
                if heap[0] <= heap[i] {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn test_heap(heap_type: HeapType) {
        let mut heap: Vec<usize> = vec![0; COUNT];

        rand::thread_rng().fill(&mut heap[..]);

        heap_sort(&mut heap, heap_type);

        assert!(top_has_correct_value(&heap, heap_type));

        while !heap.is_empty() {
            extract(&mut heap, heap_type);
            assert!(top_has_correct_value(&heap, heap_type));
        }

        for _ in 0..COUNT {
            insert(&mut heap, heap_type, rand::random::<usize>());
            assert!(top_has_correct_value(&heap, heap_type));
        }

        while !heap.is_empty() {
            let len: usize = heap.len();
            if remove(&mut heap, heap_type, rand::thread_rng().gen_range(0..len)).is_err() {
                panic!();
            }
            assert!(top_has_correct_value(&heap, heap_type));
        }

        heap.resize_with(COUNT, || rand::random::<usize>());
        heap_sort(&mut heap, heap_type);
        assert!(top_has_correct_value(&heap, heap_type));

        for _ in 0..COUNT {
            let len: usize = heap.len();
            if update(
                &mut heap,
                heap_type,
                rand::thread_rng().gen_range(0..len),
                |x| *x = rand::random::<usize>(),
            )
            .is_err()
            {
                panic!();
            }
            assert!(top_has_correct_value(&heap, heap_type));
        }

        for _ in 0..COUNT {
            let len: usize = heap.len();
            let i: usize = rand::thread_rng().gen_range(0..len);
            let element: usize = heap[i];
            if find(&heap, &element).is_none() {
                panic!()
            }
        }

        let mut prev_choice: usize = usize::MAX;

        for _ in 0..COUNT {
            let choice: usize = rand::thread_rng().gen_range(0..4);

            match choice {
                0 => {
                    // insert
                    let n = rand::random::<usize>();
                    insert(&mut heap, heap_type, n);
                }
                1 => {
                    // extract
                    extract(&mut heap, heap_type);
                }
                2 => {
                    // remove
                    if !heap.is_empty() {
                        let len: usize = heap.len();
                        if remove(&mut heap, heap_type, rand::thread_rng().gen_range(0..len))
                            .is_err()
                        {
                            panic!()
                        }
                    }
                }
                _ => {
                    // update
                    let len: usize = heap.len();
                    if update(
                        &mut heap,
                        heap_type,
                        rand::thread_rng().gen_range(0..len),
                        |x| *x = rand::random::<usize>(),
                    )
                    .is_err()
                    {
                        panic!();
                    }
                }
            }

            assert!(
                top_has_correct_value(&heap, heap_type),
                "### Your choice of {} was a bad one. prev_choice = {} ###",
                choice,
                prev_choice
            );

            prev_choice = choice;
        }
    }
}
