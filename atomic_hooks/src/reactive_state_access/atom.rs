/// An atom is an observable and changeable piece of state.
/// You can use it to update a component and render specific part of the DOM.
pub struct Atom<T> {
    pub id: StorageKey,
    pub _phantom_data_stored_type: PhantomData<T>,
}

impl<T> std::fmt::Debug for Atom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Clone for Atom<T> {
    fn clone(&self) -> Atom<T> {
        Atom::<T> {
            id: self.id, // is is not supposed to be unique ?

            _phantom_data_stored_type: PhantomData::<T>,
        }
    }
}

impl<T> Copy for Atom<T> {}

impl<T> Atom<T>
where
    T: 'static,
{
    /// Instantiate a new atom.
    pub fn new(id: StorageKey) -> Atom<T> {
        Atom {
            id,
            _phantom_data_stored_type: PhantomData,
        }
    }

    /// Stores a value of type T in a backing Store **without** reaction for
    /// observers.
    ///
    /// ```
    /// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    /// #[reaction]
    /// fn reaction_a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     a - b
    /// }
    /// a().inert_set(1);
    /// let diff = reaction_a_b_subtraction();
    /// assert_eq!(a().get(), 1);
    /// assert_eq!(
    ///     diff.get(),
    ///     0,
    ///     "We should still get 0 since we use inert setting"
    /// );
    /// ```
    ///   ## Todo doc
    /// - Add a description that explains relevant use case for this method
    pub fn inert_set(self, value: T)
    where
        T: 'static,
    {
        set_inert_atom_state_with_id(value, self.id);
    }
    /// Stores a value of type T in a backing Store **with** a reaction for
    /// observers.  
    ///
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    /// 0
    /// }
    ///
    /// a().set(1);
    ///
    /// assert_eq!(a().get(), 1);
    ///
    ///  ```
    /// - add example maybe
    /// - When to use it
    pub fn set(self, value: T)
    where
        T: 'static,
    {
        set_atom_state_with_id(value, self.id);
    }

    /// Pass a function that update the atom state related
    /// This update will trigger reactions and observers will get the update
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    /// a().update(|state| *state = 45);
    /// assert_eq!(a().get(), 45, "We should get 45 as value for a");
    /// ```
    ///
    /// This is use for example when we want to update a component rendering
    /// depending of a state. We update the atom so the component will
    /// rerender with the new state. If many components subscribed to the
    /// atom, then all of them will get the update.
    pub fn update<F: FnOnce(&mut T) -> ()>(&self, func: F)
    where
        T: 'static,
    {
        update_atom_state_with_id(self.id, func);
    }

    /// Use to remove an atom from the global state
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// a().remove();
    ///
    /// assert_eq!(a().state_exists(), false, "The a state should not exist");
    /// ```
    pub fn remove(self) -> Option<T> {
        remove_reactive_state_with_id(self.id)
    }
    /// ## Question :
    /// Why do we have remove and delete ?
    ///  
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// a().delete();
    ///
    /// assert_eq!(a().state_exists(), false, "The a state should not exist");
    /// ```
    /// ## Todo doc
    /// - When to use it
    pub fn delete(self) {
        self.remove();
    }
    /// Reset to the initial value
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// a().set(10);
    /// a().reset_to_default();
    ///
    /// assert_eq!(a().get(), 0, "The a state be reset to initial value");
    /// ```
    pub fn reset_to_default(&self) {
        (clone_reactive_state_with_id::<RxFunc>(self.id)
            .unwrap()
            .func)();
        execute_reaction_nodes(&self.id);
    }

    /// Check if the state does exist in the store.
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// a().set(10);
    /// a().delete();
    ///
    /// assert_eq!(a().state_exists(), false, "The a state should not exist");
    /// ```
    pub fn state_exists(self) -> bool {
        reactive_state_exists_for_id::<T>(self.id)
    }

    /// Allow you to get the state through a reference with a closure.
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    /// a().set(3);
    ///
    /// a().get_with(|v| assert_eq!(v, &3, "We should get 3"));
    /// ```
    ///  ## Todo doc
    /// - When to use it
    pub fn get_with<F: FnOnce(&T) -> R, R>(&self, func: F) -> R {
        read_reactive_state_with_id(self.id, func)
    }

    /// Triggers the passed function when the atom is updated
    /// This method needs to be used inside a function body that has the
    /// attributes **[reaction]**.
    ///
    /// ```
    /// use atomic_hooks::reaction::Reaction;
    /// #[reaction]
    /// fn count_print_when_update() -> Reaction<i32> {
    ///     let c = c();
    ///     let update = a().on_update(|| {
    ///         println!("UPDATE !!!");
    ///         c.update(|mut v| *v = *v + 1)
    ///     });
    ///     c.get()
    /// }
    ///
    /// #[test]
    /// fn test_on_update() {
    ///     let print = count_print_when_update();
    ///     a().update(|v| *v = 32);
    ///     a().set(2);
    ///     a().set(25);
    ///     a().set(1);
    ///
    ///     println!("{:?}", print.get());
    ///
    ///     assert_eq!(print.get(), 5)
    /// }
    /// ```
    ///
    ///  ## Todo doc
    /// - When to use it
    /// - Improve the doc
    pub fn on_update<F: FnOnce() -> R, R>(&self, func: F) -> Option<R> {
        let mut recalc = false;
        self.observe_with(|_| recalc = true);
        if recalc {
            Some(func())
        } else {
            None
        }
    }
}

impl<T> Observable<T> for Atom<T>
where
    T: 'static,
{
    fn id(&self) -> StorageKey {
        self.id
    }
}
// The below is broke as need None if no prior state
impl<T> ObserveChangeReactiveState<T> for Atom<T>
where
    T: Clone + 'static + PartialEq,
{
    /// Let you get the last changes on an Atom state
    ///
    /// ## Todo
    /// - Improve the name of the method, because user might be expecting having
    ///   an observable while in fact the value from this method does not update
    ///   change over time but give only the last change.
    /// - the unit is failling for this method because option gives always None
    ///   as value.
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[test]
    /// fn test_observe_on_atom() {
    ///     let a = a();
    ///     let change = a.observe_change();
    ///     assert_eq!(change.0.is_none(), true);
    ///     assert_eq!(change.1, 0);
    ///     a.set(1);
    ///     let change2 = a.observe_change();
    ///     assert_eq!(change2.0.unwrap(), 0);
    ///     assert_eq!(change2.1, 1);
    /// }
    /// ```
    #[topo::nested]
    fn observe_change(&self) -> (Option<T>, T) {
        let previous_value_access = crate::hooks_state_functions::use_state(|| self.get());
        previous_value_access.get_with(|previous_value| {
            self.observe_with(|new_value| {
                if *previous_value != *new_value {
                    previous_value_access.set(new_value.clone());
                    (Some(previous_value.clone()), new_value.clone())
                } else {
                    (None, new_value.clone())
                }
            })
        })
    }
    /// This method gives us the possibility to know if the state of an atom has
    /// been changed.
    ///
    /// ## Todo
    /// - the unit test is failling for this method
    ///
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[test]
    /// fn test_has_changed_on_atom() {
    ///     let a = a();
    ///     a.set(1);
    ///
    ///     assert_eq!(a.has_changed(), true);
    ///     a.set(1);
    ///     assert_eq!(a.has_changed(), false);
    /// }
    /// ```
    #[topo::nested]
    fn has_changed(&self) -> bool {
        let previous_value_access = crate::hooks_state_functions::use_state(|| self.get());
        previous_value_access
            .get_with(|previous_value| self.observe_with(|new_value| new_value != previous_value))
    }
    /// This method gives the opportunity to trigger a function and use the
    /// values from the changes.
    ///
    /// ## Todo
    /// - the unit test is failling for this method after a.set(2)
    ///
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[test]
    /// fn test_on_changes_on_atom() {
    ///     let a = a();
    ///     let mut previous = 99;
    ///     let mut current = 99;
    ///     a.on_change(|p, c| {
    ///         previous = *p;
    ///         current = *c;
    ///     });
    ///     assert_eq!(previous, 0); //todo : should we expect None when init ?
    ///     assert_eq!(current, 0);
    ///     a.set(1);
    ///     a.on_change(|p, c| {
    ///         previous = *p;
    ///         current = *c;
    ///     });
    ///     assert_eq!(previous, 0);
    ///     assert_eq!(current, 1);
    ///     a.set(1);
    ///     a.on_change(|p, c| {
    ///         previous = *p;
    ///         current = *c;
    ///     });
    ///     assert_eq!(previous, 0);
    ///     assert_eq!(current, 1);
    ///     a.set(2);
    ///     a.on_change(|p, c| {
    ///         previous = *p;
    ///         current = *c;
    ///     });
    ///     assert_eq!(previous, 1, "we should get 1");
    ///     assert_eq!(current, 2, "we should get 2");
    /// }
    /// ```
    fn on_change<F: FnOnce(&T, &T) -> R, R>(&self, func: F) -> R {
        let previous_value_access = crate::hooks_state_functions::use_state(|| self.get());
        previous_value_access.get_with(|previous_value| {
            self.observe_with(|new_value| func(previous_value, new_value))
        })
    }
}
impl<T> CloneReactiveState<T> for Atom<T>
where
    T: Clone + 'static,
{
    /// returns a clone of the stored state panics if not stored.
    fn get(&self) -> T {
        clone_reactive_state_with_id::<T>(self.id).expect("state should be present")
    }

    fn soft_get(&self) -> Option<T> {
        clone_reactive_state_with_id::<T>(self.id)
    }
}
// If the underlying type provides display then so does the ReactiveStateAccess
impl<T> std::fmt::Display for Atom<T>
where
    T: std::fmt::Display + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get_with(|t| write!(f, "{}", t))
    }
}

use crate::{
    clone_reactive_state_with_id,
    reactive_state_access::{CloneReactiveState, ObserveChangeReactiveState},
    reactive_state_exists_for_id,
    reactive_state_functions::{execute_reaction_nodes, set_atom_state_with_id},
    read_reactive_state_with_id, remove_reactive_state_with_id, set_inert_atom_state_with_id,
    store::StorageKey,
    update_atom_state_with_id, Observable, RxFunc,
};
use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

impl<T> Add for Atom<T>
where
    T: Copy + Add<Output = T> + 'static,
{
    type Output = T;

    fn add(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o + *s))
    }
}

impl<T> Mul for Atom<T>
where
    T: Copy + Mul<Output = T> + 'static,
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o * *s))
    }
}

impl<T> Div for Atom<T>
where
    T: Copy + Div<Output = T> + 'static,
{
    type Output = T;

    fn div(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o / *s))
    }
}

impl<T> Sub for Atom<T>
where
    T: Copy + Sub<Output = T> + 'static,
{
    type Output = T;

    fn sub(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o - *s))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        reactive_state_access::{atom::Atom, reaction::Reaction},
        *,
    };

    #[atom]
    fn a() -> Atom<i32> {
        0
    }

    #[atom]
    fn b() -> Atom<i32> {
        0
    }

    #[reaction]
    fn a_b_subtraction() -> Reaction<i32> {
        let a = a().observe();
        let b = b().observe();
        a - b
    }

    #[atom]
    fn c() -> Atom<i32> {
        0
    }

    #[reaction]
    fn count_print_when_update() -> Reaction<i32> {
        let c = c();
        let _update = a().on_update(|| {
            println!("UPDATE !!!");
            c.update(|v| *v += 1)
        });
        c.get()
    }

    #[reaction]
    fn count_subtraction_when_update() -> Reaction<i32> {
        let c = c();
        let _update = a_b_subtraction().on_update(|| {
            println!("UPDATE !!!");
            c.update(|v| *v += 1)
        });
        c.get()
    }

    #[test]
    fn test_set_atom() {
        let a = a();
        assert_eq!(a.get(), 0, "we should get 0 as init value");
        a.set(8);
        assert_eq!(a.get(), 8, "We should get 8 as new value inserted with set")
    }

    #[test]
    fn test_inert_set() {
        let a_b_subtraction = a_b_subtraction();
        a().set(0);
        b().set(0);

        a().inert_set(165);
        assert_eq!(
            a_b_subtraction.get(),
            0,
            "We should get 0 for subtraction because inert setting"
        );
        assert_eq!(a().get(), 165, "We should get 165");
    }

    #[test]
    fn test_update() {
        a().update(|state| *state = 40);
        assert_eq!(a().get(), 40, "We should get 40 as value for a");
    }

    #[test]
    fn test_get_with() {
        a().set(3);
        b().set(5);

        a().get_with(|v| assert_eq!(v, &3, "We should get 3"));
        b().get_with(|v| assert_eq!(v, &5, "We should get 5"));
    }

    #[test]
    fn test_on_update() {
        let print = count_print_when_update();
        a().update(|v| *v = 32);
        a().set(2);
        a().set(25);
        a().set(1);
        println!("{:?}", print.get());
        assert_eq!(print.get(), 5)
    }

    #[test]
    fn test_delete() {
        let a = a();
        a.delete();
        assert_eq!(a.state_exists(), false, "The a state should not exist");
    }

    #[test]
    fn test_reset_to_default() {
        a().set(8);
        assert_eq!(a().get(), 8);
        a().reset_to_default();
        assert_eq!(a().get(), 0, "We should get 0 as it is init value");
    }

    #[test]
    fn test_observe_on_atom() {
        let a = a();
        let change = a.observe_change();
        println!("{:?}", change.0);
        println!("{:?}", change.1);
        assert_eq!(change.0.is_none(), true);
        assert_eq!(change.1, 0);
        a.set(1);
        let change2 = a.observe_change();
        println!("{:?}", change2.0);
        println!("{:?}", change2.1);
        assert_eq!(change2.0.unwrap(), 0);
        assert_eq!(change2.1, 1);
    }

    #[test]
    fn test_has_changed_on_atom() {
        let a = a();
        a.set(1);
        assert_eq!(a.has_changed(), true);
        a.set(1);
        assert_eq!(a.has_changed(), false);
    }

    #[test]
    fn test_on_changes_on_atom() {
        let a = a();
        let mut previous = 99;
        let mut current = 99;
        a.on_change(|p, c| {
            previous = *p;
            current = *c;
        });
        assert_eq!(previous, 0); //todo : should we expect None when init ?
        assert_eq!(current, 0);
        a.set(1);
        a.on_change(|p, c| {
            previous = *p;
            current = *c;
        });
        assert_eq!(previous, 0);
        assert_eq!(current, 1);
        a.set(1);
        a.on_change(|p, c| {
            previous = *p;
            current = *c;
        });
        assert_eq!(previous, 0);
        assert_eq!(current, 1);
        a.set(2);
        a.on_change(|p, c| {
            previous = *p;
            current = *c;
        });
        assert_eq!(previous, 1, "we should get 1");
        assert_eq!(current, 2, "we should get 2");
    }

    #[test]
    fn test_copy_atom() {
        let a = a();
        a.set(8);
        assert_eq!(a.get(), 8, "We should get 8 as value");

        let a_1 = a;
        assert_eq!(a_1.get(), 8, "We should get 8 as value on the copy as well")
    }
    #[test]
    fn test_clone_atom() {
        let a = a();
        a.set(8);
        assert_eq!(a.get(), 8, "We should get 8 as value");

        let a_1 = a.clone();
        assert_eq!(a_1.get(), 8, "We should get 8 as value on the copy as well")
    }
}
