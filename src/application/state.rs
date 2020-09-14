
use ggez;
use ggez::{graphics, Context, GameResult};

pub struct ApplicationState {}

impl ApplicationState {
  pub fn new() -> GameResult<ApplicationState> {
    let state = ApplicationState {};
    Ok(state)
  }
}

impl ggez::event::EventHandler for ApplicationState {

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> { Ok(()) }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
    graphics::present(ctx)?;
    Ok(())
  }
}