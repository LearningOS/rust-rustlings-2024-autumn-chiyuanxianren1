/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;
use std::mem::replace ;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.percolate_up(self.count);
        self.count += 1;
    }

    fn percolate_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent_idx = self.parent_idx(idx);
            // 使用提供的比较函数来决定是否交换元素
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(parent_idx, idx);
                idx = parent_idx; // 继续在父节点上进行比较
            } else {
                break; // 不需要继续上浮
            }
        }
    }
    fn parent_idx(&self, idx: usize) -> usize {
        (idx-1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2+1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.right_child_idx(idx) >= self.count {
            self.left_child_idx(idx)
        } else {
            if (self.comparator)(&self.items[self.left_child_idx(idx)], &self.items[self.right_child_idx(idx)]) {
                self.left_child_idx(idx)
            } else {
                self.right_child_idx(idx)
            }
        }
    }
    
    fn percolate_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let smallest_child = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[smallest_child], &self.items[idx]) {
                self.items.swap(smallest_child, idx);
                idx = smallest_child;
            }else{
                break;
            }
            
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            let top = replace(&mut self.items[0], T::default());
            self.items[0] = replace(&mut self.items[self.count], T::default());
            self.items.remove(self.count);
            self.percolate_down(0);
            Some(top)
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    // #[test]
    // fn test_min_heap() {
    //     let mut heap = MinHeap::new();
    //     heap.add(4);
    //     heap.add(2);
    //     heap.add(9);
    //     heap.add(11);
    //     assert_eq!(heap.len(), 4);
    //     assert_eq!(heap.next(), Some(2));
    //     assert_eq!(heap.next(), Some(4));
    //     assert_eq!(heap.next(), Some(9));
    //     heap.add(1);
    //     assert_eq!(heap.next(), Some(1));
    // }

    // #[test]
    // fn test_max_heap() {
    //     let mut heap = MaxHeap::new();
    //     heap.add(4);
    //     heap.add(2);
    //     heap.add(9);
    //     heap.add(11);
    //     assert_eq!(heap.len(), 4);
    //     assert_eq!(heap.next(), Some(11));
    //     assert_eq!(heap.next(), Some(9));
    //     assert_eq!(heap.next(), Some(4));
    //     heap.add(1);
    //     assert_eq!(heap.next(), Some(2));
    // }
}