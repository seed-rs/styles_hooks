use crate::{
    reactive_state_access::{atom::Atom, reaction::Reaction, reversible_atom::ReversibleAtom},
    reverse::global_reverse_queue,
    store::{ReactiveContext, RxFunc, SlottedKey, StorageKey, Store},
};
use std::{cell::RefCell, hash::Hash, rc::Rc};

// use seed::{*,prelude};

thread_local! {
    pub static STORE: RefCell<Store> = RefCell::new(Store::new());
}

//
//  Constructs a T atom state accessor. T is stored keyed to the provided String
// id.  The accessor always references this id therefore can you can set/update/
// or get this T  from anywhere.
//
//   The passed closure is only used for the first initialisation of state.
//   Subsequent evaluations of this function just returns the accessor.
//   Only one type per context can be stored in this way.
//
//
// Typically this is created via the #[atom] attribute macro
//
pub fn atom<T: 'static, F: Fn() -> () + 'static>(id: StorageKey, data_fn: F) -> Atom<T> {
    // we do not need to re-initalize the atom if it already has been stored.
    if !reactive_state_exists_for_id::<T>(id) {
        let reaction = RxFunc {
            func: Rc::new(data_fn),
        };

        STORE.with(|store_refcell| {
            store_refcell
                .borrow_mut()
                .new_reaction(&id, reaction.clone());
        });

        (reaction.func.clone())();

        STORE.with(|store_refcell| {
            store_refcell.borrow_mut().add_atom(&id);
        })
    }
    Atom::new(id)
}

pub fn atom_reverse<T: 'static + Clone, F: Fn() -> () + 'static>(
    id: StorageKey,
    data_fn: F,
) -> ReversibleAtom<T> {
    // we do not need to re-initalize the atom if it already has been stored.
    if !reactive_state_exists_for_id::<T>(id) {
        let reaction = RxFunc {
            func: Rc::new(data_fn),
        };

        STORE.with(|store_refcell| {
            store_refcell
                .borrow_mut()
                .new_reaction(&id, reaction.clone());
        });

        (reaction.func.clone())();

        crate::reverse::global_reverse_queue().update(|u| {
            u.commands.push(crate::reverse::Command::new(
                reaction,
                RxFunc {
                    func: Rc::new(move || {
                        remove_reactive_state_with_id::<T>(id);
                    }),
                },
            ))
        });

        STORE.with(|store_refcell| {
            store_refcell.borrow_mut().add_atom(&id);
        })
    }
    ReversibleAtom::new(id)
}

//
//  Constructs a T reaction state accessor. T is stored keyed to the provided
// String id.  The accessor always references this id. Typically reaction values
// are auto  created based on changes to their dependencies which could be other
// reaction values or an  atom state.
//
//   The passed closure is run whenever a dependency of the reaction state has
// been updated.
//
//
// Typically this is created via the #[reaction] attribute macro
//
pub fn reaction<T: 'static, F: Fn() -> () + 'static>(id: StorageKey, data_fn: F) -> Reaction<T> {
    if !reactive_state_exists_for_id::<T>(id) {
        STORE.with(|store_refcell| {
            let key = store_refcell.borrow_mut().primary_slotmap.insert(id);

            store_refcell.borrow_mut().id_to_key_map.insert(id, key);
        });

        let reaction = RxFunc {
            func: Rc::new(data_fn),
        };

        STORE.with(|store_refcell| {
            store_refcell
                .borrow_mut()
                .new_reaction(&id, reaction.clone());
        });

        (reaction.func.clone())();
    }

    Reaction::<T>::new(id)
}

pub fn reaction_start_suspended<T: 'static, F: Fn() -> () + 'static>(
    id: StorageKey,
    data_fn: F,
) -> Reaction<T> {
    if !reactive_state_exists_for_id::<T>(id) {
        STORE.with(|store_refcell| {
            let key = store_refcell.borrow_mut().primary_slotmap.insert(id);

            store_refcell.borrow_mut().id_to_key_map.insert(id, key);
        });

        let reaction = RxFunc {
            func: Rc::new(data_fn),
        };

        STORE.with(|store_refcell| {
            store_refcell
                .borrow_mut()
                .new_reaction(&id, reaction.clone());
        });
    }

    Reaction::<T>::new(id)
}

pub fn unlink_dead_links(id: StorageKey) {
    let context = illicit::get::<RefCell<ReactiveContext>>().expect(
        "No #[reaction] context found, are you sure you are in one? I.e. does the current \
         function have a #[reaction] tag?",
    );
    if reactive_state_exists_for_id::<ReactiveContext>(id) {
        read_reactive_state_with_id::<ReactiveContext, _, ()>(id, |old_context| {
            let ids_to_remove = old_context
                .reactive_state_accessors
                .iter()
                .filter(|a_id| !context.borrow().reactive_state_accessors.contains(a_id));
            for id_to_remove in ids_to_remove {
                STORE.with(|store_refcell| {
                    store_refcell
                        .borrow_mut()
                        .remove_dependency(id_to_remove, &id);
                })
            }
        })
    } else {
        set_inert_atom_state_with_id::<ReactiveContext>(context.borrow().clone(), id)
    }
}

/// Sets the state of type T keyed to the given TopoId
pub fn set_inert_atom_state_with_id<T: 'static>(data: T, id: StorageKey) {
    STORE.with(|store_refcell| store_refcell.borrow_mut().set_state_with_id::<T>(data, &id))
}

/// Sets the state of type T keyed to the given TopoId
pub fn set_inert_atom_reversible_state_with_id<T: 'static + Clone>(data: T, id: StorageKey) {
    let new_data = data.clone();
    if let Some(previous_state) = clone_reactive_state_with_id::<T>(id) {
        global_reverse_queue().update(|u| {
            u.commands.truncate(u.cursor);

            u.commands.push(crate::reverse::Command::new(
                RxFunc::new(move || {
                    set_inert_atom_state_with_id::<T>(new_data.clone(), id);
                }),
                RxFunc::new(move || {
                    set_inert_atom_state_with_id::<T>(previous_state.clone(), id);
                }),
            ));
            u.cursor += 1;
        })
    }

    STORE.with(|store_refcell| store_refcell.borrow_mut().set_state_with_id::<T>(data, &id))
}

/// Sets the state of type T keyed to the given TopoId
pub fn set_atom_state_with_id<T: 'static>(data: T, id: StorageKey) {
    STORE.with(|store_refcell| store_refcell.borrow_mut().set_state_with_id::<T>(data, &id));

    execute_reaction_nodes(&id);
}

/// Sets the state of type T keyed to the given TopoId
pub fn set_atom_reversible_state_with_id<T: 'static + Clone>(data: T, id: StorageKey) {
    let new_data = data.clone();
    if let Some(previous_state) = clone_reactive_state_with_id::<T>(id) {
        global_reverse_queue().update(|u| {
            u.commands.truncate(u.cursor);

            u.commands.push(crate::reverse::Command::new(
                RxFunc::new(move || {
                    set_atom_state_with_id::<T>(new_data.clone(), id);
                }),
                RxFunc::new(move || {
                    set_inert_atom_state_with_id::<T>(previous_state.clone(), id);
                }),
            ));
            u.cursor += 1;
        })
    } else {
        global_reverse_queue().update(|u| {
            u.commands.truncate(u.cursor);

            u.commands.push(crate::reverse::Command::new(
                RxFunc::new(move || {
                    set_atom_state_with_id::<T>(new_data.clone(), id);
                }),
                RxFunc::new(move || {
                    remove_reactive_state_with_id::<T>(id);
                }),
            ));
            u.cursor += 1;
        })
    }

    STORE.with(|store_refcell| store_refcell.borrow_mut().set_state_with_id::<T>(data, &id));

    execute_reaction_nodes(&id);
}

pub fn reactive_state_exists_for_id<T: 'static>(id: StorageKey) -> bool {
    STORE.with(|store_refcell| store_refcell.borrow().state_exists_with_id::<T>(id))
}

/// Clones the state of type T keyed to the given TopoId
pub fn clone_reactive_state_with_id<T: 'static + Clone>(id: StorageKey) -> Option<T> {
    STORE.with(|store_refcell| {
        store_refcell
            .borrow_mut()
            .get_state_with_id::<T>(&id)
            .cloned()
    })
}

pub fn remove_reactive_state_with_id<T: 'static>(id: StorageKey) -> Option<T> {
    STORE.with(|store_refcell| store_refcell.borrow_mut().remove_state_with_id::<T>(&id))
}

pub fn remove_reactive_reversible_state_with_id<T: 'static + Clone>(id: StorageKey) -> Option<T> {
    if let Some(previous_state) = clone_reactive_state_with_id::<T>(id) {
        global_reverse_queue().update(|u| {
            u.commands.truncate(u.cursor);

            u.commands.push(crate::reverse::Command::new(
                RxFunc::new(move || {
                    remove_reactive_state_with_id::<T>(id);
                }),
                RxFunc::new(move || {
                    set_inert_atom_state_with_id::<T>(previous_state.clone(), id);
                }),
            ));
            u.cursor += 1;
        })
    } else {
        global_reverse_queue().update(|u| {
            u.cursor += 1;
        })
    }

    STORE.with(|store_refcell| store_refcell.borrow_mut().remove_state_with_id::<T>(&id))
}

#[derive(Clone)]
pub struct UndoVec<T>(pub Vec<T>);

pub fn execute_reaction_nodes(id: &StorageKey) {
    let ids_reactions = STORE.with(|refcell_store| {
        let mut borrow = refcell_store.borrow_mut();
        borrow.clone_dep_funcs_for_id(id)
    });

    for (key, reaction) in &ids_reactions {
        let cloned_reaction = reaction.clone();
        (cloned_reaction.func.clone())();
        execute_reaction_nodes(&key);
    }
}

pub fn update_atom_state_with_id<T: 'static, F: FnOnce(&mut T) -> ()>(id: StorageKey, func: F)
where
    T: 'static,
{
    let mut item = remove_reactive_state_with_id::<T>(id)
        .expect("You are trying to update a type state that doesnt exist in this context!");

    func(&mut item);

    set_inert_atom_state_with_id(item, id);

    //we need to get the associated data with this key
    execute_reaction_nodes(&id);
}

pub fn update_atom_reversible_state_with_id<T: 'static, F: FnOnce(&mut T) -> ()>(
    id: StorageKey,
    func: F,
) where
    T: Clone + 'static,
{
    let mut item = remove_reactive_state_with_id::<T>(id)
        .expect("You are trying to update a type state that doesnt exist in this context!");

    let previous_state = item.clone();
    func(&mut item);

    let new_item = item.clone();
    global_reverse_queue().update(|u| {
        u.commands.truncate(u.cursor);

        u.commands.push(crate::reverse::Command::new(
            RxFunc::new(move || {
                set_inert_atom_state_with_id::<T>(new_item.clone(), id);
            }),
            RxFunc::new(move || {
                set_inert_atom_state_with_id::<T>(previous_state.clone(), id);
            }),
        ));
        u.cursor += 1;
    });

    set_inert_atom_state_with_id(item, id);

    //we need to get the associated data with this key
    execute_reaction_nodes(&id);
}

pub fn read_reactive_state_with_id<T: 'static, F: FnOnce(&T) -> R, R>(
    id: StorageKey,
    func: F,
) -> R {
    let item = remove_reactive_state_with_id::<T>(id)
        .expect("You are trying to read a type state that doesnt exist in this context!");
    let read = func(&item);
    set_inert_atom_state_with_id(item, id);
    read
}

pub fn try_read_reactive_state_with_id<T: 'static, F: FnOnce(&T) -> R, R>(
    id: StorageKey,
    func: F,
) -> Option<R> {
    if let Some(item) = remove_reactive_state_with_id::<T>(id) {
        let read = func(&item);
        set_inert_atom_state_with_id(item, id);
        Some(read)
    } else {
        None
    }
}

pub fn return_key_for_type_and_insert_if_required<T: 'static + Clone + Eq + Hash>(
    value: T,
) -> StorageKey {
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};

    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash_id = hasher.finish();

    let key = StorageKey::SlottedKey(SlottedKey {
        location: hash_id,
        slot: 0,
    });

    STORE.with(|refcell_store| {
        refcell_store
            .borrow_mut()
            .return_key_for_type_and_insert_if_required(key, value.clone())
    })
}

// fn deep_collision_check_for_state_id<T: 'static>(id:StorageKey, item:T,
// stored_value:T) -> StorageKey {     if let Some() =
// remove_reactive_state_with_id::<Vec<(T,StorageKey)>(id) {

//     } else {
//         set_inert_atom_state_with_id(vec![], id);
//     }

// }
