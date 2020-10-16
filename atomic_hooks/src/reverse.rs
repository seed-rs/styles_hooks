use crate::{atom::Atom, *};

use store::RxFunc;

#[derive(Default, Clone)]
pub struct UndoStore {
    pub commands: Vec<Command>,
    pub cursor: usize,
}

#[derive(Clone)]
pub struct Command {
    do_cmd: RxFunc,
    reverse_cmd: RxFunc,
}

impl Command {
    pub fn new(do_cmd: RxFunc, undo_cmd: RxFunc) -> Self {
        Self {
            do_cmd,
            reverse_cmd: undo_cmd,
        }
    }
}

#[atom]
pub fn global_reverse_queue() -> Atom<UndoStore> {
    UndoStore::default()
}
pub trait GlobalUndo {
    fn travel_backwards(&self);
    fn travel_forwards(&self);
    fn len(&self) -> usize;
    fn travel_to_cursor(&self, cursor: usize);
}

impl GlobalUndo for Atom<UndoStore> {
    fn len(&self) -> usize {
        read_reactive_state_with_id::<UndoStore, _, _>(self.id, |q| q.commands.len())
    }

    fn travel_to_cursor(&self, cursor: usize) {
        assert!(cursor > 0);
        assert!(cursor < self.len());

        update_atom_state_with_id::<UndoStore, _>(self.id, |queue| {
            if cursor > queue.cursor {
                while cursor < queue.cursor {
                    if queue.cursor > 0 {
                        (queue.commands[queue.cursor - 1].reverse_cmd.func)();
                        queue.cursor -= 1;
                    }
                }
            } else if cursor < queue.cursor {
                while cursor < queue.cursor {
                    if queue.cursor < queue.commands.len() {
                        (queue.commands[queue.cursor].do_cmd.func)();
                        queue.cursor += 1;
                    }
                }
            }
        })
    }

    fn travel_backwards(&self) {
        update_atom_state_with_id::<UndoStore, _>(self.id, |queue| {
            if queue.cursor > 0 {
                (queue.commands[queue.cursor - 1].reverse_cmd.func)();
                queue.cursor -= 1;
            }
        });
    }

    fn travel_forwards(&self) {
        update_atom_state_with_id::<UndoStore, _>(self.id, |queue| {
            if queue.cursor < queue.commands.len() {
                (queue.commands[queue.cursor].do_cmd.func)();
                queue.cursor += 1;
            }
        });
    }
}
