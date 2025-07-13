# tlockr

![Build Status](https://github.com/OldUser101/tlockr/actions/workflows/default.yml/badge.svg)

tlockr is a highly customisable screen locker for Wayland-based compositors.

## What is it?

tlockr is a screen locker for Wayland compositors that support the `ext_session_lock_v1` protocol. 

Popular examples of compositors that support this protocol are:

- Sway
- Hyprland
- river
- niri
- Wayfire

Most other screen lockers (like swaylock) have limited customisability. tlockr addresses this, allowing users to select various QML themes to display.

This makes tlockr much more flexible that other screen lockers, allowing for dynamic images, animations, and widgets.

## How does it work?

tlockr uses a mix of Rust and C++ to achieve this.

The Rust code focuses on the Wayland backend, managing Wayland objects and buffers. It also handles authentication (when it is implemented), and provides interfaces for the C++ frontend.

The C++ code focuses on the Qt frontend. It handles the initialization and use of various Qt objects. This involves rendering QML content into an offscreen buffer, which is then copied into a Wayland buffer.

## How do I use it?

tlockr is not yet ready to be used a screen locker. Since authentication is not yet implemented, you will get locked out if you run it.

However, you can still build it if you feel like it.

## How do I build it?

To build tlockr, you will need to start by cloning this repository:

```sh
$ git clone https://github.com/OldUser101/tlockr.git
```

You can then proceed to the building step. You will need:

- CMake
- Cargo, with the latest version of Rust
- A C++ compiler
- Qt6 development libraries

```sh
$ cd tlockr
$ cargo build --release
```

The compiled binary can be found at `target/release/tlockr`.

⚠️ Currently, running the `tlockr` binary will completely lock you out, since authentication is not implemented yet. If you want to test it, it is recommended to run it in a disposable compositor session, which you can kill later.

## Contributing

Contributions to tlockr are always welcome!

The recommended workflow for this is:

- Fork this repository
- Create a new branch for your changes
- Make your changes
- Open a pull request

Any changes you make will need to compile without errors before you will be able to merge your pull request.

Since tlockr is in the early stages of development, all sorts of contributions are welcome. THis includes those that don't directly result from an issue, just make sure you detail exactly what changes you've made.

If you really want to, you can make a draft pull request to discuss your changes as you are working on them.

## License

tlockr is licensed under the GNU GPL Version 3. See [LICENSE](https://github.com/OldUser101/tlockr?tab=GPL-3.0-1-ov-file) for further details.

Copyright © 2025, Nathan Gill
