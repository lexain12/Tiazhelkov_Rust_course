#![forbid(unsafe_code)]

use std::iter::Cloned;

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
 {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_iter) = self.iter.next() {
            Some(next_iter)
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
    iter: I,
    func: F,
    vec: V,
    // TODO: your code goes here.
}

////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized + Clone,
        Self::Item: Clone,
    {
        LazyCycle::new(self)
    }

    fn extract(mut self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
    {
        let mut extract = vec![];
        let mut item = None;
        for i in 0..=index {
            if i == index {
                item = self.next();
            }
            else {
                match (self.next()) {
                    Some(item) => extract.push(item),
                    None => break,
                }
            }
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
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T> ExtendedIterator for T
where T: Iterator {}
// TODO: your code goes here.
