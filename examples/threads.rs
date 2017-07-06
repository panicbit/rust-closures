extern crate closures;

use std::thread;
use std::sync::mpsc;
use closures::Closure;

struct State {
    id: i32,
    messages: Vec<&'static str>,
    tx: mpsc::Sender<(i32, &'static str)>,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let state = State {
        id: 0,
        messages: vec!["hello", "rusty", "world"],
        tx: tx.clone(),
    };
    thread::spawn(Closure::new(state, thread));
    
    let state = State {
        id: 1,
        messages: vec!["veni", "vidi", "vici"],
        tx: tx,
    };
    thread::spawn(Closure::new(state, thread));
    
    for (id, msg) in rx {
        println!("Thread {} sent: {}", id, msg);
    }
}

fn thread(this: &State) {
    for msg in &this.messages {
        this.tx.send((this.id, msg)).unwrap();
    }
}
