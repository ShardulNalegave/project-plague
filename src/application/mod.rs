
mod state;
use state::ApplicationState;

use ggez;
use ggez::{event, GameResult};

pub struct Application {
  state: ApplicationState,
}

impl Application {
  pub fn new() -> Self {
    Self { state: ApplicationState::new().unwrap(), }
  }

  pub fn run(mut self) -> GameResult<()> {
    let cb = ggez::ContextBuilder::new("plague", "shardul_nalegave");
    let (ctx, event_loop) = &mut cb.build()?;
    event::run(ctx, event_loop, &mut self.state)
  }
}