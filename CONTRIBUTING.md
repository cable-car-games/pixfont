# Contributing

This is an early WIP. The API and formats are currently unstable.

I ([Razza]) will be frequently pushing to `main` while getting the MVP done before shifting to a PR-oriented approach.

[Razza]: https://github.com/itisrazza

If you want to contribute, [open an issue](https://github.com/cable-car-games/pixfont/issues/new) before spending time on a PR to discuss the changes.

## Repo layout

This repo contains multiple subprojects:

- [crates/pixfont](crates/pixfont): Reference implementation in Rust
- [crates/pixfont-studio](crates/pixfont-studio/): Cross-platform editor

I expect to add WebAssembly support and ports to other languages in the near future.

## Building

Currently, this is a fairly standard Rust project.

One gotcha to keep in mind is us pulling in a few external submodules.

```bash
git clone --recurse-submodules https://github.com/cable-car-games/pixfont.git
cargo build
cargo run -p pixfont-studio
```

If you already cloned without `--recurse-submodules` (I'm guilty of doing it too), you can run:

```bash
git submodule update --init
```

## Before you commit

Use [pre-commit] to run the a battery of style and tests before you commit.

```bash
pre-commit install  # install hooks
pre-commit run      # run the checks manually
```

[pre-commit]: https://pre-commit.com/

## Licensing of contributions

By submitting a contribution, you agree to license it under the relevant subproject's licence.

More details: see [LICENSE.md](./LICENSE.md).

### Source headers

Every new file needs SPDX headers.

```rust
// SPDX-License-Identifier: MIT OR Apache-2.0      (in pixfont)
// SPDX-FileCopyrightText: <year> <name>

// SPDX-License-Identifier: AGPL-3.0-or-later       (in pixfont-studio)
// SPDX-FileCopyrightText: <year> <name>
```

### Assets

Any new dependencies or bundled assets must be license-compatible and [annotated](./REUSE.toml) with the correct licence.

```toml
# RazzaSans (Nunito Sans) in PixFont Studio
[[annotations]]
path = ["crates/pixfont-studio/src/ui/font/RazzaSans/**/*"]
SPDX-License-Identifier = "OFL-1.1"
SPDX-FileCopyrightText = "Copyright 2016 The Nunito Sans Project Authors (https://github.com/Fonthausen/NunitoSans)"

# Bootstrap Icons (until we replace them with our own)
[[annotations]]
path = ["external/bootstrap-icons/**/*"]
SPDX-License-Identifier = "MIT"
SPDX-FileCopyrightText = "Copyright (c) 2019-2024 The Bootstrap Authors"
```

## Commits & PRs

I try to use a variant of [Conventional Commits]. Try keep commits and PRs focused on a task.

```
fix(pixfont/glyph): use u32::checked_sub for calculating offset
feat(studio/topbar): have export be a dropdown menu instead of a dialog
```

For the scope, follow the pattern in Git history. I will come up with a table later.

[Conventional Commits]: https://www.conventionalcommits.org/en/v1.0.0/

## Code style & patterns

TBD

## Bugs & features

TBD

## Code of Conduct

This is meant to be a chill fun project. Annoying people goes against that goal.

Pushing back on sloppy code is fine, attacking people personally isn't.

A proper CoC will likely be added if the project grows.
