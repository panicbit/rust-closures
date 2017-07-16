#![feature(unboxed_closures,fn_traits)]
#![no_std]

//! ```rust
//! extern crate closures;
//!
//! use std::thread;
//! use std::sync::mpsc;
//! use closures::Closure;
//!
//! struct State {
//!     id: i32,
//!     messages: Vec<&'static str>,
//!     tx: mpsc::Sender<(i32, &'static str)>,
//! }
//!
//! fn main() {
//!     let (tx, rx) = mpsc::channel();
//!     
//!     let state = State {
//!         id: 0,
//!         messages: vec!["hello", "rusty", "world"],
//!         tx: tx.clone(),
//!     };
//!     thread::spawn(Closure::new(state, thread));
//!     
//!     let state = State {
//!         id: 1,
//!         messages: vec!["veni", "vidi", "vici"],
//!         tx: tx,
//!     };
//!     thread::spawn(Closure::new(state, thread));
//!     
//!     for (id, msg) in rx {
//!         println!("Thread {} sent: {}", id, msg);
//!     }
//! }
//!
//! fn thread(this: &State) {
//!     for msg in &this.messages {
//!         this.tx.send((this.id, msg)).unwrap();
//!     }
//! }
//! ```

#[macro_use]
mod macros;

impl_closures!(ClosureOnce, ClosureMut, Closure, RecClosureOnce, RecClosureMut, RecClosure,);
impl_closures!(ClosureOnce1, ClosureMut1, Closure1, RecClosureOnce1, RecClosureMut1, RecClosure1, T1,);
impl_closures!(ClosureOnce2, ClosureMut2, Closure2, RecClosureOnce2, RecClosureMut2, RecClosure2, T1, T2);
impl_closures!(ClosureOnce3, ClosureMut3, Closure3, RecClosureOnce3, RecClosureMut3, RecClosure3, T1, T2, T3);
impl_closures!(ClosureOnce4, ClosureMut4, Closure4, RecClosureOnce4, RecClosureMut4, RecClosure4, T1, T2, T3, T4);
impl_closures!(ClosureOnce5, ClosureMut5, Closure5, RecClosureOnce5, RecClosureMut5, RecClosure5, T1, T2, T3, T4, T5);
impl_closures!(ClosureOnce6, ClosureMut6, Closure6, RecClosureOnce6, RecClosureMut6, RecClosure6, T1, T2, T3, T4, T5, T6);
impl_closures!(ClosureOnce7, ClosureMut7, Closure7, RecClosureOnce7, RecClosureMut7, RecClosure7, T1, T2, T3, T4, T5, T6, T7);
impl_closures!(ClosureOnce8, ClosureMut8, Closure8, RecClosureOnce8, RecClosureMut8, RecClosure8, T1, T2, T3, T4, T5, T6, T7, T8);
impl_closures!(ClosureOnce9, ClosureMut9, Closure9, RecClosureOnce9, RecClosureMut9, RecClosure9, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_closures!(ClosureOnce10, ClosureMut10, Closure10, RecClosureOnce10, RecClosureMut10, RecClosure10, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
