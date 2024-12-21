# Corvid Debug

Corvid Debug is a tool that helps with debugging [Bevy Engine](https://bevyengine.org/) software, made in Rust.

This project was split from [Starlight Engine](https://github.com/Mad-Star-Studio/Starlight), a voxel game engine that is currently in development which uses Bevy Engine.

## Features

Eventual planned features include:

- [ ] **Profiling**: Measure the performance of your game and find bottlenecks.
  - [ ] **Frame Time**: Measure the time it takes to render a frame.
  - [ ] **System Time**: Measure the time it takes to run a system, without having to manually add recording code.
  - [ ] **Entity Count**: Measure the number of entities in your game.
  - [ ] **Component Count**: Measure the number of components in your game.
  - [ ] **History**: Keep a history of the performance of your game.
    - [ ] **Real-time Full-History Display**: Display, as a graph, the history of things taking up performance, without fancy presentation.
    - [ ] **Real-time Simplified Display**: Display, as a graph, the history of things taking up performance, at a glance.
- [ ] **Inspector**: Measure the state of your game and find bugs.
  - [ ] **Entity Inspector**: Inspect the state of an entity.
  - [ ] **Component Inspector**: Inspect the state of a component.
  - [ ] **Resource Inspector**: Inspect the state of a resource.
  - [ ] **System Inspector**: Inspect the state of a system.
  - [ ] **Event Inspector**: Inspect the state of an event.
- [ ] **Console**: Get information about your game.
  - [ ] **Command Engine**: Run commands to get information about your game, or alter its state.
  - [ ] **Logging**: Get logs from your game.
- [ ] **Render Debugging**: Debug the rendering of your game.
  - [ ] **Wireframe Mode**: Render your game in wireframe mode.
  - [ ] **Bounding Box Mode**: Render bounding boxes around entities.
  - [ ] **Fly Mode**: Fly around your game, without any systems bothering you.
  - [ ] **Freeze Mode**: Freeze your game, so you can inspect it.
  - [ ] **GPU Asset Inspector**: Inspect the state of a GPU asset.
    - [ ] **Texture Inspector**: Inspect the state of a texture.
    - [ ] **Mesh Inspector**: Inspect the state of a mesh.
    - [ ] **Material Inspector**: Inspect the state of a material.
    - [ ] **Shader Inspector**: Inspect the state of a shader.

## Getting Started

### Integration

Please come back later, as the project is still in development.
It should be able to be at least be used very soon, however.

### Developing

To develop the project, you will need to have Rust installed on your machine.
To do this, you need to either visit the [Rust website](https://www.rust-lang.org/) for instructions or use [Rustup](https://rustup.rs/).
Alternatively, we provide a [flake.nix](./flake.nix) file that can be used with [Nix and NixOS](https://nixos.org/) to install Rust and other dependencies automatically, with everything else you need to develop the project.

If you don't otherwise have a preferred editor, we recommend using [Visual Studio Code](https://code.visualstudio.com/) with the [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.

## Corvid?

Corvids are a family of birds that include crows, ravens, and magpies.
They are known for their intelligence and problem-solving skills.

This was inspired by Bevy Engine, which is named after a group of birds.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
