# bevy_lints

This project provides my lints for bevy, using [Dylint](https://github.com/trailofbits/dylint). This is mainly for my other project, [Wisphaven](https://github.com/jim-works/Wisphaven). These lints will be specialized for what I care about, so you may find some annoying. You can always pick which lints to use.

Current lints:
- Require all spawned entities to be state scoped

# How to use

You do not need to clone this repository to use it. Dylint provides support for running lints from git repos, though you can also clone and run it from a path on your system if you prefer. See https://github.com/trailofbits/dylint#workspace-metadata for more detailed instructions (including how to use a subset of lints).

1. Install the necessary dependencies by running

`cargo install cargo-dylint dylint-link`

2. Add the following to your project's `Cargo.toml` file (this pulls in all lints, see the guide above to only use specific lints using `pattern`)

```toml

[workspace.metadata.dylint]
libraries = [
    { git = "https://github.com/jim-works/bevy_lints", branch="master" },
    # or if you have cloned locally
    # { path = "path/to/repo/folder" }
]
```

3. Now, you can run `cargo dylint --all` in your project to get the lints.

## Disabling lints

You can use something like `#[cfg_attr(dylint_lib = "bevy_lints", allow(state_scoped_entities))]` to disable a specific lint.
You may get a warning about an unknown library, which can be avoided by updating your Cargo.toml as described [here.](https://github.com/trailofbits/dylint?tab=readme-ov-file#rustcs-unexpected_cfg-lint)

If you don't care about receiving warnings about unknown lints, you can also use `#[allow(lint_name)]` in combination with `#[allow(unknown_lints)]`. If you go this path, you would probably want to use the global version `#![allow(unknown_lints)]` in your crate's `lib.rs` or `mod.rs`. Another approch is to add this to your `Cargo.toml`:

```toml

[lints.rust] #or [workspace.lints.rust]
unknown_lints = "allow"
```

## VSCode integration

See [the Dylint docs](https://github.com/trailofbits/dylint?tab=readme-ov-file#vs-code-integration).

# Related projects

The inspiration for this: https://github.com/MinerSebas/bevy_lint/