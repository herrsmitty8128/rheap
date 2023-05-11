#[cfg(test)]
pub mod test {

    use rand::prelude::*;
    use rheap::{extract, find, heap_sort, insert, remove, update, HeapType};

    pub fn smallest_is_on_top(heap: &Vec<usize>) -> bool {
        for i in 1..heap.len() {
            if heap[0] >= heap[i] {
                return false;
            }
        }
        return true;
    }

    const COUNT: usize = 10000;

    #[test]
    pub fn test_heap() {
        let mut heap: Vec<usize> = vec![0; COUNT];

        rand::thread_rng().fill(&mut heap[..]);

        heap_sort(&mut heap, HeapType::MinHeap);

        assert!(smallest_is_on_top(&heap));

        while !heap.is_empty() {
            extract(&mut heap, HeapType::MinHeap);
            assert!(smallest_is_on_top(&heap));
        }

        for _ in 0..COUNT {
            insert(&mut heap, HeapType::MinHeap, rand::random::<usize>());
            assert!(smallest_is_on_top(&heap));
        }

        while !heap.is_empty() {
            let len: usize = heap.len();
            if remove(
                &mut heap,
                HeapType::MinHeap,
                rand::thread_rng().gen_range(0..len),
            )
            .is_err()
            {
                panic!();
            }
            assert!(smallest_is_on_top(&heap));
        }

        heap.resize_with(COUNT, || rand::random::<usize>());
        heap_sort(&mut heap, HeapType::MinHeap);
        assert!(smallest_is_on_top(&heap));

        for _ in 0..COUNT {
            let len: usize = heap.len();
            if update(
                &mut heap,
                HeapType::MinHeap,
                rand::thread_rng().gen_range(0..len),
                rand::random::<usize>(),
            )
            .is_err()
            {
                panic!();
            }
            assert!(smallest_is_on_top(&heap));
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
                    insert(&mut heap, HeapType::MinHeap, n);
                }
                1 => {
                    // extract
                    extract(&mut heap, HeapType::MinHeap);
                }
                2 => {
                    // remove
                    if !heap.is_empty() {
                        let len: usize = heap.len();
                        if remove(
                            &mut heap,
                            HeapType::MinHeap,
                            rand::thread_rng().gen_range(0..len),
                        )
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
                        HeapType::MinHeap,
                        rand::thread_rng().gen_range(0..len),
                        rand::random::<usize>(),
                    )
                    .is_err()
                    {
                        panic!();
                    }
                }
            }

            assert!(
                smallest_is_on_top(&heap),
                "### Your choice of {} was a bad one. prev_choice = {} ###",
                choice,
                prev_choice
            );

            prev_choice = choice;
        }
    }
}
