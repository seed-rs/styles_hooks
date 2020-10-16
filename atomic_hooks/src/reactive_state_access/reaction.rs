use crate::{
    clone_reactive_state_with_id, reactive_state_exists_for_id, read_reactive_state_with_id,
    remove_reactive_state_with_id, store::StorageKey, Observable, RxFunc,
};

use crate::reactive_state_access::{
    state_access::CloneState, CloneReactiveState, ObserveChangeReactiveState,
};
use std::marker::PhantomData;

/// A reaction is an observable state combined from one or multiple atom state.
/// Literally you can write code that is function of atom state value to produce
/// a new value which you can observe. The new value will get automatically
/// updated as long as the update on the atom is not **inert**.  
///
/// ```
/// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
/// #[atom]
/// fn a() -> Atom<i32> {
///     0
/// }
///
/// #[atom]
/// fn b() -> Atom<i32> {
///     0
/// }
///
/// #[reaction]
/// fn a_b_subtraction() -> Reaction<i32> {
///     let a = a().observe();
///     let b = b().observe();
///     (a - b)
/// }
/// // we have the state
/// // of a - b and we can get it when never we want.
/// // the value should always be automatically updated
/// fn test_reaction() {
///     let a_b_subtraction = a_b_subtraction();
///
///     a().set(0);
///     b().set(0);
///     a().update(|state| *state = 40);
///     assert_eq!(a().get(), 40, "We should get 40 as value for a");
///     assert_eq!(
///         a_b_subtraction.get(),
///         40,
///         "We should get 40 for subtraction because setting"
///     );
///
///     b().set(10);
///     assert_eq!(
///         a_b_subtraction.get(),
///         30,
///         "We should get 40 for subtraction because setting"
///     );
///     b().inert_set(0);
///     assert_eq!(
///         a_b_subtraction.get(),
///         30,
///         "We should get 30 for subtraction because setting inert"
///     );
///     b().set(20);
///     assert_eq!(
///         a_b_subtraction.get(),
///         20,
///         "We should get 20 for subtraction because setting"
///     );
/// }
/// ```
pub struct Reaction<T> {
    pub id: StorageKey,

    pub _phantom_data_stored_type: PhantomData<T>,
}

impl<T> std::fmt::Debug for Reaction<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}
//

//
impl<T> Clone for Reaction<T> {
    fn clone(&self) -> Reaction<T> {
        Reaction::<T> {
            id: self.id,

            _phantom_data_stored_type: PhantomData::<T>,
        }
    }
}

impl<T> Copy for Reaction<T> {}

impl<T> Reaction<T>
where
    T: 'static,
{
    /// Create a new reaction
    pub fn new(id: StorageKey) -> Reaction<T> {
        Reaction {
            id,

            _phantom_data_stored_type: PhantomData,
        }
    }
    /// Remove the reaction from the global state
    /// ```
    /// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[atom]
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[reaction]
    /// fn a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     (a - b)
    /// }
    /// fn test_delete() {
    ///     let a_b_subtraction = a_b_subtraction();
    ///     a_b_subtraction.remove();
    ///
    ///     assert_eq!(
    ///         a_b_subtraction.state_exists(),
    ///         false,
    ///         "The state has been removed, so it should not exist"
    ///     );
    /// }
    /// ```
    pub fn remove(self) -> Option<T> {
        remove_reactive_state_with_id(self.id)
    }

    ///
    /// ```
    /// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[atom]
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[reaction]
    /// fn a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     (a - b)
    /// }
    /// fn test_delete() {
    ///     let a_b_subtraction = a_b_subtraction();
    ///     a_b_subtraction.delete();
    ///
    ///     assert_eq!(
    ///         a_b_subtraction.state_exists(),
    ///         false,
    ///         "The state has been removed, so it should not exist"
    ///     );
    /// }
    /// ```
    pub fn delete(self) {
        self.remove();
    }

    /// This method force the update of the new combined value
    /// ## Question
    /// - I thought the new value was updated automatically, isn't ?
    /// - When & why to use this method ?
    pub fn force_trigger(&self) {
        (clone_reactive_state_with_id::<RxFunc>(self.id)
            .unwrap()
            .func)();
    }
    /// Check if the state_exist
    /// ```
    /// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[atom]
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[reaction]
    /// fn a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     (a - b)
    /// }
    /// fn test_state_exist() {
    ///     let a_b_subtraction = a_b_subtraction();
    ///
    ///     assert_eq!(
    ///         a_b_subtraction.state_exists(),
    ///         true,
    ///         "The state should exist"
    ///     );
    ///     a_b_subtraction.delete();
    ///
    ///     assert_eq!(
    ///         a_b_subtraction.state_exists(),
    ///         false,
    ///         "The state has been removed, so it should not exist"
    ///     );
    /// }
    /// ```
    pub fn state_exists(self) -> bool {
        reactive_state_exists_for_id::<T>(self.id)
    }
    /// Let you get the value as a reference from a closure.
    /// ```
    /// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[atom]
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[reaction]
    /// fn a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     (a - b)
    /// }
    ///
    /// fn test_get_with() {
    ///     let a_b_subtraction = a_b_subtraction();
    ///     a_b_subtraction.get_with(|v| assert_eq!(v, &0, "We should get 0"));
    ///     a().set(10);
    ///     a_b_subtraction.get_with(|v| assert_eq!(v, &10, "We should get 10"));
    /// }
    /// ```
    pub fn get_with<F: FnOnce(&T) -> R, R>(&self, func: F) -> R {
        read_reactive_state_with_id(self.id, func)
    }
    /// Triggers the passed function when the atom is updated
    /// This method needs to be use insided a function body that has the
    /// attributes **[reaction]**.
    /// ```
    /// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[atom]
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    /// #[atom]
    /// fn c() -> Atom<i32> {
    ///     0
    /// }
    /// #[reaction]
    /// fn a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     (a - b)
    /// }
    /// #[reaction]
    /// fn count_subtraction_when_update() -> Reaction<i32> {
    ///     let c = c();
    ///     let update = a_b_subtraction().on_update(|| {
    ///         println!("UPDATE !!!");
    ///         c.update(|mut v| *v = *v + 1)
    ///     });
    ///     c.get()
    /// }
    /// fn test_on_update() {
    ///     let count = count_subtraction_when_update();
    ///     a().update(|v| *v = 32);
    ///     a().set(2);
    ///     a().set(25);
    ///     a().set(1);
    ///     assert_eq!(count.get(), 5);
    /// }
    /// ```
    #[topo::nested]
    pub fn on_update<F: FnOnce() -> R, R>(&self, func: F) -> Option<R> {
        let first_call_accessor = crate::hooks_state_functions::use_state(|| true);
        let mut recalc = false;

        self.observe_with(|_| {
            if first_call_accessor.get() {
                first_call_accessor.set(false)
            } else {
                recalc = true
            }
        });
        if recalc {
            Some(func())
        } else {
            None
        }
    }

    /// This method give us the possibility to know if a reaction has been
    /// updated.
    /// ```
    /// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[atom]
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    /// #[reaction]
    /// fn a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     (a - b)
    /// }
    ///
    /// #[reaction]
    /// fn count_subtraction_when_update() -> Reaction<i32> {
    ///     let c = c();
    ///     let update = a_b_subtraction().on_update(|| {
    ///         println!("UPDATE !!!");
    ///         c.update(|mut v| *v = *v + 1)
    ///     });
    ///     c.get()
    /// }
    /// fn test_has_updated() {
    ///     let count = count_subtraction_when_update();
    ///     a().update(|v| *v = 32);
    ///     assert_eq!(count.has_updated(), true);
    ///     a().set(32);
    ///     assert_eq!(
    ///         count.has_updated(),
    ///         false,
    ///         "No update should have been done since we updated a to 32 which is the same value as \
    ///              before"
    ///     );
    ///     a().set(25);
    ///     assert_eq!(count.has_updated(), true);
    ///     a().set(1);
    ///     assert_eq!(count.has_updated(), true);
    /// }
    /// ```
    #[topo::nested]
    pub fn has_updated(&self) -> bool {
        let first_call_accessor = crate::hooks_state_functions::use_state(|| true);
        let mut recalc = false;

        self.observe_with(|_| {
            if first_call_accessor.get() {
                first_call_accessor.set(false)
            } else {
                recalc = true
            }
        });
        recalc
    }
}

impl<T> Observable<T> for Reaction<T>
where
    T: 'static,
{
    fn id(&self) -> StorageKey {
        self.id
    }
}
impl<T> ObserveChangeReactiveState<T> for Reaction<T>
where
    T: Clone + 'static + PartialEq,
{
    /// Let you get the last changes on a reaction.
    ///
    /// ## Todo
    ///
    /// - the unit test is failing so I guess we need to investigate the bug
    /// ```
    /// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[atom]
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[reaction]
    /// fn a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     (a - b)
    /// }
    /// #[test]
    /// fn test_observe_changes_on_reaction() {
    ///     let a_b_subtraction = a_b_subtraction();
    ///     let changes = a_b_subtraction.observe_change();
    ///     assert_eq!(changes.0.is_none(), true);
    ///     assert_eq!(changes.1, 0);
    ///
    ///     a().set(2);
    ///     let changes = a_b_subtraction.observe_change();
    ///     assert_eq!(changes.0.unwrap(), 1);
    ///     assert_eq!(changes.1, 2);
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
    /// Let you know if changes has been made.
    ///
    /// ## Todo
    /// - the unit test is failing so I guess we need to investigate the bug
    /// ```
    /// use atomic_hooks::{atom::Atom, reaction::Reaction, Observable};
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[atom]
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[reaction]
    /// fn a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     (a - b)
    /// }
    /// #[test]
    /// fn test_has_changes_on_reaction() {
    ///     let a_b_subtraction = a_b_subtraction();
    ///
    ///     a().set(2);
    ///     let changes_happened = a_b_subtraction.has_changed();
    ///     assert_eq!(changes_happened, true);
    ///
    ///     a().set(3);
    ///     let changes_happened = a_b_subtraction.has_changed();
    ///     assert_eq!(changes_happened, true);
    ///
    ///     a().set(3);
    ///     let changes_happened = a_b_subtraction.has_changed();
    ///     assert_eq!(changes_happened, false);
    /// }
    /// ```
    #[topo::nested]
    fn has_changed(&self) -> bool {
        let previous_value_access = crate::hooks_state_functions::use_state(|| self.get());
        previous_value_access
            .get_with(|previous_value| self.observe_with(|new_value| new_value != previous_value))
    }

    /// Let you apply a function on previous and current value from changes.
    ///
    /// ## Todo
    ///
    /// - the unit test is failing so I guess we need to investigate the bug
    /// ```
    /// use atomic_hooks::atom::Atom;
    /// use atomic_hooks::reaction::Reaction;
    /// use atomic_hooks::Observable;
    /// #[atom]
    /// fn a() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[atom]
    /// fn b() -> Atom<i32> {
    ///     0
    /// }
    ///
    /// #[reaction]
    /// fn a_b_subtraction() -> Reaction<i32> {
    ///     let a = a().observe();
    ///     let b = b().observe();
    ///     (a - b)
    /// }
    /// #[test]
    /// fn test_on_changes_on_reaction() {
    ///     let a_b_subtraction = a_b_subtraction();
    ///     let mut previous = 99;
    ///     let mut current = 99;
    ///     a_b_subtraction.on_change(|p, c| {
    ///         previous = *p;
    ///         current = *c;
    ///     });
    ///     assert_eq!(previous, 0);
    ///     assert_eq!(current, 0);
    ///     a().set(1);
    ///     a_b_subtraction.on_change(|p, c| {
    ///         previous = *p;
    ///         current = *c;
    ///     });
    ///     assert_eq!(previous, 0);
    ///     assert_eq!(current, 1);
    ///     a().set(2);
    ///     a_b_subtraction.on_change(|p, c| {
    ///         previous = *p;
    ///         current = *c;
    ///     });
    ///     assert_eq!(previous, 1);
    ///     assert_eq!(current, 2);
    /// ```
    #[topo::nested]
    fn on_change<F: FnOnce(&T, &T) -> R, R>(&self, func: F) -> R {
        let previous_value_access = crate::hooks_state_functions::use_state(|| self.get());
        previous_value_access.get_with(|previous_value| {
            self.observe_with(|new_value| func(previous_value, new_value))
        })
    }
}
impl<T> CloneReactiveState<T> for Reaction<T>
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        reactive_state_access::{atom::Atom, reaction::Reaction, reversible_atom::ReversibleAtom},
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

    #[reaction]
    fn a_b_reversible_subtraction() -> Reaction<i32> {
        let a = a_reversible().observe();
        let b = b_reversible().observe();
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

    #[atom(reversible)]
    fn a_reversible() -> ReversibleAtom<i32> {
        0
    }

    #[atom(reversible)]
    fn b_reversible() -> ReversibleAtom<i32> {
        0
    }
    #[test]
    fn test_on_changes_on_reaction() {
        let a_b_subtraction = a_b_subtraction();
        let mut previous = 99;
        let mut current = 99;
        a_b_subtraction.on_change(|p, c| {
            previous = *p;
            current = *c;
        });
        assert_eq!(previous, 0); //todo : should we expect None when init ?
        assert_eq!(current, 0);
        a().set(1);
        a_b_subtraction.on_change(|p, c| {
            previous = *p;
            current = *c;
        });
        assert_eq!(previous, 0); //todo : should we expect None when init ?
        assert_eq!(current, 1);
        a().set(2);
        a_b_subtraction.on_change(|p, c| {
            previous = *p;
            current = *c;
        });
        assert_eq!(previous, 1); //todo : should we expect None when init ?
        assert_eq!(current, 2);
    }
    #[test]
    fn test_has_changes_on_reaction() {
        let a_b_subtraction = a_b_subtraction();

        a().set(2);
        let changes_happened = a_b_subtraction.has_changed();
        assert_eq!(changes_happened, true);

        a().set(3);
        let changes_happened = a_b_subtraction.has_changed();
        assert_eq!(changes_happened, true);

        a().set(3);
        let changes_happened = a_b_subtraction.has_changed();
        assert_eq!(changes_happened, false);
    }
    #[test]
    fn test_observe_changes_on_reaction() {
        let a_b_subtraction = a_b_subtraction();
        let changes = a_b_subtraction.observe_change();
        assert_eq!(changes.0.is_none(), true);
        assert_eq!(changes.1, 0);

        a().set(2);
        let changes = a_b_subtraction.observe_change();
        assert_eq!(changes.0.unwrap(), 1);
        assert_eq!(changes.1, 2);
    }

    #[test]
    fn test_get_with() {
        let a_b_reversible_subtraction = a_b_reversible_subtraction();
        a_reversible().set(3);
        b_reversible().set(5);

        a_reversible().get_with(|v| assert_eq!(v, &3, "We should get 3"));
        b_reversible().get_with(|v| assert_eq!(v, &5, "We should get 5"));
        a_b_reversible_subtraction.get_with(|v| assert_eq!(v, &-2, "We should get -2"));
    }

    #[test]
    fn test_on_update_reaction() {
        let count = count_subtraction_when_update();
        println!("subtraction value -> {:?}", a_b_subtraction().get());
        a().update(|v| *v = 32);
        println!("subtraction value -> {:?}", a_b_subtraction().get());
        a().set(2);
        println!("subtraction value -> {:?}", a_b_subtraction().get());
        a().set(25);
        println!("subtraction value -> {:?}", a_b_subtraction().get());
        a().set(1);
        println!("subtraction value -> {:?}", a_b_subtraction().get());

        println!("number of updates-> {:?}", count.get());

        assert_eq!(count.get(), 5, "We should get 5 update counted");
    }

    #[test]
    fn test_inert_set() {
        a_reversible().inert_set(155);
        assert_eq!(a_reversible().get(), 155, "We should get 155");

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
    fn test_delete() {
        let subtraction_reaction = a_b_subtraction();
        subtraction_reaction.delete();

        assert_eq!(
            subtraction_reaction.state_exists(),
            false,
            "The state  a_b_subtraction should not exist"
        );
    }

    #[test]
    fn test_reaction() {
        let a_b_subtraction = a_b_subtraction();
        a().set(0);
        b().set(0);
        a().update(|state| *state = 40);
        assert_eq!(a().get(), 40, "We should get 40 as value for a");
        assert_eq!(
            a_b_subtraction.get(),
            40,
            "We should get 40 for subtraction because setting"
        );

        b().set(10);
        assert_eq!(
            a_b_subtraction.get(),
            30,
            "We should get 40 for subtraction because setting"
        );
        b().inert_set(0);
        assert_eq!(
            a_b_subtraction.get(),
            30,
            "We should get 30 for subtraction because setting inert"
        );
        b().set(20);
        assert_eq!(
            a_b_subtraction.get(),
            20,
            "We should get 20 for subtraction because setting"
        );
    }

    #[test]
    fn test_reversible_reaction() {
        let a_b_reversible_subtraction = a_b_reversible_subtraction();
        a_reversible().set(0);
        b_reversible().set(0);
        a_reversible().update(|state| *state = 40);
        assert_eq!(a_reversible().get(), 40, "We should get 40 as value for a");
        assert_eq!(
            a_b_reversible_subtraction.get(),
            40,
            "We should get 40 for subtraction because setting"
        );

        global_reverse_queue().travel_backwards();

        assert_eq!(
            a_reversible().get(),
            0,
            "We should get 0 on a because back in time"
        );

        assert_eq!(
            a_b_reversible_subtraction.get(),
            0,
            "We should get 0 as result for subtraction because back in time"
        );

        b_reversible().inert_set(0);
        assert_eq!(
            a_b_reversible_subtraction.get(),
            0,
            "We should get 0 for subtraction because setting inert"
        );
        a_reversible().set(20);
        assert_eq!(
            a_b_reversible_subtraction.get(),
            20,
            "We should get 20 for subtraction because setting"
        );
    }
}
