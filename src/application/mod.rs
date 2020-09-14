
mod state;
use state::ApplicationState;

use ggez::{self, ContextBuilder};
use ggez::{event, GameResult};

pub struct Application {
  context_builder: ContextBuilder,
  state: ApplicationState,
}

impl Application {
  pub fn new() -> Self {
    let context_builder = ContextBuilder::new("plague", "shardul_nalegave");
    Self { context_builder, state: ApplicationState::new().unwrap(), }
  }

  pub fn run(mut self) -> GameResult<()> {
    let (ctx, event_loop) = &mut self.context_builder.build()?;
    event::run(ctx, event_loop, &mut self.state)
  }
}