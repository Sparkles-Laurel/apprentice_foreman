use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

pub type Polyamorous<T: Send + Sync> = Arc<Mutex<T>>;

pub struct Job<T: Send + Sync> {
    pub id: usize,
    pub work: fn() -> T
}

pub enum Message<T: Send + Sync> {
    NewJob(Job<T>),
    Claim(usize),
    Done(usize),
    Terminate,
}

/// Represents a foreman that manages a pool of apprentices.
pub struct Foreman<T: Send + Sync> {
    /// Sender for sending messages from the foreman.
    pub sender: Sender<Message<T>>,
    /// Thread for the foreman.
    pub thread: JoinHandle<T>,
    /// Message queue for the foreman.
    _msg_queue: VecDeque<Message<T>>,
    /// List of apprentices under the foreman.
    pub apprentices: Vec<Apprentice<T>>,
}

/// Represents an apprentice that works under a foreman.
pub struct Apprentice<T: Send + Sync> {
    /// ID of the apprentice.
    pub id: usize,
    /// Sender for sending messages to the apprentice.
    pub sender: Sender<Message<T>>,
    /// Thread for the apprentice.
    pub thread: JoinHandle<T>,
    /// Message queue for the apprentice.
    _msg_queue: VecDeque<Message<T>>,
}