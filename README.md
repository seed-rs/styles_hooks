# Seed Styles and Hooks

- This repo has been created from these ones:
  - https://github.com/rebo/atomic_hooks
  - https://github.com/rebo/seed_style_preview
  - https://github.com/rebo/seed_hooks

 - The main purpose of the repo is to group libraries `atomic_hooks`, `seed_hooks` and `seed_styles` to make development faster and easier.

 - _Note:_ There are still active issues in the repos above and all corresponding crates currently deployed on [crates.io](https://crates.io/) are based on them.

 - You can find tutorials and documentation on [seed-style-hooks.netlify.app](https://seed-style-hooks.netlify.app/), however it's possible that some information will become obsolete.

 - Include chosen libraries in your Seed project this way:
    ```rust
    [dependencies]
    seed_styles = { git = "https://github.com/seed-rs/styles_hooks", package = "seed_styles", branch = "main" }
    seed_hooks = { git = "https://github.com/seed-rs/styles_hooks", package = "seed_hooks", branch = "main" }
    atomic_hooks = { git = "https://github.com/seed-rs/styles_hooks", package = "atomic_hooks", branch = "main" }
    ```

- All libraries use Seed's `master` branch.

- _Development:_ 
   - Please run at least `cargo build` from the root before `push` to make sure all libraries are still compatible. We'll setup CI with tests and linters later. 
   - Use `stable` and the latest Rust version (`$ rustup update`).
   - Squash commits and try to respect [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
