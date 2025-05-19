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
    static RENDERING_SCOPE: RefCell<bool> = RefCell::new(false);

    // Store next scope ID
    static NEXT_SCOPE_ID: RefCell<usize> = RefCell::new(1);

    // Store signal counter for each scope
    static SCOPE_SIGNAL_COUNTERS: RefCell<HashMap<usize, usize>> = RefCell::new(HashMap::new());

    // Track signals that changed during current scope execution (for batching)
    static SCOPE_SIGNAL_CHANGES: RefCell<HashSet<(usize, usize)>> = RefCell::new(HashSet::new());

    static SIGNALS: RefCell<HashMap<(usize, usize), SignalValue>> = RefCell::new(HashMap::new());

    // Store scope functions that can be re-executed
    static SCOPE_FUNCTIONS: RefCell<HashMap<usize, Arc<dyn Fn() -> Node + Send>>> = RefCell::new(HashMap::new());

    // Store next effect ID for each scope
    static SCOPE_EFFECT_COUNTERS: RefCell<HashMap<usize, usize>> = RefCell::new(HashMap::new());

    // Store effects with their IDs
    static SCOPE_EFFECTS: RefCell<HashMap<(usize, usize), Box<dyn Fn() + Send + Sync>>> = RefCell::new(HashMap::new());

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
            SCOPE_SIGNAL_CHANGES.with(|changes| {
                changes.borrow_mut().insert(self.id);
            });
        }
    }

    pub fn get(&self) -> Option<T> {
        if let Some(current_scope) = get_current_scope() {
            SIGNAL_DEPENDENCIES.with(|deps| {
                let mut deps = deps.borrow_mut();
                let scopes = deps.entry(self.id).or_insert_with(HashSet::new);
                scopes.insert(current_scope);
            });
        }

        SIGNALS.with(|signals| {
            if let Some(stored) = signals.borrow().get(&self.id) {
                if let Some(parsed) = stored
                    .value
                    .as_any()
                    .and_then(|any| any.downcast_ref::<T>())
                {
                    return Some(parsed.clone());
                }
            }
            None
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

fn schedule_dependent_scopes_for_rerender(signal_id: (usize, usize)) {
    let dependent_scopes = SIGNAL_DEPENDENCIES.with(|deps| {
        if let Ok(deps) = deps.try_borrow() {
            deps.get(&signal_id).cloned().unwrap_or_default()
        } else {
            HashSet::new()
        }
    });

    PENDING_SCOPE_RENDERS.with(|pending| {
        if let Ok(mut pending) = pending.try_borrow_mut() {
            for scope_id in dependent_scopes {
                pending.insert(scope_id);
            }
        }
    });
}

fn process_pending_renders() {
    loop {
        let scopes_to_render = PENDING_SCOPE_RENDERS.with(|pending| {
            if let Ok(mut pending) = pending.try_borrow_mut() {
                if pending.is_empty() {
                    return Vec::new();
                }
                let scopes = pending.iter().copied().collect::<Vec<_>>();
                pending.clear();
                scopes
            } else {
                Vec::new()
            }
        });

        if scopes_to_render.is_empty() {
            break;
        }

        for scope_id in scopes_to_render {
            render_scope(scope_id);
        }
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

    // Clear dependencies for this scope
    SIGNAL_DEPENDENCIES.with(|deps| {
        if let Ok(mut deps) = deps.try_borrow_mut() {
            for (_, scopes) in deps.iter_mut() {
                scopes.remove(&scope_id);
            }
        }
    });

    // Set rendering flag and clear changes
    RENDERING_SCOPE.with(|flag| {
        if let Ok(mut flag) = flag.try_borrow_mut() {
            *flag = true;
        }
    });

    SCOPE_SIGNAL_CHANGES.with(|changes| {
        if let Ok(mut changes) = changes.try_borrow_mut() {
            changes.clear();
        }
    });

    // Execute the scope function
    let scope_fn = SCOPE_FUNCTIONS.with(|scope_functions| {
        let scope_functions = scope_functions.borrow();
        if let Some(scope_fn) = scope_functions.get(&scope_id) {
            return Some(scope_fn.clone());
        }
        return None;
    });

    let mut node = None;

    if let Some(scope_fn) = scope_fn {
        node = Some(scope_fn());
    }

    reset_signal_counters(scope_id);
    run_scope_effects(scope_id);
    reset_effect_counters(scope_id);

    // Collect signal changes
    let signal_changes = SCOPE_SIGNAL_CHANGES.with(|stored_changes| {
        if let Ok(mut changes) = stored_changes.try_borrow_mut() {
            let collected = changes.clone();
            changes.clear();
            collected
        } else {
            HashSet::new()
        }
    });

    RENDERING_SCOPE.with(|flag| {
        if let Ok(mut flag) = flag.try_borrow_mut() {
            *flag = false;
        }
    });

    // Schedule dependent scopes for rerender
    for signal_id in signal_changes {
        schedule_dependent_scopes_for_rerender(signal_id);
    }

    node
    // Guard will automatically restore previous scope when dropped
}

fn run_scope_effects(scope_id: usize) {
    SCOPE_EFFECTS.with(|effects| {
        let effects = effects.borrow();
        for (&(effect_scope_id, _), effect) in effects.iter() {
            if effect_scope_id == scope_id {
                effect();
            }
        }
    });
}

pub fn create_signal<T: DynamicValue + PartialEq + 'static>(
    initial_value: T,
) -> Result<Signal<T>, SignalCreationError> {
    let scope_id = get_current_scope().ok_or(SignalCreationError::OutsideScope)?;

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

    Ok(signal)
}

#[derive(Clone, Copy, Debug)]
pub struct Effect {
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

pub fn create_effect(
    effect: impl Fn() + Send + Sync + 'static,
) -> Result<Effect, SignalCreationError> {
    let scope_id = get_current_scope().ok_or(SignalCreationError::OutsideScope)?;

    let effect_id = get_next_effect_id_for_scope(scope_id);
    let effect_struct = Effect {
        id: (scope_id, effect_id),
    };

    SCOPE_EFFECTS.with(|effects| {
        effects
            .borrow_mut()
            .insert(effect_struct.id, Box::new(effect));
    });

    Ok(effect_struct)
}

pub fn run_scope(scope_fn: impl Fn() -> Node + Send + Sync + 'static) -> Option<Node> {
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
        scope_functions.insert(scope_id, Arc::new(scope_fn));
    });

    // Initial render of the scope
    let node = render_scope(scope_id);

    // Process any pending renders that might have been triggered
    process_pending_renders();

    node
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
        run_scope(|| {
            let outer_signal = create_signal(0).unwrap();

            run_scope(move || {
                let inner_signal = create_signal("hello").unwrap();
                assert!(inner_signal.get().is_some());
                outer_signal.set(42); // Can access outer scope's signals
                Node::Empty
            });

            // assert_ne!(outer_scope_id, inner_scope_id);
            assert_eq!(outer_signal.get(), Some(42));

            Node::Empty
        });
    }

    #[test]
    fn test_signal_and_effect_in_scope() {
        run_scope(move || {
            let effect_count = Arc::new(AtomicUsize::new(0));
            let effect_count_clone = effect_count.clone();
            let signal = create_signal(0).unwrap();

            create_effect(move || {
                let _ = signal.get();
                effect_count_clone.fetch_add(1, Ordering::SeqCst);
                // Effect should run once initially
                assert!(effect_count.load(Ordering::SeqCst) > 0);
                // Update signal value
                signal.set(1);
            })
            .unwrap();

            Node::Empty
        });
    }

    #[test]
    fn test_multiple_signals_and_dependencies() {
        run_scope(|| {
            let signal1 = create_signal("hello").unwrap();
            let signal2 = create_signal(0).unwrap();

            create_effect(move || {
                let str_val = signal1.get().unwrap_or_default();
                let num_val = signal2.get().unwrap_or_default();

                println!("Effect running with values: {}, {}", str_val, num_val);
            })
            .unwrap();

            signal1.set("world");
            signal2.set(42);

            // Verify final values
            assert_eq!(signal1.get().unwrap(), "world");
            assert_eq!(signal2.get().unwrap(), 42);

            Node::Empty
        });
    }

    #[test]
    fn test_signal_creation_outside_scope() {
        let result = create_signal(0);
        assert!(matches!(result, Err(SignalCreationError::OutsideScope)));

        let effect_result = create_effect(|| {});
        assert!(matches!(
            effect_result,
            Err(SignalCreationError::OutsideScope)
        ));
    }
}
