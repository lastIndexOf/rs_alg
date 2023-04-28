pub struct Heap<T> {
    length: usize,
    values: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            length: 0,
            values: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn parent(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    pub fn child_left(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    pub fn child_right(&self, idx: usize) -> usize {
        2 * (idx + 1)
    }

    pub fn has_child(&self, idx: usize) -> bool {
        self.child_left(idx) < self.length
    }

    pub fn suitable_child(&self, idx: usize) -> usize {
        if !self.has_child(idx) {
            eprintln!("{idx} do not has child");
            std::process::exit(1);
        }

        if self.child_right(idx) >= self.length {
            return self.child_left(idx);
        }

        let left = self.child_left(idx);
        let right = self.child_right(idx);

        if (self.comparator)(&self.values[left], &self.values[right]) {
            return left;
        }

        right
    }

    pub fn add(&mut self, value: T) {
        self.length += 1;
        self.values.push(value);

        let mut child = self.length - 1;

        if child == 0 {
            return;
        }

        let mut parent = self.parent(child);

        while parent > 0 || parent == 0 {
            if (self.comparator)(&self.values[child], &self.values[parent]) {
                self.values.swap(parent, child);
                child = parent;

                if parent == 0 {
                    break;
                }

                parent = self.parent(child);
            } else {
                break;
            }
        }
    }
}

impl<T: Ord> Heap<T> {
    pub fn new_max() -> Self {
        Self {
            length: 0,
            values: vec![],
            comparator: |a, b| a > b,
        }
    }

    pub fn new_min() -> Self {
        Self {
            length: 0,
            values: vec![],
            comparator: |a, b| a < b,
        }
    }
}

impl<T> Iterator for Heap<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length <= 0 {
            return None;
        }

        let target = self.values.swap_remove(0);
        self.length -= 1;
        let mut idx = 0;

        while self.has_child(idx) {
            let cdx = self.suitable_child(idx);
            if (self.comparator)(&self.values[cdx], &&self.values[idx]) {
                self.values.swap(idx, cdx);
                idx = cdx;
            } else {
                break;
            }
        }

        Some(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap: Heap<i32> = Heap::new_max();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        println!("heap.values = {:?}", heap.values.clone());
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        println!("heap.values = {:?}", heap.values.clone());
        assert_eq!(heap.next(), Some(11));
        println!("heap.values = {:?}", heap.values.clone());
        assert_eq!(heap.next(), Some(9));
        println!("heap.values = {:?}", heap.values.clone());
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }

    #[derive(Default)]
    struct Point(/* x */ i32, /* y */ i32);

    #[test]
    fn test_key_heap() {
        let mut heap: Heap<Point> = Heap::new(|a, b| a.0 < b.0);
        heap.add(Point(1, 5));
        heap.add(Point(3, 10));
        heap.add(Point(-2, 4));
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.next().unwrap().0, -2);
        assert_eq!(heap.next().unwrap().0, 1);
        heap.add(Point(50, 34));
        assert_eq!(heap.next().unwrap().0, 3);
    }
}
