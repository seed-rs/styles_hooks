use seed::prelude::*;
use crate::reactive_state_access::{ReactiveStateAccess, CloneReactiveState};

impl<Ms : 'static,U,A> IntoNodes<Ms> for ReactiveStateAccess<Vec<Node<Ms>>,U,A> {
    fn into_nodes(self) -> Vec<Node<Ms>> {
        self.get()
    }
}