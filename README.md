# dockertui

A terminal UI for Docker, written in Rust.

> **Status: early work in progress.** The service layer and the Docker adapter exist and
> work; the terminal UI does not exist yet. Right now the binary lists your containers to
> stdout. The architecture below is the point of the project so far — the interface is next.
> Do not expect a usable tool yet.

## What works today

- `ContainerService` / `SystemService` abstractions, bundled as `DockerService`
- `ContainerFilter` for choosing which containers to list and whether to pay
  for size information
- A working adapter over [bollard](https://crates.io/crates/bollard), including the
  conversion from Docker's API types into this project's own models
- A binary that connects to the local Docker daemon and prints container details
- A concrete `Error` enum describing situations a UI reacts to differently
  (daemon unreachable, permission denied, not found, conflict, timeout, …)
- 40 tests over the adapter: the type conversions, the error mapping, and the
  two decisions behind listing — what the daemon is asked for, and what is kept
  of its answer. None of them need a running daemon.

## What does not exist yet

- The terminal UI itself — no `ratatui`, no rendering, no key handling
- Any operation beyond listing: start, stop, logs, exec, inspect
- Any test that talks to a real daemon, so nothing checks that Docker still
  accepts the filter values this crate sends

## Architecture

```
dockertui/
├── dockerservice/          library crate — everything Docker, no UI
│   ├── traits/             the ports the frontend depends on
│   │   ├── container.rs    ContainerService
│   │   ├── system.rs       SystemService
│   │   └── mod.rs          DockerService: the two bundled together
│   ├── models.rs           own domain models (ContainerInfo, …)
│   ├── error.rs            backend-agnostic Error and Result
│   ├── types.rs            shared type aliases
│   └── bollard/            adapter, behind the "bollard-service" feature
│       ├── services.rs     DockerServiceImpl — implements the service traits
│       ├── conversions.rs  bollard API types → own models
│       └── errors.rs       bollard errors → own Error
└── frontend/cli/           binary crate — depends only on the trait
    └── main.rs
```

The frontend depends on the `DockerService` trait, never on `bollard`:

```rust
#[async_trait]
pub trait ContainerService: Send + Sync {
    async fn list_containers(&self, filter: ContainerFilter) -> Result<Vec<ContainerInfo>>;
}

pub trait SystemService: Send + Sync {
    fn version(&self) -> String;
}

/// A bundle, not a definition: no methods of its own, implemented for free
/// by anything providing every part.
pub trait DockerService: ContainerService + SystemService {}
```

One trait per resource, split along Docker's own API grouping. Code under test
bounds on the part it uses, so a double for the container list does not have to
stub out images and volumes; anything wanting the whole daemon takes
`DockerService`.

Three things follow from that split:

**The Docker client is replaceable.** `bollard` sits behind the `bollard-service` feature
flag and is an optional dependency. A different client — or a Podman adapter — is a second
implementation of the same trait, not a rewrite.

**The models are ours, not Docker's.** `conversions.rs` is the only place that knows what
Docker's API returns, and `errors.rs` the only place that knows what it fails with. The
`Error` enum names situations, not transports — nothing in it mentions bollard — so a
second adapter maps onto the same set and every screen above keeps working.

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
4. Integration tests against a real daemon, covering what the unit tests cannot
5. Images, volumes, networks

## Why this exists

I wanted a Docker TUI that fits how I work, and an excuse to write more Rust. The
architecture is deliberate: I would rather get the seams right while the project is small
than untangle them once there is an interface on top.
