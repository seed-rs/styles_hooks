mod ev_handlers;

mod reactive_enhancements;
mod seed_bind;
mod update_el;
mod utils;
pub use ev_handlers::StateAccessEventHandlers;
pub use reactive_enhancements::ReactiveEnhancements;
pub use seed_bind::{InputBind, UpdateElLocal};
pub use update_el::{LocalUpdateEl2, StateAccessUpdateEl};
pub use utils::{
    after_render,
    after_render_once,
    get_html_element_by_id, //handle_unmount,
    request_animation_frame,
};

pub use atomic_hooks::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
