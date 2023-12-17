use std::{collections::HashSet, hash::Hash};

pub struct MinHeap<K, V> {
    keys: HashSet<K>,
    slots: Vec<Option<(K, V)>>,
}

impl<K, V> MinHeap<K, V>
where
    K: Clone + Eq + Hash,
    V: Ord,
{
    pub fn new() -> Self {
        MinHeap {
            keys: HashSet::new(),
            slots: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.keys.insert(key.clone());
        self.slots.push(Some((key, value)));
        self.bubble_up(self.slots.len() - 1);
    }

    pub fn contains_key(&self, key: &K) -> bool
    where
        K: PartialEq,
    {
        self.keys.contains(key)
    }

    pub fn remove_min(&mut self) -> Option<(K, V)> {
        if self.slots.is_empty() {
            return None;
        }

        let (key, value) = self.slots.swap_remove(0).unwrap();

        if !self.slots.is_empty() {
            self.bubble_down(0);
        }

        self.keys.remove(&key);

        Some((key, value))
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.slots[parent].as_ref().unwrap().1 <= self.slots[index].as_ref().unwrap().1 {
                break;
            }
            self.slots.swap(parent, index);
            index = parent;
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        loop {
            let left = index * 2 + 1;
            let right = index * 2 + 2;
            let mut smallest = index;
            if left < self.slots.len()
                && self.slots[left].as_ref().unwrap().1 < self.slots[smallest].as_ref().unwrap().1
            {
                smallest = left;
            }
            if right < self.slots.len()
                && self.slots[right].as_ref().unwrap().1 < self.slots[smallest].as_ref().unwrap().1
            {
                smallest = right;
            }
            if smallest == index {
                break;
            }
            self.slots.swap(index, smallest);
            index = smallest;
        }
    }
}
