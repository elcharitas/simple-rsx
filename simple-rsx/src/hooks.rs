//! WASM-based hooks implementation for Simple RSX
//!
//! This module provides React-like hooks that only work in WebAssembly context
//! after the component has been hydrated on the client side.

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// Represents the state of a component
#[derive(Clone)]
pub struct State<T: Clone + 'static> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Box<dyn Fn()>>>>,
}

impl<T: Clone + 'static> State<T> {
    /// Creates a new state with the given initial value
    pub fn new(initial: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(initial)),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Gets the current value
    pub fn get(&self) -> T {
        (*self.value.borrow()).clone()
    }

    /// Sets a new value and notifies all subscribers
    pub fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
        for subscriber in self.subscribers.borrow().iter() {
            subscriber();
        }
    }

    /// Subscribes to state changes
    pub fn subscribe(&self, callback: Box<dyn Fn()>) {
        self.subscribers.borrow_mut().push(callback);
    }
}

/// Creates a new state hook that persists across re-renders
#[wasm_bindgen]
pub fn use_state<T: Clone + 'static>(initial: T) -> State<T> {
    State::new(initial)
}

/// Effect hook for handling side effects
#[wasm_bindgen]
pub fn use_effect(effect: Box<dyn Fn()>, deps: Vec<JsValue>) {
    let prev_deps = Rc::new(RefCell::new(None));

    let should_run = match &*prev_deps.borrow() {
        None => true,
        Some(old_deps) => deps != *old_deps,
    };

    if should_run {
        effect();
        *prev_deps.borrow_mut() = Some(deps);
    }
}

/// Memo hook for memoizing expensive computations
#[wasm_bindgen]
pub fn use_memo<T, F>(compute: F, deps: Vec<JsValue>) -> T
where
    F: Fn() -> T,
    T: Clone + 'static,
{
    let prev_deps = Rc::new(RefCell::new(None));
    let memoized = Rc::new(RefCell::new(None));

    let should_recompute = match &*prev_deps.borrow() {
        None => true,
        Some(old_deps) => deps != *old_deps,
    };

    if should_recompute {
        let result = compute();
        *memoized.borrow_mut() = Some(result.clone());
        *prev_deps.borrow_mut() = Some(deps);
        result
    } else {
        memoized.borrow().clone().unwrap()
    }
}
