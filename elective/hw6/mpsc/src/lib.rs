#![forbid(unsafe_code)]

use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T> {
    pub value: T,
}

pub struct Sender<T> {
    channel: Rc<RefCell<VecDeque<T>>>,
    is_closed: Rc<Cell<bool>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        if self.is_closed() {
            Err(SendError { value })
        } else {
            match self.channel.try_borrow_mut() {
                Ok(mut channel) => {
                    channel.push_back(value);
                    Ok(())
                }
                Err(_) => Err(SendError { value }),
            }
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed.get()
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.channel, &other.channel)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            channel: Rc::clone(&self.channel),
            is_closed: Rc::clone(&self.is_closed),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        if Rc::strong_count(&self.is_closed) == 2 {
            self.is_closed.set(true);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    channel: Rc<RefCell<VecDeque<T>>>,
    is_closed: Rc<Cell<bool>>,
    // To make anti clone because negative trait are still not stable
    _anti_clone: PhantomData<*mut ()>,
}

impl<T> Receiver<T> {
    // SAFETY: we use this only in single threaded version
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        if self.is_closed() && self.channel.borrow().len() == 0 {
            return Err(ReceiveError::Closed);
        }
        self.channel
            .borrow_mut()
            .pop_front()
            .ok_or(ReceiveError::Empty)
    }

    pub fn close(&mut self) {
        self.is_closed.set(true);
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed.get()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.close();
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Rc::new(RefCell::new(VecDeque::new()));
    let is_closed = Rc::new(Cell::new(false));
    (
        Sender {
            channel: Rc::clone(&channel),
            is_closed: Rc::clone(&is_closed),
        },
        Receiver {
            channel: Rc::clone(&channel),
            is_closed: Rc::clone(&is_closed),
            _anti_clone: PhantomData,
        },
    )
}
