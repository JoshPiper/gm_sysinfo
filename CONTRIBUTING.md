# Contributing to gm_sysinfo

Thanks for taking a look. This is a small project, so the process is deliberately light — but there's one rule that isn't optional.

## Commit messages must be Conventional Commits

This is not a style preference. [release-plz](https://release-plz.dev) reads your commit messages to decide whether a release is needed, what the next version number is, and what goes in the changelog. A commit that doesn't follow the format doesn't get you a broken release — it gets you **no release at all**, silently.

Format: `<type>[optional scope]: <description>`

The types that matter to the release process:

| Type | Effect |
|---|---|
| `fix:` | Patch release (`x.y.Z`) |
| `feat:` | Minor release (`x.Y.0`) |
| `feat!:` or a `BREAKING CHANGE:` footer | Major release (`X.0.0`) |
| `chore:`, `build:`, `ci:`, `docs:`, `test:`, `refactor:`, `style:` | No release triggered on their own |

Examples:

```
fix: correct the get_host_name error message

feat: expose sysinfo.get_build_info()

feat!: return memory and swap in bytes instead of KiB

BREAKING CHANGE: sysinfo.get_memory() and sysinfo.get_swap() now
return bytes. Previously they returned KiB.
```

If a PR mixes several logical changes, prefer several small commits with correct types over one commit with the "biggest" type — release-plz aggregates all of them for the changelog, and smaller commits review better too.

## What happens after you open a PR

- `ci.yml` runs `cargo fmt --check`, `cargo clippy -D warnings` (both realms), the full 10-target build matrix, and [GLuaTest](https://github.com/CFC-Servers/GLuaTest) against real Garry's Mod server instances. All of it needs to pass before merge.
- You don't need to bump the version, edit `CHANGELOG.md`, or create a tag. Once your PR merges to `main`, release-plz opens (or updates) a standing release PR by itself, from your commit messages. A maintainer merges that when it's time to ship — see the root README's semantics/release notes for why that's a deliberate gate, not an oversight.

## Local setup

The pinned nightly toolchain in [`rust-toolchain.toml`](rust-toolchain.toml) is picked up automatically by `rustup` — you shouldn't need to install anything by hand beyond `rustup` itself.

```bash
cargo build --release                    # server realm
cargo build --release --features gmcl    # client realm
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --features gmcl -- -D warnings
```

Run both clippy invocations — the two realms compile different code paths (see the `gmcl` feature gate in `src/lib.rs`), and CI lints both.

Cross-compiling to a specific release target needs that target installed:

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

32-bit Linux targets additionally need a multilib GCC (`sudo apt-get install gcc-multilib` on Debian/Ubuntu).

### Running the Lua tests locally

CI runs [GLuaTest](https://github.com/CFC-Servers/GLuaTest) against the built server module automatically on every PR (`.github/workflows/ci.yml`). Running it locally requires Docker; see GLuaTest's own documentation for a local invocation. Test specs live in `lua/tests/sysinfo/`.

## Code style

- No comments explaining *what* the code does — names should do that. A comment earns its place only when it captures a non-obvious *why*: a hidden constraint, a workaround, a decision that would otherwise look like a mistake on review.
- Keep the `unsafe` surface exactly as small as it already is. If you're adding a new Lua-exported function, follow the existing `#[lua_function]` pattern in `src/lib.rs` rather than introducing a new calling convention.
- Prefer extending the existing macros (`err!`, `export_lua_function!`, `set_field!`) over hand-rolling a one-off when a new getter fits the same shape as its neighbours.
- Run `cargo fmt` before pushing — CI enforces it and won't auto-fix it for you.

## Reporting a security issue

Don't open a public issue for that — see [SECURITY.md](SECURITY.md).
