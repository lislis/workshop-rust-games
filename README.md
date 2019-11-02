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
│   └── snack.png
└── src
    ├── config.rs
    ├── game
    │   ├── assets.rs
    │   ├── claw.rs
    │   ├── crab.rs
    │   └── mod.rs
    └── main.rs
```

We'll be spending most of our efforts in the contents of the `src` folder, but the rest are worth noting:

- `Cargo.lock and Cargo.toml` are present in a typical Rust project as a place to declare depdendencies and persist their version management.
- `resources` is where we'll store our games' assets, be it **fonts, images, audio**, and others.
