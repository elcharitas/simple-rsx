use crate::nodes::Node;
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::{
    any::Any,
    cmp::Ordering,
    marker::PhantomData,
    ops::{AddAssign, DivAssign, MulAssign, Not, SubAssign},
};
use spin::Mutex;

//==============================================================================
// GLOBAL STATE
//==============================================================================

/// Current scope being executed
static CURRENT_SCOPE: Mutex<Option<usize>> = Mutex::new(None);
/// Scope currently being rendered (0 = none)
static RENDERING_SCOPE: Mutex<usize> = Mutex::new(0);
/// Next available scope ID
static NEXT_SCOPE_ID: Mutex<usize> = Mutex::new(1);

/// Signal counter per scope
static SCOPE_SIGNAL_COUNTERS: Mutex<BTreeMap<usize, usize>> = Mutex::new(BTreeMap::new());
/// Effect counter per scope
static SCOPE_EFFECT_COUNTERS: Mutex<BTreeMap<usize, usize>> = Mutex::new(BTreeMap::new());

/// All signal values by (scope_id, signal_id)
static SIGNALS: Mutex<BTreeMap<(usize, usize), StoredValue>> = Mutex::new(BTreeMap::new());
/// Signals that changed during current scope execution
static SCOPE_SIGNAL_CHANGES: Mutex<BTreeSet<(usize, usize)>> = Mutex::new(BTreeSet::new());

/// Functions that can be re-executed per scope
static SCOPE_FUNCTIONS: Mutex<BTreeMap<usize, Box<dyn FnMut() -> Node + Send>>> =
    Mutex::new(BTreeMap::new());
/// Callbacks to run after scope renders
static SCOPE_CALLBACKS: Mutex<BTreeMap<usize, Arc<dyn Fn(&Node) + Send + Sync>>> =
    Mutex::new(BTreeMap::new());
/// Effects by (scope_id, effect_id)
static SCOPE_EFFECTS: Mutex<BTreeMap<(usize, usize), Box<dyn Fn() + Send>>> =
    Mutex::new(BTreeMap::new());

/// Which signals each scope depends on
static SCOPE_DEPENDENCIES: Mutex<BTreeMap<usize, BTreeSet<(usize, usize)>>> =
    Mutex::new(BTreeMap::new());
/// Which scopes depend on each signal
static SIGNAL_DEPENDENCIES: Mutex<BTreeMap<(usize, usize), BTreeSet<usize>>> =
    Mutex::new(BTreeMap::new());
/// Scopes waiting to re-render
static PENDING_SCOPE_RENDERS: Mutex<BTreeSet<usize>> = Mutex::new(BTreeSet::new());

//==============================================================================
// TRAITS
//==============================================================================

/// Values that can be stored in signals
pub trait SignalValue: Send {
    fn as_any(&self) -> Option<&dyn Any>;
}

macro_rules! impl_signal_value {
    ($($t:ty),*) => {
        $(
            impl SignalValue for $t {
                fn as_any(&self) -> Option<&dyn Any> {
                    Some(self)
                }
            }
        )*
    };
}

impl_signal_value!(
    String,
    &'static str,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    bool,
    char,
    ()
);

impl<T: SignalValue + 'static> SignalValue for alloc::vec::Vec<T> {
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}

impl<T: SignalValue + 'static> SignalValue for Option<T> {
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}

//==============================================================================
// SIGNAL TYPE
//==============================================================================

/// Reactive value that triggers re-renders when changed
#[derive(Copy, Debug)]
pub struct Signal<T> {
    id: (usize, usize),
    _marker: PhantomData<T>,
}

impl<T: SignalValue + Not<Output = bool> + Clone + 'static> Not for Signal<T> {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.get()
    }
}

impl<T: SignalValue + Clone + 'static> Signal<T> {
    pub fn then<R, F: FnOnce() -> R>(self, f: F) -> Option<R>
    where
        Signal<T>: Not<Output = bool>,
    {
        if !!self { Some(f()) } else { None }
    }
}

// implement .then for Signal

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal {
            id: self.id,
            _marker: PhantomData,
        }
    }
}

impl<T: SignalValue + PartialEq + Clone + core::ops::Add<Output = T> + 'static> AddAssign<T>
    for Signal<T>
{
    fn add_assign(&mut self, rhs: T) {
        self.set(self.get() + rhs);
    }
}

impl<T: SignalValue + PartialEq + Clone + core::ops::Sub<Output = T> + 'static> SubAssign<T>
    for Signal<T>
{
    fn sub_assign(&mut self, rhs: T) {
        self.set(self.get() - rhs);
    }
}

impl<T: SignalValue + PartialEq + Clone + core::ops::Mul<Output = T> + 'static> MulAssign<T>
    for Signal<T>
{
    fn mul_assign(&mut self, rhs: T) {
        self.set(self.get() * rhs);
    }
}

impl<T: SignalValue + PartialEq + Clone + core::ops::Div<Output = T> + 'static> DivAssign<T>
    for Signal<T>
{
    fn div_assign(&mut self, rhs: T) {
        self.set(self.get() / rhs);
    }
}

impl<T: SignalValue + PartialEq + 'static> PartialEq<T> for Signal<T> {
    fn eq(&self, other: &T) -> bool {
        self.with(|val| val == other).unwrap_or(false)
    }
}

impl<T: SignalValue + PartialOrd + Clone + 'static> PartialOrd<T> for Signal<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.with(|val| val.partial_cmp(other)).unwrap_or(None)
    }
}

impl<T: SignalValue + PartialOrd + Clone + 'static> PartialOrd<Signal<T>> for Signal<T> {
    fn partial_cmp(&self, other: &Signal<T>) -> Option<Ordering> {
        other
            .with(|other_val| {
                self.with(|self_val| self_val.partial_cmp(other_val))
                    .unwrap_or(None)
            })
            .unwrap_or(None)
    }
}

// Signal-to-Signal equality
impl<T: SignalValue + PartialEq + Clone + 'static> PartialEq<Signal<T>> for Signal<T> {
    fn eq(&self, other: &Signal<T>) -> bool {
        other
            .with(|other_val| self.with(|self_val| self_val == other_val).unwrap_or(false))
            .unwrap_or(false)
    }
}

// Boolean convenience methods
impl Signal<bool> {
    /// Toggle the boolean value
    pub fn toggle(&self) {
        self.set(!self.get());
    }

    /// Set to true
    pub fn turn_on(&self) {
        self.set(true);
    }

    /// Set to false
    pub fn turn_off(&self) {
        self.set(false);
    }
}

// Vector convenience methods
impl<T: SignalValue + PartialEq + Clone + 'static> Signal<Vec<T>> {
    /// Push an item to the vector
    pub fn push(&self, item: T) {
        let mut vec = self.get();
        vec.push(item);
        self.set(vec);
    }

    /// Pop an item from the vector
    pub fn pop(&self) -> Option<T> {
        let mut vec = self.get();
        let result = vec.pop();
        self.set(vec);
        result
    }

    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.with(|v| v.len()).unwrap_or(0)
    }

    /// Check if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.with(|v| v.is_empty()).unwrap_or(true)
    }

    /// Clear the vector
    pub fn clear(&self) {
        self.set(Vec::new());
    }
}

// impl iter for Signal where T is a vec
impl<T: SignalValue + Clone + 'static> Signal<Vec<T>> {
    /// Get an iterator over the vector contents (creates a snapshot)
    pub fn iter(&self) -> impl Iterator<Item = T> {
        self.get().into_iter()
    }

    /// Apply a function to each element and collect results
    pub fn map<R, F>(&self, f: F) -> Vec<R>
    where
        F: FnMut(T) -> R,
    {
        self.iter().map(f).collect()
    }

    /// Filter elements and return a new vector
    pub fn filter<F>(&self, f: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        self.iter().filter(f).collect()
    }
}

impl<T: SignalValue + 'static> Signal<T> {
    /// Access signal value immutably
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        if let Some(current_scope) = get_current_scope() {
            {
                let mut signal_deps = SIGNAL_DEPENDENCIES.lock();
                signal_deps
                    .entry(self.id)
                    .or_insert_with(BTreeSet::new)
                    .insert(current_scope);
            }
            {
                let mut scope_deps = SCOPE_DEPENDENCIES.lock();
                scope_deps
                    .entry(current_scope)
                    .or_insert_with(BTreeSet::new)
                    .insert(self.id);
            }
        }

        let signals = SIGNALS.lock();
        signals
            .get(&self.id)
            .and_then(|stored| {
                stored
                    .value
                    .as_any()
                    .and_then(|any| any.downcast_ref::<T>())
            })
            .map(|val| f(val))
    }

    /// Update signal value and trigger re-renders if changed
    pub fn set(&self, value: T)
    where
        T: PartialEq,
    {
        let mut changed = false;

        {
            let mut signals = SIGNALS.lock();
            if let Some(stored) = signals.get_mut(&self.id) {
                if let Some(current_val) = stored
                    .value
                    .as_any()
                    .and_then(|any| any.downcast_ref::<T>())
                {
                    if current_val != &value {
                        *stored = StoredValue {
                            value: Box::new(value),
                        };
                        changed = true;
                    }
                }
            }
        }

        if changed {
            {
                let mut changes = SCOPE_SIGNAL_CHANGES.lock();
                changes.insert(self.id);
            }

            if get_current_scope().is_none() {
                render_scope(self.id.0);
            }
        }
    }

    /// Get cloned value
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.with(|val| val.clone()).unwrap()
    }
}

struct StoredValue {
    value: Box<dyn SignalValue>,
}

//==============================================================================
// SIGNAL CREATION
//==============================================================================

/// Signal initialization options
pub enum SignalInit<T> {
    Value(T),
    InitFn(Box<dyn Fn() -> T + Send + 'static>),
}

impl<T: SignalValue> From<T> for SignalInit<T> {
    fn from(value: T) -> Self {
        SignalInit::Value(value)
    }
}

#[derive(Debug)]
pub enum SignalCreationError {
    OutsideScope,
}

impl core::fmt::Display for SignalCreationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Signals can only be created within a scope context")
    }
}

/// Create new signal within current scope
pub fn create_signal<T, I>(init: I) -> Signal<T>
where
    T: SignalValue + PartialEq + 'static,
    I: Into<SignalInit<T>>,
{
    let scope_id = get_current_scope()
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap(); // safe, we want to panic if not in scope
    let signal_id = get_next_signal_id_for_scope(scope_id);
    let signal = Signal {
        id: (scope_id, signal_id),
        _marker: PhantomData,
    };

    {
        let mut signals = SIGNALS.lock();
        if signals.get(&signal.id).is_none() {
            let initial_value = match init.into() {
                SignalInit::Value(v) => v,
                SignalInit::InitFn(f) => f(),
            };
            signals.insert(
                signal.id,
                StoredValue {
                    value: Box::new(initial_value),
                },
            );
        }
    }

    signal
}

//==============================================================================
// EFFECTS
//==============================================================================

#[derive(Clone, Copy, Debug)]
struct Effect {
    id: (usize, usize),
}

/// Create effect that runs when dependencies change
pub fn create_effect(effect: impl Fn() + Send + 'static) {
    let scope_id = get_current_scope()
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap(); // safe, we want to panic if not in scope
    let effect_id = get_next_effect_id_for_scope(scope_id);
    let effect_struct = Effect {
        id: (scope_id, effect_id),
    };

    {
        let mut effects = SCOPE_EFFECTS.lock();
        effects.insert(effect_struct.id, Box::new(effect));
    }
}

//==============================================================================
// SCOPE MANAGEMENT
//==============================================================================

/// Run function within new reactive scope
pub(crate) fn run_scope(
    scope_fn: impl FnMut() -> Node + Send + 'static,
    callback: impl Fn(&Node) + Send + Sync + 'static,
) -> Node {
    let scope_id = {
        let mut next_id = NEXT_SCOPE_ID.lock();
        let current = *next_id;
        *next_id = current + 1;
        current
    };

    {
        let mut scope_functions = SCOPE_FUNCTIONS.lock();
        scope_functions.insert(scope_id, Box::new(scope_fn));
    }

    {
        let mut scope_callbacks = SCOPE_CALLBACKS.lock();
        scope_callbacks.insert(scope_id, Arc::new(callback));
    }

    render_scope(scope_id)
}

//==============================================================================
// INTERNAL FUNCTIONS
//==============================================================================

fn get_current_scope() -> Option<usize> {
    *CURRENT_SCOPE.lock()
}

fn set_current_scope(scope_id: Option<usize>) {
    *CURRENT_SCOPE.lock() = scope_id;
}

fn get_next_signal_id_for_scope(scope_id: usize) -> usize {
    let mut counters = SCOPE_SIGNAL_COUNTERS.lock();
    let counter = counters.entry(scope_id).or_insert(0);
    *counter += 1;
    *counter
}

fn get_next_effect_id_for_scope(scope_id: usize) -> usize {
    let mut counters = SCOPE_EFFECT_COUNTERS.lock();
    let counter = counters.entry(scope_id).or_insert(0);
    *counter += 1;
    *counter
}

fn reset_signal_counters(scope_id: usize) {
    SCOPE_SIGNAL_COUNTERS.lock().remove(&scope_id);
}

fn reset_effect_counters(scope_id: usize) {
    SCOPE_EFFECT_COUNTERS.lock().remove(&scope_id);
}

struct ScopeGuard {
    previous_scope: Option<usize>,
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        set_current_scope(self.previous_scope);
    }
}

fn render_scope(scope_id: usize) -> Node {
    let _guard = ScopeGuard {
        previous_scope: get_current_scope(),
    };
    set_current_scope(Some(scope_id));

    let (should_clear_deps, was_rendering) = {
        let mut rendering_flag = RENDERING_SCOPE.lock();
        let mut changes = SCOPE_SIGNAL_CHANGES.lock();
        let was_rendering = *rendering_flag;
        *rendering_flag = scope_id;
        changes.clear();
        (was_rendering == 0, was_rendering)
    };

    if should_clear_deps {
        let signal_ids = {
            let mut scope_deps = SCOPE_DEPENDENCIES.lock();
            scope_deps.remove(&scope_id)
        };

        if let Some(signal_ids) = signal_ids {
            let mut signal_deps = SIGNAL_DEPENDENCIES.lock();
            for signal_id in signal_ids {
                if let Some(scopes) = signal_deps.get_mut(&signal_id) {
                    scopes.remove(&scope_id);
                }
            }
        }
    }

    let scope_fn = {
        let mut scope_functions = SCOPE_FUNCTIONS.lock();
        scope_functions.remove(&scope_id)
    };

    let node = scope_fn.map(|mut fnc| {
        let mut node = fnc();
        {
            let mut scope_functions = SCOPE_FUNCTIONS.lock();
            scope_functions.insert(scope_id, fnc);
        }
        if let Some(el) = node.as_element_mut() {
            el.key = scope_id.to_string();
        }
        node
    });

    if let Some(ref node) = node {
        let scope_callbacks = SCOPE_CALLBACKS.lock();
        if let Some(callback) = scope_callbacks.get(&scope_id) {
            callback(node);
        }
    }

    reset_signal_counters(scope_id);
    run_scope_effects(scope_id);
    reset_effect_counters(scope_id);

    let signal_changes = {
        let mut changes = SCOPE_SIGNAL_CHANGES.lock();
        let mut rendering_flag = RENDERING_SCOPE.lock();
        let result = if !changes.is_empty() {
            Some(core::mem::take(&mut *changes))
        } else {
            None
        };
        *rendering_flag = was_rendering;
        result
    };

    if let Some(changes) = signal_changes {
        {
            let mut pending = PENDING_SCOPE_RENDERS.lock();
            let signal_deps = SIGNAL_DEPENDENCIES.lock();
            for signal_id in changes {
                if let Some(dependent_scopes) = signal_deps.get(&signal_id) {
                    pending.extend(dependent_scopes.clone());
                }
            }
        }

        if was_rendering == 0 {
            process_pending_renders();
        }
    }

    node.unwrap_or(Node::Empty)
}

fn run_scope_effects(scope_id: usize) {
    let effects = SCOPE_EFFECTS.lock();
    effects
        .iter()
        .filter(|((effect_scope_id, _), _)| *effect_scope_id == scope_id)
        .for_each(|(_, effect)| effect());
}

fn process_pending_renders() {
    loop {
        let scope_to_render = {
            let mut pending = PENDING_SCOPE_RENDERS.lock();
            if pending.is_empty() {
                None
            } else {
                pending.iter().next().copied().map(|scope_id| {
                    pending.remove(&scope_id);
                    scope_id
                })
            }
        };

        match scope_to_render {
            Some(scope_id) => render_scope(scope_id),
            None => break,
        };
    }
}

//==============================================================================
// RESOURCE
//==============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceStatus {
    Idle,
    Pending,
    Loading,
    Resolved,
}

impl SignalValue for ResourceStatus {
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}

pub struct Resource<T> {
    status: Signal<ResourceStatus>,
    value: Signal<Option<T>>,
}

impl<T: SignalValue + PartialEq + 'static> Resource<T> {
    pub fn status(&self) -> Signal<ResourceStatus> {
        self.status
    }

    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.value.get()
    }

    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.value.with(|v| v.as_ref().map(f)).unwrap_or_default()
    }

    pub fn retry(&self) {
        self.status.set(ResourceStatus::Pending);
    }
}

#[allow(unused_variables)]
/// Create a resource that can be asynchronously loaded
pub fn create_resource<T, F>(fetcher: F) -> Resource<T>
where
    T: SignalValue + PartialEq + 'static,
    F: AsyncFn() -> T + Send + Clone + 'static,
    Option<T>: Copy,
{
    let value = create_signal(None);
    let status = create_signal(ResourceStatus::Idle);

    create_effect(move || {
        if status.get() == ResourceStatus::Idle || status.get() == ResourceStatus::Pending {
            status.set(ResourceStatus::Loading);

            #[cfg(feature = "wasm")]
            let fetcher = fetcher.clone();

            #[cfg(feature = "wasm")]
            wasm_bindgen_futures::spawn_local(async move {
                let val = fetcher().await;
                value.set(Some(val));
                status.set(ResourceStatus::Resolved);
            });
        }
    });

    Resource { status, value }
}

//==============================================================================
// TESTS
//==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::sync::Arc;
    use core::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_nested_scopes() {
        run_scope(
            || {
                let outer_signal = create_signal(0);

                run_scope(
                    move || {
                        let inner_signal = create_signal("hello");
                        assert!(inner_signal.get() == "hello");
                        outer_signal.set(42);
                        Node::Empty
                    },
                    |_| {},
                );

                assert_eq!(outer_signal.get(), 42);
                Node::Empty
            },
            |_| {},
        );
    }

    #[test]
    fn test_signal_and_effect_in_scope() {
        run_scope(
            move || {
                let effect_count = Arc::new(AtomicUsize::new(0));
                let effect_count_clone = effect_count.clone();
                let signal = create_signal(0);

                create_effect(move || {
                    let _ = signal.get();
                    effect_count_clone.fetch_add(1, Ordering::SeqCst);
                    assert!(effect_count.load(Ordering::SeqCst) > 0);
                    signal.set(1);
                });

                Node::Empty
            },
            |_| {},
        );
    }

    #[test]
    fn test_multiple_signals_and_dependencies() {
        run_scope(
            || {
                let signal1 = create_signal("hello");
                let signal2 = create_signal(0);

                create_effect(move || {
                    let _str_val = signal1.get();
                    let _num_val = signal2.get();
                });

                signal1.set("world");
                signal2.set(42);

                assert_eq!(signal1.get(), "world");
                assert_eq!(signal2.get(), 42);

                Node::Empty
            },
            |_| {},
        );
    }
}
