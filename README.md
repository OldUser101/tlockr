# tlockr

![Build Status](https://github.com/OldUser101/tlockr/actions/workflows/default.yml/badge.svg)

tlockr is a highly customisable screen locker for Wayland-based compositors.

## What is it?

tlockr is a screen locker for Wayland compositors that support the `ext_session_lock_v1` protocol. 

Examples of compositors that support this protocol are:

- [sway](https://github.com/swaywm/sway)
- [Hyprland](https://github.com/hyprwm/Hyprland)
- [river](https://codeberg.org/river/river)
- [niri](https://github.com/YaLTeR/niri)
- [Wayfire](https://github.com/WayfireWM/wayfire)
- and others...

Note that not all of these compositors have actually been tested to work with tlockr.

Most other screen lockers (like swaylock) have limited customisability. tlockr addresses this, allowing users to select various QML themes to display.

This makes tlockr much more flexible that other screen lockers, allowing for dynamic images, animations, and widgets.

## How does it work?

tlockr uses a mix of Rust and C++ to achieve this.

The Rust code focuses on the Wayland backend, managing Wayland objects and buffers. It also handles authentication, and provides interfaces for the C++ frontend.

The C++ code focuses on the Qt frontend. It handles the initialization and use of various Qt objects. This involves rendering QML content into an offscreen buffer, which is then copied into a Wayland buffer.

## How do I use it?

You will need to build it from source, as there are no binary releases yet, see below for build instructions.

Running tlockr is currently quite simple, just run the compiled binary with the QML file to display as an argument. 

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

## Themes

At the moment, themes are just QML files with a specific structure.

The root of the content should not be a `Window`, since the content does not use a window. Check out some of the test files in the `test` directory for samples.

tlockr provides interfaces for connecting QML themes with the rest of the application:

- `tlockr.sendAuthSubmit`: submits authentication information, the only argument is the password as a string.
- Future interfaces planned...

When tlockr loads QML content, any errors are displayed in the log.

Since tlockr locks you out, if the QML content is invalid, you may not be able to unlock your session.
To avoid this, when developing themes, it is a good idea to run tlockr in a disposable compositor session and
pipe the logs back to your main session. That way, if you get locked out, you can safely kill the other
compositor.

## Contributing

Contributions to tlockr are always welcome!

The recommended workflow for this is:

- Fork this repository
- Create a new branch for your changes
- Make your changes
- Open a pull request

Any changes you make will need to compile without errors before you will be able to merge your pull request.

If you want to, you can make a draft pull request to discuss your changes as you are working on them.

## License

tlockr is licensed under the GNU GPL Version 3. See [LICENSE](https://github.com/OldUser101/tlockr?tab=GPL-3.0-1-ov-file) for further details.

Copyright © 2025, Nathan Gill
