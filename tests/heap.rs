#[cfg(test)]
pub mod test {

    use rand::prelude::*;
    use rheap::Heap;

    const COUNT: usize = 5000;

    #[test]
    pub fn test_min_heap() {
        test_heap::<2, false>();
    }

    #[test]
    pub fn test_max_heap() {
        test_heap::<2, true>();
    }

    pub fn test_heap<const D: usize, const H: bool>() {
        let mut v: Vec<usize> = vec![0; COUNT];

        rand::thread_rng().fill(&mut v[..]);

        let mut heap: Heap<usize, H, D> = Heap::from(&v[..]);

        assert!(heap.is_valid());

        while !heap.is_empty() {
            heap.top();
            assert!(heap.is_valid(), "heap.top() failed");
        }

        for _ in 0..COUNT {
            heap.insert(rand::random::<usize>());
            assert!(heap.is_valid(), "heap.insert() failed");
        }

        while !heap.is_empty() {
            let len: usize = heap.len();
            if heap.remove(rand::thread_rng().gen_range(0..len)).is_err() {
                panic!();
            }
            assert!(heap.is_valid(), "heap.remove() failed");
        }

        for _ in 0..COUNT {
            heap.insert(rand::random::<usize>());
            assert!(heap.is_valid(), "heap.insert() failed");
        }

        for _ in 0..COUNT {
            let len: usize = heap.len();
            if heap
                .update(rand::thread_rng().gen_range(0..len), |x| {
                    *x = rand::random::<usize>()
                })
                .is_err()
            {
                panic!();
            }
            assert!(heap.is_valid(), "heap.update() failed");
        }

        let mut prev_choice: usize = usize::MAX;

        for _ in 0..COUNT {
            let choice: usize = rand::thread_rng().gen_range(0..4);

            match choice {
                0 => {
                    // insert
                    let n = rand::random::<usize>();
                    heap.insert(n);
                }
                1 => {
                    // extract
                    heap.top();
                }
                2 => {
                    // remove
                    if !heap.is_empty() {
                        let len: usize = heap.len();
                        if heap.remove(rand::thread_rng().gen_range(0..len)).is_err() {
                            panic!()
                        }
                    }
                }
                _ => {
                    // update
                    let len: usize = heap.len();
                    if !heap.is_empty() {
                        if heap
                            .update(rand::thread_rng().gen_range(0..len), |x| {
                                *x = rand::random::<usize>()
                            })
                            .is_err()
                        {
                            panic!("heap.update() returned an error");
                        }
                    }
                }
            }

            assert!(
                heap.is_valid(),
                "### Your choice of {} was a bad one. prev_choice = {} ###",
                choice,
                prev_choice
            );

            prev_choice = choice;
        }
    }
}
