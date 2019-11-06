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

## Overview

- `main.rs`
- Explain game module structure
- `state.rs` declarations
- `mod.rs` impl of State EventHandler
- Go into EventHandler (`update`, `draw`, etc.)
- `config.rs` explanation
- `assets.rs` explanation

## Getting started

To run the game, we'll be using Cargo:

    $ cargo run

Once the game has compiled and started running, you should see a blank window. Believe it or not, that's what we want! It means the game is running and displaying successfully.

Wondering where the window dimensions are determined? Take another look at `main.rs`:
```
    let (ref mut ctx, ref mut event_loop) =
        ContextBuilder::new("crab", "lislis & ramonh")
        .window_setup(conf::WindowSetup::default().title("Crab"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_W, SCREEN_H))
        .add_resource_path(resource_dir)
        .build()?;
```

As shown above, we get them from the `SCREEN_W` and `SCREEN_H` variables that in turn are declared in `game/config.rs`. Cool!

All told, this blank window is exciting but also not very dynamic. So how about we...

## Draw the background

Here we go! Our first task in creating our crab game. Time to do some coding!

Let's take a look at `mod.rs`, where the main logic of the event loop takes place. In particular, let's examine the `draw` function:

```
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        /*
        * TODO: Draw the background
        */
        for s in self.snacks.iter() {
            s.draw(ctx, &self.assets.snack_image)?;
        }
        self.crab.draw(ctx, &self.assets.crab_image)?;
        self.player1.draw(ctx, &self.assets.claw_left)?;
        self.player2.draw(ctx, &self.assets.claw_right)?;

        self.render_ui(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
```

Let's go over the steps here:

1. Clear the window's graphics
2. Iterate over all snacks and draw each
3. Draw the crab
4. Draw player 1
5. Draw player 2
6. Render the UI (Score, etc.)
7. Do the actual drawing
8. Return an `Ok` `GameResult`

One thing you'll find here is a TODO.

The TODO is asking that we draw the background image.

Let's take a look at [graphics::draw](https://docs.rs/ggez/0.5.1/ggez/graphics/fn.draw.html).

The signature is made up of three parts:

- A graphics `Context`
- A `Drawable`
- A set of drawing `params`

We already have access to our graphics context `ctx`, next we need our `Drawable`.

This is to be our image! Let's have a look at our `Assets` and see which images we have available:

```
Assets {
            crab_image,
            claw_left,
            claw_right,
            bg_image,
            snack_image,
            font,
            bg_sound,
            snap_sound
        }
```

In this case, we'll take the `bg_image`.

Next come in our `params`. The only one needed here is the [position](https://docs.rs/ggez/0.5.1/ggez/graphics/struct.DrawParam.html): `graphics::DrawParam::new().dest(Point2::new(0., 0.))`

Let's replace the TODO with the following:

```
        graphics::draw(ctx,
                       &self.assets.bg_image,
                       graphics::DrawParam::new().dest(Point2::new(0., 0.)))?;
```

Let's re-run the game, et voilà! You now have a sandy beach on your game window.
