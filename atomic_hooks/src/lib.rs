pub use atomic_hooks_macros::{atom, reaction};
// storage
pub mod store;

// hooks
mod hooks_state_functions;

// reactive state

mod marker;
mod reactive_state_access;
pub mod reactive_state_functions;

// helpers
mod helpers;
// mod seed_integration;
pub mod reverse;

// public exports
mod prelude;
pub use prelude::*;
pub mod unmount;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
