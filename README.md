# 2D games in Rust workshop

RustFest Barcelona 2019



Modern Jazz Samba Kevin MacLeod (incompetech.com)
Licensed under Creative Commons: By Attribution 3.0 License
http://creativecommons.org/licenses/by/3.0/

version cut for brevity

## Setting the stage

Make sure you've got [version 1.38.0 or newer of Rust installed before starting](https://www.rust-lang.org/tools/install).

Next, we'll check our dependencies. Run the following in the terminal:

    $ cargo build
    
Should you see no errors, then you're all set!

## Project contents

Here's what you'll find when you clone the project:

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── resources
│   ├── Airstream.ttf
│   ├── Modern-Jazz-Samba-CUT.mp3
│   ├── Nick's Fonts License.txt
│   ├── claw_left.png
│   ├── claw_right.png
│   ├── crab.png
│   ├── sand.png
│   ├── snack.png
│   └── woopwoop.mp3
└── src
    ├── game
    │   ├── assets.rs
    │   ├── claw.rs
    │   ├── config.rs
    │   ├── crab.rs
    │   ├── mod.rs
    │   ├── player.rs
    │   ├── snacks.rs
    │   └── state.rs
    └── main.rs
```

We'll be spending most of our efforts in the contents of the `src` folder, but the rest are worth noting:

- `Cargo.lock and Cargo.toml` are present in a typical Rust project as a place to declare depdendencies and persist their version management.
- `resources` is where we'll store our games' assets, be it **fonts, images, audio**, and others.

## Dependencies

Next, let's take a look at the dependencies the project comes with:

```
[package]
name = "crab"
version = "0.1.0"
authors = ["lislis <mail@lislis.de>", "ramonh <hola@ramonh.dev>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
ggez = "0.5"
nalgebra = {version = "0.18", features = ["mint"] }
rand = "0.7"
```

Worthy of note here are three dependencies listed underneath the `dependencies` heading. Let's go through each of these:

- ggez ([docs](https://ggez.rs)): This is the library that we'll be using to do the heavy lifting in our game's code. This takes care of everything from drawing graphics to audio, as well as event handling (key presses and the like).
- nalgebra ([docs](https://www.nalgebra.org)): We'll be using this to do our vector algebra (It'll be super fun, promise!)
- rand ([docs](https://docs.rs/rand/0.7.2/rand/)): This we'll use for generating random numbers.
