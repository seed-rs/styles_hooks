use crate::hooks_state_functions::use_state;
use crate::reactive_state_access::state_access::{CloneState, StateAccess};

/// call the provided function once and once only
/// returns a unmmunt which will allow the do_once
/// to repeat if .execute_if_activated() is called on the unmount.
/// Example
///
/// do_once(||{
///     println!("This will print only once");
/// });
#[topo::nested]
pub fn do_once<F: FnMut() -> ()>(mut func: F) -> StateAccess<bool> {
    let has_done = use_state(|| false);
    if !has_done.get() {
        func();
        has_done.set(true);
    }
    has_done
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Local(topo::CallId);

impl std::fmt::Display for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl Local {
    #[topo::nested]
    pub fn new() -> Local {
        Local(topo::CallId::current())
    }
}

/// A value unique to the source location where it is created.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CallSite {
    location: usize,
}

impl CallSite {
    /// Constructs a callsite whose value is unique to the source location at
    /// which it is called.
    #[track_caller]
    pub fn here() -> Self {
        Self {
            // the pointer value for a given location is enough to differentiate it from all others
            location: std::panic::Location::caller() as *const _ as usize,
        }
    }

    #[track_caller]
    pub fn loc() -> String {
        std::panic::Location::caller().to_string()
    }
}
