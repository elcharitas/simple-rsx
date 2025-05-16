//! WASM-based hooks implementation for Simple RSX
//!
//! This module provides React-like hooks that only work in WebAssembly context
//! after the component has been hydrated on the client side.
use std::cell::RefCell;
use std::rc::Rc;

/// Represents a signal that can be read and written
#[derive(Clone)]
pub struct Signal<T: 'static> {
    inner: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Box<dyn Fn()>>>>,
}

impl<T: Clone + 'static> Signal<T> {
    /// Creates a new signal with the given initial value
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(value)),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Gets the current value of the signal
    pub fn get(&self) -> &T {
        &self.inner.borrow().clone()
    }

    /// Sets a new value for the signal and notifies subscribers
    pub fn set(&self, value: T) {
        *self.inner.borrow_mut() = value;
        self.notify_subscribers();
    }

    /// Updates the value using a closure
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut *self.inner.borrow_mut());
        self.notify_subscribers();
    }

    /// Adds a subscriber to be notified when the signal changes
    pub fn subscribe(&self, callback: Box<dyn Fn()>) {
        self.subscribers.borrow_mut().push(callback);
    }

    /// Notifies all subscribers about a value change
    fn notify_subscribers(&self) {
        for callback in self.subscribers.borrow().iter() {
            callback();
        }
    }
}

impl<T: Clone + 'static> std::ops::Deref for Signal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // This is not ideal since we need to clone the value
        // but it allows for simpler usage patterns
        // The actual value is stored in the Rc<RefCell<T>>
        &self.get()
    }
}

/// A mutable reference to a signal's value
#[derive(Clone)]
pub struct SignalMut<T: 'static> {
    signal: Signal<T>,
}

impl<T: Clone + 'static> SignalMut<T> {
    /// Gets the current value of the signal
    pub fn get(&self) -> T {
        self.signal.get()
    }

    /// Sets a new value for the signal
    pub fn set(&self, value: T) {
        self.signal.set(value);
    }

    /// Updates the value using a closure
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        self.signal.update(f);
    }
}

impl<T: Clone + 'static> std::ops::Deref for SignalMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.get()
    }
}

// Global state for hooks
thread_local! {
    static CURRENT_COMPONENT: RefCell<Option<usize>> = RefCell::new(None);
    // static COMPONENT_HOOKS: RefCell<Vec<Vec<Box<dyn Any>>>> = RefCell::new(Vec::new());
    static CURRENT_HOOK_INDEX: RefCell<usize> = RefCell::new(0);
}

/// Initialize a new component context for hooks
pub fn init_component(id: usize) {
    CURRENT_COMPONENT.with(|current| {
        *current.borrow_mut() = Some(id);
    });

    COMPONENT_HOOKS.with(|hooks| {
        let mut hooks = hooks.borrow_mut();
        while hooks.len() <= id {
            hooks.push(Vec::new());
        }
    });

    CURRENT_HOOK_INDEX.with(|index| {
        *index.borrow_mut() = 0;
    });
}

/// Get or create a hook state
fn use_hook<T: 'static, F: FnOnce() -> T>(init: F) -> &'static mut T {
    let component_id = CURRENT_COMPONENT
        .with(|current| current.borrow().expect("Component context not initialized"));

    let hook_index = CURRENT_HOOK_INDEX.with(|index| {
        let idx = *index.borrow();
        *index.borrow_mut() += 1;
        idx
    });

    COMPONENT_HOOKS.with(|hooks| {
        let mut hooks = hooks.borrow_mut();
        let component_hooks = &mut hooks[component_id];

        if hook_index >= component_hooks.len() {
            let hook = Box::new(init());
            component_hooks.push(hook);
        }

        let hook = &mut component_hooks[hook_index];
        let hook_ref = hook.downcast_mut::<T>().expect("Hook type mismatch");
        hook_ref
    })
}

/// `use_signal` hook, works like solidjs's `useSignal`
/// In order to work with wasm, it doesn't use type parameters or generic types
/// instead it uses a closure to initialize the signal value
///
/// # Example
///
/// ```rust
/// use simple_rsx::*;
/// use wasm_bindgen::prelude::*;
///
/// #[component]
/// fn Counter() -> Node {
///     let mut count = use_signal(|| 0);
///
///     rsx!(
///         <div>
///             <button onclick={move || count.set(count + 1)}>
///                 Increment
///             </button>
///             <p>{count()}</p>
///         </div>
///     )
/// }
/// ```
pub fn use_signal<T: Clone + 'static, F: FnOnce() -> T>(init: F) -> SignalMut<T> {
    let signal = use_hook(|| Signal::new(init()));
    SignalMut {
        signal: signal.clone(),
    }
}

/// `use_memo` hook for memoized values
/// Recalculates only when dependencies change
pub fn use_memo<T: Clone + 'static, D: PartialEq + Clone + 'static, F: Fn(&D) -> T>(
    deps: D,
    compute: F,
) -> T {
    #[derive(Clone)]
    struct MemoState<T, D> {
        value: T,
        deps: D,
    }

    let memo = use_hook(|| {
        let value = compute(&deps);
        MemoState { value, deps }
    });

    if memo.deps != deps {
        memo.value = compute(&deps);
        memo.deps = deps;
    }

    memo.value.clone()
}

/// `use_effect` hook for side effects
/// Runs the effect when dependencies change
pub fn use_effect<
    D: PartialEq + Clone + 'static,
    F: FnOnce() -> Option<Box<dyn FnOnce()>> + 'static,
>(
    deps: D,
    effect: F,
) {
    struct EffectState<D> {
        deps: Option<D>,
        cleanup: Option<Box<dyn FnOnce()>>,
    }

    let effect_state = use_hook(|| EffectState::<D> {
        deps: None,
        cleanup: None,
    });

    let should_run = match &effect_state.deps {
        None => true,
        Some(prev_deps) => *prev_deps != deps,
    };

    if should_run {
        // Run cleanup if it exists
        if let Some(cleanup) = effect_state.cleanup.take() {
            cleanup();
        }

        // Run the effect and store cleanup if returned
        effect_state.cleanup = effect();
        effect_state.deps = Some(deps);
    }
}

/// `use_state` hook, similar to React's useState
pub fn use_state<T: Clone + 'static, F: FnOnce() -> T>(init: F) -> (T, impl Fn(T)) {
    let signal = use_signal(init);
    let getter = signal.get();
    let setter = move |value| signal.set(value);
    (getter, setter)
}

/// `use_ref` hook for mutable references
pub fn use_ref<T: 'static, F: FnOnce() -> T>(init: F) -> Rc<RefCell<T>> {
    use_hook(|| Rc::new(RefCell::new(init()))).clone()
}

/// `use_callback` hook for memoized callbacks
pub fn use_callback<F: Clone + 'static, D: PartialEq + Clone + 'static>(deps: D, callback: F) -> F {
    use_memo(deps, |_| callback.clone())
}
