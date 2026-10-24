# Kodanu

**A modular 3D game engine written in Rust.**

Kodanu is a personal game engine project focused on modular architecture, real-time rendering, and low-level control over engine systems. Built with Rust, `wgpu`, `winit`, and a custom Entity Component System (ECS), it provides the foundation for developing 3D games and interactive applications.

The engine is organized as a collection of specialized crates, allowing rendering, physics, input, mathematics, and other subsystems to evolve independently while working together through a unified application layer.

**[Repository](https://github.com/PsychoRosse/kodanu)**

## Overview

Kodanu is built around a plugin-based architecture. Engine functionality is divided into systems and modules that can be registered with an application, keeping the core runtime separate from optional features and game-specific logic.

The project aims to provide direct control over the engine's architecture rather than relying on a large, monolithic game engine.

### Key Features

- **Rendering:** A real-time rendering pipeline built on `wgpu`.
- **Custom ECS:** Entity and component management, queries, systems, resources, events, and plugins.
- **Application Runtime:** Window lifecycle, event processing, and system scheduling.
- **3D Graphics:** Perspective cameras, directional lighting, meshes, materials, depth testing, MSAA, and GPU instancing.
- **Physics:** Integration with `rapier3d` for rigid bodies, colliders, and physics simulation.
- **Input:** Keyboard and other input state integrated with the engine.
- **Time Management:** Frame timing and fixed- and variable-timestep scheduling.
- **Mathematics:** Vector, quaternion, and matrix types for 3D calculations.
- **Transforms:** Components for positioning, rotating, and scaling objects.
- **Asset Management:** Typed asset handles and centralized asset storage.
- **UI:** Retained-mode UI components, layout, styling, and draw lists.
- **Extensibility:** A plugin system for composing engine features and application logic.

## Architecture

Kodanu uses a modular workspace in which each crate is responsible for a distinct part of the engine.

### Application and Runtime

The application layer provides the entry point and manages the window event loop. It connects engine plugins and coordinates execution throughout the application's lifecycle.

### Entity Component System

The custom ECS is the foundation for organizing game state and behavior.

Its API supports:

- Queries for accessing entities and their components.
- Explicit read and write access to components.
- Query filters for selecting entities.
- Shared resources accessible to systems.
- Commands for managing entities.
- Plugin-based system registration.
- Staged system execution and events.

This approach keeps game logic separate from the engine's rendering and platform code.

### Rendering

The graphics subsystem uses `wgpu` to provide a cross-platform GPU abstraction.

Current rendering functionality includes perspective cameras, directional lights, mesh and material rendering, GPU instancing, depth testing, and multisampled rendering.

The graphics crate also contains UI rendering components and asset-related functionality.

### Physics

Physics integration is provided by `rapier3d`. The engine exposes physics functionality through its own subsystem, allowing game logic to work with rigid bodies, colliders, and spatial queries.

### Mathematics and Transforms

Kodanu includes a dedicated mathematics crate for vectors, quaternions, and matrices, alongside a transform subsystem for representing object placement and orientation in 3D space.

Keeping these primitives separate makes them reusable across rendering, physics, cameras, and gameplay systems.

## Workspace Structure

| Crate | Responsibility |
|---|---|
| `kodanu` | Main crate and public prelude |
| `kodanu_app` | Application entry point and event loop |
| `kodanu_ecs` | ECS world, systems, and plugins |
| `kodanu_graphics` | Rendering pipeline and UI components |
| `kodanu_camera` | Camera and projection components |
| `kodanu_input` | Input state and events |
| `kodanu_window` | Window integration |
| `kodanu_time` | Time management and scheduling |
| `kodanu_physics` | Physics integration with `rapier3d` |
| `kodanu_math` | Vectors, matrices, and quaternions |
| `kodanu_transform` | Transform components |
| `kodanu_ai` | Machine learning integration |
| `kodanu_macros` | Derive macros |
| `kodanu_plugins` | Development, demo, and free-camera plugins |

## Getting Started

### Requirements

- Rust toolchain and Cargo
- A graphics environment supported by `wgpu`
- Platform dependencies required by the underlying windowing and graphics libraries

### Run the Development Application

Clone the repository:

```bash
git clone https://github.com/PsychoRosse/kodanu.git
cd kodanu
```

Run the development application:

```bash
cargo run
```

The development binary initializes the standard development plugins and launches a demo scene with a free camera.

### Application Example

The engine can be initialized through its application API and plugin system:

```rust
use kodanu::prelude::*;

fn main() {
    App::default()
        .with_plugin(DevPlugins)
        .run_from(WindowAttributes::default());
}
```

`DevPlugins` registers the development stack, including window, input, time, rendering, and physics functionality.

## Design Goals

Kodanu is developed around several architectural goals:

- **Modularity:** Separate engine subsystems into focused crates.
- **Explicit composition:** Assemble application functionality through plugins.
- **Control:** Keep engine architecture and system behavior accessible to the developer.
- **Reusability:** Design core components and utilities for use across different engine systems.
- **Rust-first development:** Use Rust's type system and ownership model to structure engine code.

These are design goals rather than claims that every subsystem is already complete or optimized.

## Project Status

Kodanu is under active development. Its architecture and public APIs may evolve as new features are implemented and existing systems are refined.

The repository currently includes the core engine structure, a rendering subsystem, ECS infrastructure, physics integration, input and time systems, mathematics, transforms, and development plugins.

The project should be considered a work in progress rather than a finished, production-ready game engine.

## Contributing

Contributions, bug reports, and technical feedback are welcome. Before proposing substantial changes, review the existing crate structure and conventions to keep the engine modular and consistent.

## Author

Developed by **[PsychoRosse](https://github.com/PsychoRosse)**.

Kodanu is an independent project exploring the design and implementation of a modular game engine in Rust.
