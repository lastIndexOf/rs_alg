use std::ops::{Add, AddAssign};

pub struct FenwickTree<T: Add + AddAssign + Default + Clone + Copy> {
    raw: Vec<T>,
    data: Vec<T>,
}

impl<T: Add + AddAssign + Default + Clone + Copy> FenwickTree<T> {
    pub fn with_len(size: usize) -> Self {
        Self {
            raw: vec![Default::default(); size + 1],
            data: vec![Default::default(); size + 1],
        }
    }

    pub fn add(&mut self, index: usize, value: T) {
        // self.raw.push(value);
        if self.raw.len() < index {
            eprintln!("overflow");
            std::process::exit(1);
        }

        let mut idx = index + 1;
        while idx <= self.raw.len() {
            self.data[idx] += value;
            idx += lowbit(idx);
        }

        self.raw[index + 1] = value;
    }

    pub fn prefix_sum(&self, size: usize) -> T {
        if size > self.raw.len() {
            eprintln!("overflow");
            std::process::exit(1);
        }

        let mut result = Default::default();
        let mut idx = size + 1;

        while idx > 0 {
            result += self.data[idx];
            idx -= lowbit(idx);
        }

        result
    }
}

const fn lowbit(m: usize) -> usize {
    m & -(m as isize) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut ft = FenwickTree::with_len(10);

        ft.add(0, 1);
        ft.add(1, 2);
        ft.add(2, 3);
        ft.add(3, 4);
        ft.add(4, 5);
        ft.add(5, 6);
        ft.add(6, 7);
        ft.add(7, 8);
        ft.add(8, 9);
        ft.add(9, 10);

        assert_eq!(ft.prefix_sum(0), 1);
        assert_eq!(ft.prefix_sum(1), 3);
        assert_eq!(ft.prefix_sum(2), 6);
        assert_eq!(ft.prefix_sum(3), 10);
        assert_eq!(ft.prefix_sum(4), 15);
        assert_eq!(ft.prefix_sum(5), 21);
        assert_eq!(ft.prefix_sum(6), 28);
        assert_eq!(ft.prefix_sum(7), 36);
        assert_eq!(ft.prefix_sum(8), 45);
        assert_eq!(ft.prefix_sum(9), 55);
    }
}
