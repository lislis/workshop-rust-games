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

## Implementing the `player`

Let's briefly open `player.rs` and examine its `Player` struct:

```
pub struct Player {
    pub score: usize,
    pub claw: Claw
}
```

Let's see here, we've got a `usize` describing the player's score and a...

A `claw`, eh? Well, if we're going to be concerning ourselves with a claw, let's implement that right away! 


## Implementing the ~~`player`~~ `crab`

Well, let's glance at that too, over at `claw.rs`:

```
pub struct Claw {
    pub location: Point2,
    body_anchor: Vector2,
    joint_anchor: Vector2,
    w: f32,
    h: f32,
    s: f32
}
```

Let's see what we've got here:

- A location `Point2`, [just like we had on the crab](https://github.com/lislis/workshop-rust-games/tree/writeup#implementing-the-crab)
- An body anchor `Vector2` to determine the position of the edge of the crab relative to the claw
- An joint anchor `Vector2` to determine the position of the claw itself
- An `f32` to determine the width `w`
- An `f32` to determine the height `h`
- An `f32` to determine the speed `s`

With that said, let's take a look at the function to create a `new` `Claw`:

```
    pub fn new(location: Point2,
               body_anchor: Vector2,
               joint_anchor: Vector2) -> GameResult<Claw> {
        let c = Claw {
            location,
            body_anchor,
            joint_anchor,
            w: CLAW_W,
            h: CLAW_H,
            s: CLAW_S
        };
        Ok(c)
    }
```

Cool! So we give it a location, the body anchor as well as the joint anchor.

You may have noticed a function called `get_origin`. Why do we need this?

In order to go into it, we need to look closely into the relationship between the `location`, `body_anchor`, `joint_anchor` and the claw's origin:

- The `location`, as stated before, is where the crab is in the game space
- The `body_anchor` is the point on the crab where their arm begins (this is either the leftmost or rightmost side of the crab, depending on which claw we're working on.)
- The `joint_anchor` is the location of the bottom horizontal middle of the claw
- The claw's origin is then the top-left corner of the claw

The aim of the `get_origin` function is therefore to dynamically calculate the origin of the claw. We can calculate each coordinate of this origin:

The `x` coordinate would be the `x` coordinate of the horizontal middle of the claw (based on the `joint_anchor` relative to the crab) minus a half of the width of the claw
- The `y` coordinate would be the `y` coordinate of the bottom of the claw (based on the `joint_anchor` relative to the crab) minus the height of the claw

Both of the above mention a relative position. This requires adding the location of the crab to the `joint_anchor` and then calculating the coordinates and then returning a new `Point`:

```
    pub fn get_origin(&self) -> Point2 {
        let joint_position = self.location + self.joint_anchor;
        let x = joint_position.x - self.w / 2.;
        let y = joint_position.y - self.h;
        Point2::new(x, y)
    }
```

Not too shabby! Now that we have this origin we can get to `draw`'ing the claw itself:

```
    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        /*
        * TODO: 
        * 1. Draw a pure red line from the body to the claw
        * 2. Draw the claw image
        */
        Ok(self)
    }
```

First, let's draw the claw image itself from part 2, just like we draw the crab, except we're using the origin:

```
        let drawparams = graphics::DrawParam::new()
            .dest(self.get_origin())
            .rotation(0.0)
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, drawparams)?;
```

Once we've done that, try running the game. The crab now has disembodied claws floating around them! A great step, but er...

Next, we'll draw the arm of the crab. This will involve drawing a [`graphics::Mesh::new_line`](https://docs.rs/ggez/0.5.1/ggez/graphics/struct.Mesh.html#method.new_line) from the `body_anchor` to the `joint_anchor`. The function takes four parameters:

- The graphics context
- A vector (list) of points. That is, a start and finish of the line.
- The width of the line
- A [`Color`](https://docs.rs/ggez/0.5.1/ggez/graphics/struct.Color.html)

Let's first construct our color. It takes a series of `rgba` numbers. We'll need a red color, so we'll assign it accordingly. We'll then need the relative locations of our body and joints respectively, and then we're ready to go:

```
        let redColor = graphics::Color::new(1.0, 0.0, 0.0, 1.0);
        let body_location = self.location + self.body_anchor;
        let joint_location = self.location + self.joint_anchor;
        let arm = graphics::Mesh::new_line(ctx,
                                           &[body_location,
                                             joint_location],
                                           10.,
                                           redColor)?;       
```

Once we have this, all we gotta do is draw the arm:

```
        graphics::draw(ctx, &arm, graphics::DrawParam::default())?;
```

Check it out! You now have a crab with little claws going around. 

Let's take a step back and look at our completed `draw` function:

```
    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        let b_anchor = self.location + self.body_anchor;
        let j_anchor = self.location + self.joint_anchor;

        let arm = graphics::Mesh::new_line(ctx,
                                           &[b_anchor,
                                             j_anchor],
                                           10.,
                                           graphics::Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::draw(ctx, &arm, graphics::DrawParam::default())?;

        let drawparams = graphics::DrawParam::new()
            .dest(self.get_origin())
            .rotation(0.0)
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, drawparams)?;

        Ok(self)
```

Phew! Looks good.

Let's move onto the `update` function now, which will be considerably quicker:

```
    pub fn update(&mut self, parent_loc: Point2) -> GameResult<&Self> {
        /*
        * TODO: Update claw location according to body's location
        */
        Ok(self)
    }
```

As you saw when drawing the claw, the claw's `location` attribute is that of the crab. `parent_loc` in this context refers to the location of the crab, so in order to `update` the claw, we just need to update the `location` attribute:

```
self.location = parent_loc
```

Fantastic! If we run this, the claws will now move relative to the crab. Very cool!

One more function to go! This time, we've got the `movedir` function:

```
    pub fn movedir(&mut self, dir:Directions) -> Vector2 {
        /*
        * TODO: Change joint anchor according to direction
        */
        Vector2::new(0., 0.)
    }
```

You'll notice it takes a `Directions` as a parameter. This is an enum we delcared here in `claw.rs`:

```
pub enum Directions {
    Up,
    Down,
    Left,
    Right
}
```

Depending on which direction the claw receives, it'll move accordingly.

In order to move the claw, we need to update its `joint_anchor`. We'll use pattern matching (You can read the [Rust docs on pattern matching](https://doc.rust-lang.org/1.6.0/book/patterns.html) if you don't know what it is. It's super cool!) on the received direction to do this. Replace the contents of the function with the following:

```
    pub fn movedir(&mut self, dir:Directions) -> Vector2 {
        match dir {
            Directions::Up => {
                self.joint_anchor.y -= self.s;
            },
            Directions::Down => {
                self.joint_anchor.y += self.s;
            },
            Directions::Right => {
                self.joint_anchor.x += self.s;
            },
            Directions::Left => {
                self.joint_anchor.x -= self.s;
            }
        }
        self.joint_anchor
    }
```

Depending on which direction is received by the claw, it'll update the coordinate of its joint accordingly.

With that done, we've still got a ways to go until we can test this movement, but hey! Quick pat on the back, as you've implemented the claw!

Give it a quick run and check that it still compiles.

## Implementing the `player` (for realsies this time)

Alrighty, back to `player.rs`! Let's look at the function for declaring a `new` player:

```
    pub fn new(loc: Point2,
               b_anchor: Vector2,
               j_anchor: Vector2) -> GameResult<Player> {
        let p = Player {
            score: 0,
            claw: Claw::new(loc,
                            b_anchor,
                            j_anchor)?
        };
        Ok(p)
    }
```

Great, so to declare a new player, we need to pass to it the crab's location, a body anchor and a joint anchor. As you saw in the previous section, these will all be used by the player's corresponding claw!

Taking a quick glance back at `state.rs`, we declare not one, but two instances of the `Player` struct (makes sense, this is a 2-player game after all!):

```
        let assets = Assets::new(ctx)?;
        let (width, height) = ggez::graphics::drawable_size(ctx);
        let crab_origin = Point2::new(width / 2.0 - (CRAB_W / 2.0),
                                      height - CRAB_H);

        let s = State {
            player1: Player::new(crab_origin,
                                 Vector2::new(CLAW_W - 20., CRAB_H / 2.),
                                 Vector2::new(-30., -20.))?,
            player2: Player::new(crab_origin,
                                 Vector2::new(CRAB_W + 30.0, CRAB_H / 2.),
                                 Vector2::new(170.0, -20.0))?,
            crab: Crab::new(crab_origin)?,
            snacks: spawn_snacks(NUM_SNACKS),
            screen_width: width,
            assets: assets
        };
```

In case you were wondering, this is where we determine the respective left and right body and joint anchors for each player's claw.

Cool! Next we'll implement the empty functions. 

First off, let's start with the `increase_score` function:

```
    pub fn increase_score(&mut self) -> GameResult<&Self> {
        /*
        * TODO: Make score go up
        */
        Ok(self)
    }
```

Every time the player grabs a snack, their score will go up by one. Let's make that happen!

```
    pub fn increase_score(&mut self) -> GameResult<&Self> {
        self.score += 1;
        Ok(self)
    }
```

Noice! As for the rest of the functions....

```
    pub fn update(&mut self, new_loc: Point2) -> GameResult<&Self> {
        /*
        * TODO: Update claw
        */
        Ok(self)
    }

    pub fn draw(&self,
                ctx: &mut Context,
                img: &graphics::Image) -> GameResult<&Self> {
        /*
        * TODO: Draw claw
        */
        Ok(self)
    }

    pub fn movedir(&mut self, dir: Directions) {
        /*
        * TODO: Move the claw
        */
    }
```

You might notice, but these functions serve mostly as wrappers around the claw, meaning we can just pass these function calls directly to the player's claw! Let's do that:

```
    pub fn update(&mut self, new_loc: Point2) -> GameResult<&Self> {
        self.claw.update(new_loc)?;
        Ok(self)
    }

    pub fn draw(&self,
                ctx: &mut Context,
                img: &graphics::Image) -> GameResult<&Self> {
        self.claw.draw(ctx, &img)?;
        Ok(self)
    }

    pub fn movedir(&mut self, dir: Directions) {
        self.claw.movedir(dir);
    }
```

Not bad, eh? With that, we've deftly dealt with the player! 

## Getting the players to move.

Now comes the one of the big parts! You see, the game will be controlled by the keyboard. You might've seen already in `state.rs` that this will be done with the `WASD` keys for player 1 and the `IJKL` keys  for player 2, corresponding to up, left, down and right respectively.

Let's pop over the `mod.rs`, where we'll be taking a look at this function:

```
    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        /*
        * TODO: Provide 2 key matches. One for player 1 and the other for player 2
        */
    }
```

This function will be automatically called up when a player **lifts** up their finger from the keyboard (trust us when we say this makes for a good laugh!) What we'll do here is pattern matching like we did in the previous section to determine which key is being pressed and pass that onto the according player with the appropriate direction. 

You'll notice that one of the parameters passed by the function is the `keycode` one. This will let us know which key the player pressed.

Let's try making it happen for player 1:

```
        match keycode {
            KeyCode::W => {
                self.player1.movedir(Directions::Up);
            },
            KeyCode::A => {
                self.player1.movedir(Directions::Left);
            },
            KeyCode::S => {
                self.player1.movedir(Directions::Down);
            },
            KeyCode::D => {
                self.player1.movedir(Directions::Right);
            },
            _ => (),
        }
```

Give it a go! You'll notice that player 1 can already start moving. All good, but what about player 2? We **could** add a few more cases for pattern matching against the keycode, but that would mean that there could be times where both players would trigger the event and one of them would be gobbled up, never to be found! 

What we'll do instead is add an identical pattern matching set of cases below. The completed function will look like this:

```
    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        match keycode {
            KeyCode::W => {
                self.player1.movedir(Directions::Up);
            },
            KeyCode::A => {
                self.player1.movedir(Directions::Left);
            },
            KeyCode::S => {
                self.player1.movedir(Directions::Down);
            },
            KeyCode::D => {
                self.player1.movedir(Directions::Right);
            },
            _ => (),
        }
        match keycode {
            KeyCode::I => {
                self.player2.movedir(Directions::Up);
            },
            KeyCode::J => {
                self.player2.movedir(Directions::Left);
            },
            KeyCode::K => {
                self.player2.movedir(Directions::Down);
            },
            KeyCode::L => {
                self.player2.movedir(Directions::Right);
            },
            _ => (),
        }
    }
```

There we go! Give it a try now. Our crab pal can now move their claws!

> Hey while you're at it, make the crab give itself a highfive!

## Implementing the `snack`s

Now that we've gotten the claw-waving out of our systems, it's time to make the final chunk of the game happen: The snacks!

> Fun fact: Crabs eat algae

We'll begin by taking a look at `snacks.rs`. Specifically, we'll look at the `Snack` struct:

```
pub struct Snack {
    location: Point2,
    velocity: Vector2,
    w: f32,
    active: bool
}
```

Okay what we see here should be mostly familiar to us by now:

- A location `Point2`
- A moving velocity `Vector2`
- An `f32` width `w`
- A boolean flag indicating whether the snack is active

A quick glance at the `new` function shows us the default values:

```
    fn new () -> GameResult<Snack> {
        let s = Snack {
            location: Point2::new(rand::random::<f32>() * SCREEN_W,
                                  rand::random::<f32>() * SCREEN_H - SCREEN_H),
            velocity: Vector2::new(0.0,
                                   rand::random::<f32>() * 2.0 + 0.1),
            w: SNACK_W,
            active: true
        };
        Ok(s)
    }
```

Here we can see why we're using the `rand` crate. The location will be a random `x` coordinate between the left and rightmost sides of the screen, and the `y` will randomly be somewhere above the screen (so that it looks like the snacks are raining down on our crab buddy).

By default, a new snack will always be `active`.

The first function we'll implement is called up by `state.rs` when setting up the game:

```
pub fn spawn_snacks(num: usize) -> Vec<Snack> {
        /*
        * TODO: 
        * Generate snacks
        */
        vec![]
}
```

At the moment, this returns an empty vector of sadness. What we need is to return a vector of snacks. How many, you ask? We can set that up in `config.rs` with the `NUM_SNACKS` variable. Let's make that many, as passed down from State!

```
pub fn spawn_snacks(num: usize) -> Vec<Snack> {
       (0..num).map(|_v| Snack::new()
                 .expect("Could not create snack")).collect()
}
```

We'll iterate `num` times, create a new snack for each, and then call `collect()` to transform the iterator into a vector. All done!

Next we'll `draw` our snack. This should already be somewhat familiar:

```
    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        /*
        * TODO: 
        * Draw the snack, but only if it's active
        */
        Ok(self)
    }
```

That's right, we gotta draw its image! Big caveat here, though: We'll only be drawing the snack if it's active, okay?

```
        if self.active {
            let drawparams = graphics::DrawParam::new()
                .dest(self.location);
            graphics::draw(ctx, img, drawparams)?;
        }
```

Looks good so far! We won't be scaling it, as the image is small enough. 

Alright, next up we'll take a look at the `update` function:

```
    pub fn update(&mut self) -> GameResult<&Self> {
        /*
        * TODO: 
        * 1. Move snack down
        * 2. Set active to false if the snack has left the screen
        * 3. If not active, reset the snack
        */
        Ok(self)
    }
```

Okay three parts here. Let's first move the snack down using its velocity.

```
        self.location += self.velocity;
```

As shown in the `new` function, the velocity will have a random downwards trajectory.

Next, we'll set the snack to be inactive if it's left the bottom of the screen:

```
        if self.location.y > SCREEN_H {
            self.active = false;
        }
```

Looks good! We're checking the `y` coordinate of the snack against the height of the screen.

Next, we'll reset the snack if it's inactive:

```
        if !self.active {
            self.location = Point2::new(rand::random::<f32>() * SCREEN_W,
                                        -SNACK_W);
            self.velocity = Vector2::new(0.0,
                                         rand::random::<f32>() * 2.0 + 0.1);
            self.active = true;
        }
```

You'll notice that the location and velocity are being set in the same way as when initializing a `new` snack. Then we set the `active` flag back to true.

Take a step back and look at the `update` function:

```
    pub fn update(&mut self) -> GameResult<&Self> {
        self.location += self.velocity;
        if self.location.y > SCREEN_H {
            self.active = false;
        }
        if !self.active {
            self.location = Point2::new(rand::random::<f32>() * SCREEN_W,
                                        -SNACK_W);
            self.velocity = Vector2::new(0.0,
                                         rand::random::<f32>() * 2.0 + 0.1);
            self.active = true;
        }
        Ok(self)
    }
```

Looks good! 

Now we've got some geometry to do. Let's look at `collides_with`:

```
    pub fn collides_with(&mut self, other: Point2) -> bool {
        /*
        * TODO: 
        * Draw the snack, but only if it's active
        */
        false
    }
```

