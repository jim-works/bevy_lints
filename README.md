# bevy_lints

This project provides my lints for bevy, using [Dylint](https://github.com/trailofbits/dylint). This is mainly for my other project, [Wisphaven](https://github.com/jim-works/Wisphaven)

# How to use

You do not need to clone this repository to use it. Dylint provides support for running lints from git repos, though you can also clone and run it from a path on your system. See https://github.com/trailofbits/dylint#workspace-metadata for instructions.

1. Install the necessary dependencies by running

`cargo install cargo-dylint dylint-link`

2. Add the following to your project's `Cargo.toml` file (this pulls in all lints, see the guide above to only use specific lints using `pattern`)

```toml

[workspace.metadata.dylint]
libraries = [
    { git = "https://github.com/jim-works/bevy_lints", branch="main" },
]
```

3. Now, you can run `cargo dylint --all` in your project to get the lints.

# Related projects

The inspiration for this: https://github.com/MinerSebas/bevy_lint/