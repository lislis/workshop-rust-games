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

## Implementing the `crab`

Next, let's take a look at `crab.rs`. Our crustacean friend will move left-to-right and then right-to-left, depending on which way they're going.

Let's first take a gander at the `Crab` struct:

```
pub struct Crab {
    pub location: Point2,
    velocity: Vector2,
    w: f32,
    s: f32
}
```

What we've got here is:
- a `Point2` (that is, [a 2-dimensional point in 2D space](https://www.nalgebra.org/rustdoc/nalgebra/geometry/type.Point2.html)) that makes up the crab's location inside the game space
- a `Vector2` making up the [velocity](https://www.nalgebra.org/rustdoc/nalgebra/base/type.Vector2.html) the crab is headed in (that being left or right)
- an `f32` (floating point number) that describes the crab's width (`w`)
- and finally, an `f32` describing the crab's natural speed (`s`)

You'll notice that the `location` attribute is preceeded by the `pub` declaration. This allows the crab's location to be called up outside the `Crab` module!

Next, we've got our `impl` section for the crab, along with some finished and non-finished functions. Let's first take a look at what happens when a new `crab` is instantiated:

```
    pub fn new(location: Point2) -> GameResult<Crab> {
        let c = Crab {
            location,
            velocity: Vector2::new(CRAB_S, 0.0),
            w: CRAB_W,
            s: CRAB_S
        };
        Ok(c)
    }
```

What this function does is it takes a starting location as a parameter and results a `GameResult` wrapped around a seaworthy new crab!

Also worthy of note are the default values for the crab's attributes, most of which the velocity being initially set to `[CRAB_S, 0.0]`. In geometric terms, this crab will move to the right, providing `CRAB_S` is set to a positive value. A quick glance at `config.rs` confirms that this is indeed the case!

Before we continue, it's worth noting that in `mod.rs` we call the crab's `draw` function inside the event loop's corresponding `draw` function:

```
self.crab.draw(ctx, &self.assets.crab_image)?;
```

You may wonder why it does nothing, right? Well, going back to `crab.rs` and inspecting the `draw` function here reveals the culprit:

```
    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        /*
        * TODO: Draw crab image
        */
        Ok(self)
    }
```

We gotta do the actual drawing!

# YO RAMON LINK TO THE BACKGROUND DRAWING SECTION WHEN YOU"RE DONE

As you may recall from drawing the background graphics, we'll be calling `graphics::draw`

Our params in this case will be a little more complex, however, as we'll also be scaling the image down (the original is large!):

```
    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        let drawparams = graphics::DrawParam::new()
            .dest(self.location)
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, drawparams)?;
        Ok(self)
    }
```

Done! Now you can run the game and be greeted by our debonaire crab. 

Looking good, crab! But now they've gotta start moving. Let's look at the crab's `update` method, which in turn is called from the `update` method in the event loop, implemented in `mod.rs`:

```
    pub fn update(&mut self, parent_loc: Point2) -> GameResult<&Self> {
        /*
        * TODO: Update claw location according to body's location
        */
        Ok(self)
    }
```

Another TODO! What we want to do now is adjust the crab's location based on its velocity. Remember, the `update` function will be called for every frame refresh, so every time the game screen refreshes, the crab will be moved only as much as its velocity dictates:

```
        self.location.x += self.velocity.x;
```

With that said and done, let's run our game!

There they go!... And still going... And gone. Some say the crab's moving right even as we speak.

No problem, all we have to do is after updating the crab's location, check if they've reached the right-most end of the screen. We'll do that by comparing the location of the crab with the right-most end of the screen. Good thing we're passing that `max_screen` parameter! If it does, we just set the `x` factor of the velocity to be the same, but negative:

```
        if self.location.x + (self.w * 2.) >= max_screen {
            self.velocity.x = - self.s;
        }
```

With that done, let's run the game again!

You'll notice that the crab successfully bounces of the right side of the screen. But wait, what about the left side... oh no there it goes.

You might be guessing already! We now need to check if the location of the crab has reached the left-most side:

```
    pub fn update(&mut self, max_screen: f32) -> GameResult<&Self> {
        self.location.x += self.velocity.x;
        if self.location.x + (self.w * 2.) >= max_screen {
            self.velocity.x = - self.s;
        } else if self.location.x < self.w {
            self.velocity.x = self.s;
        }
        Ok(self)
    }
```

With this done, let's give this one more try...

Magnificent! The crab gallantly bounces from end to end. Well done! We now have a living and breathing crab buddy.
