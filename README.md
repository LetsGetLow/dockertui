# dockertui

A terminal UI for Docker, written in Rust.

> **Status: early work in progress.** The service layer and the Docker adapter exist and
> work; the terminal UI does not exist yet. Right now the binary lists your containers to
> stdout. The architecture below is the point of the project so far — the interface is next.
> Do not expect a usable tool yet.

## What works today

- `DockerService` abstraction with `version()` and `list_containers()`
- A working adapter over [bollard](https://crates.io/crates/bollard), including the
  conversion from Docker's API types into this project's own models
- A binary that connects to the local Docker daemon and prints container details

## What does not exist yet

- The terminal UI itself — no `ratatui`, no rendering, no key handling
- Any operation beyond listing: start, stop, logs, exec, inspect
- A concrete error type; the crate currently returns `Box<dyn Error>`
- Tests

## Architecture

```
dockertui/
├── dockerservice/          library crate — everything Docker, no UI
│   ├── traits.rs           DockerService: the port the frontend depends on
│   ├── models.rs           own domain models (ContainerInfo, …)
│   ├── types.rs            shared type aliases and Result
│   └── bollard/            adapter, behind the "bollard-service" feature
│       ├── services.rs     DockerServiceImpl — implements DockerService
│       └── conversions.rs  bollard API types → own models
└── frontend/cli/           binary crate — depends only on the trait
    └── main.rs
```

The frontend depends on the `DockerService` trait, never on `bollard`:

```rust
#[async_trait]
pub trait DockerService {
    fn version(&self) -> String;
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>>;
}
```

Three things follow from that split:

**The Docker client is replaceable.** `bollard` sits behind the `bollard-service` feature
flag and is an optional dependency. A different client — or a Podman adapter — is a second
implementation of the same trait, not a rewrite.

**The models are ours, not Docker's.** `conversions.rs` is the only place that knows what
Docker's API returns. Everything above it works with `ContainerInfo`, so an API change is
contained in one file.

**The UI is one frontend among several.** `frontend/cli` is a separate crate. The TUI will
be another one, and both can sit on the same service without duplicating Docker logic.

## Running it

Requires a reachable Docker daemon.

```sh
cargo run -p dockertui
```

## Roadmap

1. `ratatui`-based interface: container list, selection, live status
2. Container actions — start, stop, restart, remove
3. Log streaming
4. A concrete error enum instead of `Box<dyn Error>`
5. Tests on the conversion layer, which is testable without a running daemon
6. Images, volumes, networks

## Why this exists

I wanted a Docker TUI that fits how I work, and an excuse to write more Rust. The
architecture is deliberate: I would rather get the seams right while the project is small
than untangle them once there is an interface on top.
