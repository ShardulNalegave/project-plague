
use ggez;
use ggez::{graphics, Context, GameResult};
use crate::Human;

pub struct ApplicationState {
  humans: Vec<Human>,
}

impl ApplicationState {
  pub fn new() -> GameResult<ApplicationState> {
    let state = ApplicationState {
      humans: Vec::new(),
    };
    Ok(state)
  }
}

impl ggez::event::EventHandler for ApplicationState {

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> { Ok(()) }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::set_window_title(ctx, "Plague");
    graphics::clear(ctx, [0.1, 0.12, 0.15, 1.0].into());
    graphics::present(ctx)?;
    Ok(())
  }
}