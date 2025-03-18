#![forbid(unsafe_code)]

use std::{cmp, collections::VecDeque};

#[derive(Default)]
pub struct MinQueue<T> {
    stack1: VecDeque<(T, T)>,
    stack2: VecDeque<(T, T)>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            stack1: VecDeque::<(T, T)>::new(),
            stack2: VecDeque::<(T, T)>::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        let min = if self.stack1.is_empty() {
            &val
        } else {
            cmp::min(&val, &self.stack1.front().unwrap().1)
        }
        .clone();

        self.stack1.push_front((val, min));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.stack2.is_empty() {
            while !self.stack1.is_empty() {
                let elem = self.stack1.pop_front().unwrap().0;
                let min = if self.stack2.is_empty() {
                    &elem
                } else {
                    cmp::min(&elem, &self.stack2.front().unwrap().1)
                }
                .clone();
                self.stack2.push_front((elem, min));
            }
        }
        self.stack2.pop_front().map(|x| x.0)
    }

    pub fn front(&self) -> Option<&T> {
        if !self.stack2.is_empty() {
            self.stack2.front().map(|x| &x.0)
        } else {
            self.stack1.back().map(|x| &x.0)
        }
    }

    pub fn min(&self) -> Option<&T> {
        if self.stack1.is_empty() || self.stack2.is_empty() {
            if self.stack1.is_empty() {
                self.stack2.front().map(|x| &x.1)
            } else {
                self.stack1.front().map(|x| &x.1)
            }
        } else {
            cmp::min(
                self.stack1.front().map(|x| &x.1),
                self.stack2.front().map(|x| &x.1),
            )
        }
    }

    pub fn len(&self) -> usize {
        self.stack1.len() + self.stack2.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}
