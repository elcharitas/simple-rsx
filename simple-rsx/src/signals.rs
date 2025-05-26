use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::Node;

thread_local! {
    // Track the stack of active scopes
    static SCOPE_STACK: RefCell<Vec<usize>> = RefCell::new(Vec::new());

    // Track the current scope being executed
    static CURRENT_SCOPE: RefCell<Option<usize>> = RefCell::new(None);

    // Track if we're currently inside a scope render
    static RENDERING_SCOPE: RefCell<usize> = RefCell::new(0);

    // Store next scope ID
    static NEXT_SCOPE_ID: RefCell<usize> = RefCell::new(1);

    // Store signal counter for each scope
    static SCOPE_SIGNAL_COUNTERS: RefCell<HashMap<usize, usize>> = RefCell::new(HashMap::new());

    // Track signals that changed during current scope execution (for batching)
    static SCOPE_SIGNAL_CHANGES: RefCell<HashSet<(usize, usize)>> = RefCell::new(HashSet::new());

    static SIGNALS: RefCell<HashMap<(usize, usize), SignalValue>> = RefCell::new(HashMap::new());

    // Store scope functions that can be re-executed
    static SCOPE_FUNCTIONS: RefCell<HashMap<usize, Box<dyn FnMut() -> Node>>> = RefCell::new(HashMap::new());

    static SCOPE_DEPENDENCIES: RefCell<HashMap<usize, HashSet<(usize, usize)>>> = RefCell::new(HashMap::new());

    // Store scope functions that can be re-executed
    static SCOPE_CALLBACKS: RefCell<HashMap<usize, Arc<dyn Fn(&Node)>>> = RefCell::new(HashMap::new());

    // Store next effect ID for each scope
    static SCOPE_EFFECT_COUNTERS: RefCell<HashMap<usize, usize>> = RefCell::new(HashMap::new());

    // Store effects with their IDs
    static SCOPE_EFFECTS: RefCell<HashMap<(usize, usize), Box<dyn Fn()>>> = RefCell::new(HashMap::new());

    // Track which scopes depend on which signals
    static SIGNAL_DEPENDENCIES: RefCell<HashMap<(usize, usize), HashSet<usize>>> = RefCell::new(HashMap::new());

    // Queue for scopes that need to re-render
    static PENDING_SCOPE_RENDERS: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());
}

pub trait DynamicValue {
    fn as_any(&self) -> Option<&dyn std::any::Any>;
}

impl DynamicValue for String {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for &'static str {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for i64 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for i128 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for i16 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for i32 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for i8 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for usize {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for u64 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for u128 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for u16 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for u32 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for u8 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for f64 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for f32 {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for bool {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for char {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl DynamicValue for () {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl<T: DynamicValue + 'static> DynamicValue for Option<T> {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Signal<T> {
    id: (usize, usize),
    _marker: std::marker::PhantomData<T>,
}

struct SignalValue {
    value: Box<dyn DynamicValue>,
}

impl<T: DynamicValue + PartialEq + Clone + 'static> Signal<T> {
    pub fn set(&self, value: T) {
        let mut changed = false;
        // Update the signal value
        SIGNALS.with(|signals| {
            if let Some(stored) = signals.borrow_mut().get_mut(&self.id) {
                // Only update if the value actually changed
                if let Some(should_update) = stored
                    .value
                    .as_any()
                    .and_then(|any| any.downcast_ref::<T>().and_then(|val| Some(val != &value)))
                    .or(Some(false))
                {
                    if should_update {
                        *stored = SignalValue {
                            value: Box::new(value),
                        };
                        changed = true;
                    }
                }
            }
        });

        if changed {
            // Just collect the signal change, don't trigger re-renders yet
            SCOPE_SIGNAL_CHANGES.with(|changes| {
                let mut changes = changes.borrow_mut();
                changes.insert(self.id);
            });
        }
    }
}

impl<T: DynamicValue + PartialEq + Clone + 'static> Signal<T> {
    pub fn get(&self) -> T {
        if let Some(current_scope) = get_current_scope() {
            // Track both signal->scope and scope->signal dependencies
            SIGNAL_DEPENDENCIES.with(|deps| {
                deps.borrow_mut()
                    .entry(self.id)
                    .or_insert_with(HashSet::new)
                    .insert(current_scope);
            });

            SCOPE_DEPENDENCIES.with(|deps| {
                deps.borrow_mut()
                    .entry(current_scope)
                    .or_insert_with(HashSet::new)
                    .insert(self.id);
            });
        }

        SIGNALS.with(|signals| {
            signals
                .borrow()
                .get(&self.id)
                .and_then(|stored| {
                    stored
                        .value
                        .as_any()
                        .and_then(|any| any.downcast_ref::<T>())
                        .map(|val| val.clone())
                })
                .unwrap()
        })
    }
}

#[derive(Debug)]
pub enum SignalCreationError {
    OutsideScope,
}

impl std::fmt::Display for SignalCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalCreationError::OutsideScope => {
                write!(f, "Signals can only be created within a scope context")
            }
        }
    }
}

impl std::error::Error for SignalCreationError {}

fn get_current_scope() -> Option<usize> {
    CURRENT_SCOPE.with(|scope| *scope.borrow())
}

fn set_current_scope(scope_id: Option<usize>) {
    CURRENT_SCOPE.with(|scope| {
        *scope.borrow_mut() = scope_id;
    });
    if let Some(id) = scope_id {
        SCOPE_STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            if !stack.contains(&id) {
                stack.push(id);
            }
        });
    }
}

fn get_next_signal_id_for_scope(scope_id: usize) -> usize {
    SCOPE_SIGNAL_COUNTERS.with(|counters| {
        let mut counters = counters.borrow_mut();
        let counter = counters.entry(scope_id).or_insert(0);
        *counter += 1;
        *counter
    })
}

fn reset_signal_counters(scope_id: usize) {
    SCOPE_SIGNAL_COUNTERS.with(|counters| {
        counters.borrow_mut().remove(&scope_id);
    });
}

fn process_pending_renders() {
    loop {
        let scope_to_render = PENDING_SCOPE_RENDERS.with(|pending| {
            let mut pending = pending.borrow_mut();
            if pending.is_empty() {
                None
            } else {
                // Take one scope at a time to avoid holding the borrow
                pending.iter().next().copied().map(|scope_id| {
                    pending.remove(&scope_id);
                    scope_id
                })
            }
        });

        match scope_to_render {
            Some(scope_id) => render_scope(scope_id),
            None => break,
        };
    }
}

struct ScopeGuard {
    previous_scope: Option<usize>,
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        set_current_scope(self.previous_scope);
    }
}

fn render_scope(scope_id: usize) -> Option<Node> {
    // Create a scope guard that will restore the previous scope when dropped
    let _guard = ScopeGuard {
        previous_scope: get_current_scope(),
    };

    // Set the current scope for rendering
    set_current_scope(Some(scope_id));

    // Batch all pre-render operations
    let (should_clear_deps, was_rendering) = RENDERING_SCOPE.with(|flag| {
        SCOPE_SIGNAL_CHANGES.with(|changes| {
            let mut flag = flag.borrow_mut();
            let was_rendering = *flag;
            *flag = scope_id;

            let mut changes = changes.borrow_mut();
            changes.clear();

            (was_rendering == 0, was_rendering)
        })
    });

    // Only clear dependencies if this is not a nested render
    if should_clear_deps {
        // Clear only this scope's dependencies efficiently
        SCOPE_DEPENDENCIES.with(|scope_deps| {
            if let Some(signal_ids) = scope_deps.borrow_mut().remove(&scope_id) {
                // Remove this scope from each signal's dependency list
                SIGNAL_DEPENDENCIES.with(|signal_deps| {
                    let mut signal_deps = signal_deps.borrow_mut();
                    for signal_id in signal_ids {
                        if let Some(scopes) = signal_deps.get_mut(&signal_id) {
                            scopes.remove(&scope_id);
                        }
                    }
                });
            }
        });
    }

    // Execute the scope function without removing it
    let scope_fn = SCOPE_FUNCTIONS.with(|scope_functions| {
        let mut scope_functions = scope_functions.borrow_mut();
        scope_functions.remove(&scope_id)
    });

    let node = scope_fn.map(|mut fnc| {
        let node = fnc();
        SCOPE_FUNCTIONS.with_borrow_mut(|scope_functions| {
            scope_functions.insert(scope_id, fnc);
        });
        node
    });

    if let Some(ref node) = node {
        SCOPE_CALLBACKS.with(|scope_callbacks| {
            scope_callbacks
                .borrow()
                .get(&scope_id)
                .and_then(|callback| Some(callback(node)));
        });
    }

    reset_signal_counters(scope_id);
    run_scope_effects(scope_id);
    reset_effect_counters(scope_id);

    let signal_changes = SCOPE_SIGNAL_CHANGES.with(|stored_changes| {
        RENDERING_SCOPE.with(|flag| {
            let mut changes = stored_changes.borrow_mut();
            let mut flag = flag.borrow_mut();

            let result = if !changes.is_empty() {
                let collected = std::mem::take(&mut *changes);
                Some(collected)
            } else {
                None
            };

            *flag = was_rendering;
            result
        })
    });

    if let Some(changes) = signal_changes {
        PENDING_SCOPE_RENDERS.with(|pending| {
            let mut pending = pending.borrow_mut();
            for signal_id in changes {
                if let Some(dependent_scopes) =
                    SIGNAL_DEPENDENCIES.with(|deps| deps.borrow().get(&signal_id).cloned())
                {
                    pending.extend(dependent_scopes);
                }
            }
        });

        if was_rendering == 0 {
            process_pending_renders();
        }
    }

    node
}

fn run_scope_effects(scope_id: usize) {
    SCOPE_EFFECTS.with(|effects| {
        let effects = effects.borrow();
        effects
            .iter()
            .filter(|((effect_scope_id, _), _)| *effect_scope_id == scope_id)
            .for_each(|(_, effect)| effect());
    });
}

pub fn create_signal<T: DynamicValue + PartialEq + 'static>(initial_value: T) -> Signal<T> {
    let scope_id = get_current_scope()
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap();

    let signal_id = get_next_signal_id_for_scope(scope_id);
    let signal = Signal {
        id: (scope_id, signal_id),
        _marker: std::marker::PhantomData,
    };

    SIGNALS.with(|signals| {
        if signals.borrow_mut().get_mut(&signal.id).is_none() {
            signals.borrow_mut().insert(
                signal.id,
                SignalValue {
                    value: Box::new(initial_value),
                },
            );
        }
    });

    signal
}

#[derive(Clone, Copy, Debug)]
struct Effect {
    id: (usize, usize),
}

fn get_next_effect_id_for_scope(scope_id: usize) -> usize {
    SCOPE_EFFECT_COUNTERS.with(|counters| {
        let mut counters = counters.borrow_mut();
        let counter = counters.entry(scope_id).or_insert(0);
        *counter += 1;
        *counter
    })
}

fn reset_effect_counters(scope_id: usize) {
    SCOPE_EFFECT_COUNTERS.with(|counters| {
        counters.borrow_mut().remove(&scope_id);
    });
}

pub fn create_effect(effect: impl Fn() + 'static) {
    let scope_id = get_current_scope()
        .ok_or(SignalCreationError::OutsideScope)
        .unwrap();

    let effect_id = get_next_effect_id_for_scope(scope_id);
    let effect_struct = Effect {
        id: (scope_id, effect_id),
    };

    SCOPE_EFFECTS.with(|effects| {
        effects
            .borrow_mut()
            .insert(effect_struct.id, Box::new(effect));
    });
}

pub(crate) fn run_scope(
    scope_fn: impl FnMut() -> Node + 'static,
    callback: impl Fn(&Node) + 'static,
) -> Option<Node> {
    // Get next scope ID
    let scope_id = NEXT_SCOPE_ID.with(|id| {
        if let Ok(mut id) = id.try_borrow_mut() {
            let current = *id;
            *id = current + 1;
            current
        } else {
            panic!("Failed to get next scope ID")
        }
    });

    // Store the scope function so it can be re-executed
    SCOPE_FUNCTIONS.with(|scope_functions| {
        let mut scope_functions = scope_functions.borrow_mut();
        scope_functions.insert(scope_id, Box::new(scope_fn));
    });

    SCOPE_CALLBACKS.with(|scope_callbacks| {
        let mut scope_callbacks = scope_callbacks.borrow_mut();
        scope_callbacks.insert(scope_id, Arc::new(callback));
    });

    // Initial render of the scope
    render_scope(scope_id)
}

// Helper function to manually trigger all scopes to re-render (useful for debugging)
pub fn rerender_all_scopes() {
    SCOPE_FUNCTIONS.with(|scope_functions| {
        let scope_functions = scope_functions.borrow();
        for scope_id in scope_functions.keys().cloned() {
            render_scope(scope_id);
        }
    });

    process_pending_renders();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_nested_scopes() {
        run_scope(
            || {
                let outer_signal = create_signal(0);

                run_scope(
                    move || {
                        let inner_signal = create_signal("hello");
                        assert!(inner_signal.get() == "hello");
                        outer_signal.set(42); // Can access outer scope's signals
                        Node::Empty
                    },
                    |_| {},
                );

                // assert_ne!(outer_scope_id, inner_scope_id);
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
                    // Effect should run once initially
                    assert!(effect_count.load(Ordering::SeqCst) > 0);
                    // Update signal value
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
                    let str_val = signal1.get();
                    let num_val = signal2.get();

                    println!("Effect running with values: {}, {}", str_val, num_val);
                });

                signal1.set("world");
                signal2.set(42);

                // Verify final values
                assert_eq!(signal1.get(), "world");
                assert_eq!(signal2.get(), 42);

                Node::Empty
            },
            |_| {},
        );
    }
}
