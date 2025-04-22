#![forbid(unsafe_code)]

use std::fmt::Debug;

use extern_itertools::ChunkBy;
use extern_itertools::Group;
use extern_itertools::Itertools;

pub struct LazyCycle<I>
where
    I: Iterator + Clone,
{
    start_iter: I,
    iter: I,
}

impl<I> LazyCycle<I> 
where 
    I: Iterator + Clone,
{
    fn new(iter: I) -> Self {
        Self {
            start_iter: iter.clone(),
            iter: iter,
        } 
    }
}

impl<I> Iterator for LazyCycle<I>
where 
    I: Iterator + Clone,
    I::Item: Clone,
 {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_iter) = self.iter.next() {
            Some(next_iter.clone())
        }
        else {
            self.iter = self.start_iter.clone();
            self.iter.next()
        }
    }

}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    into_iter: std::vec::IntoIter<<I as Iterator>::Item>
}

impl<I> Extract<I> 
where I: Iterator
{
    pub fn new(vec: Vec<I::Item>) -> Self {
        Self {
            into_iter: vec.into_iter()
        }
    }
}

impl<I> Iterator for Extract<I>  
where I: Iterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.into_iter.next()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
}

impl<I> Iterator for Tee<I>
where I: Iterator,
    I::Item: Clone
{
    type Item = I::Item;

    fn next (&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    chunk_by: ChunkBy<V, I, F>,
}

impl<I, F, V> GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    pub fn new(iter: I, func: F) -> Self {
        GroupBy { chunk_by: iter.into_iter().chunk_by(func) }
    }
}

impl<I, F, V> Iterator for GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    type Item = (V, Vec<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let (iter, vec) = self.chunk_by.into_iter().next()?;
        Some((iter, vec.into_iter().collect()))
    }

}
    

////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized + Clone,
        Self::Item: Clone + Debug,
    {
        LazyCycle::new(self)
    }

    fn extract(mut self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
        Self::Item: Debug,
    {
        let mut extract = vec![];
        let mut item = None;
        let mut i= 0;
        loop {
            dbg!(&extract);
            let yielded = self.next();
            if index == i {
                item = yielded;
            }
            else {
                match yielded {
                    Some(item) => extract.push(item),
                    None => break,
                }
            }
            i += 1;
        }
        (item, Extract::new(extract))
    }

    fn tee(self) -> (Tee<Self>, Tee<Self>)
    where
        Self: Sized + Clone,
        Self::Item: Clone,
    {
        let tee = Tee {iter: self};
        (tee.clone(), tee)
    }

    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq,
    {
        GroupBy::new(self, func)
    }
}

impl<T> ExtendedIterator for T
where T: Iterator {}
// TODO: your code goes here.
