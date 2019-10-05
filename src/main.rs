use ggez::*;


struct State {
    dt: std::time::Duration
}

impl State {
    fn new() -> ggez::GameResult<State> {
        let s = State {
            dt: std::time::Duration::new(0, 0)
        };
        Ok(s)
    }

}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        Ok(())
    }
}


pub fn main() -> ggez::GameResult {
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("crab", "lislis & ramonh")
        .conf(c)
        .build()?;
    let state = &mut State::new()?;
    event::run(ctx, event_loop, state)
}
