
use cgmath as math;
use ggez;
use ggez::{graphics, Context, GameResult};

use crate::ui::{
  Widget,
  BuildContext,
  Container,
};

pub struct ApplicationState {}

impl ApplicationState {
  pub fn new() -> ApplicationState {
    ApplicationState {}
  }
}

impl ggez::event::EventHandler for ApplicationState {

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> { Ok(()) }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::set_window_title(ctx, "Plague");
    graphics::clear(ctx, [0.1, 0.12, 0.15, 1.0].into());

    {
      let mut build_context = BuildContext {
        draw_context: ctx,
      };
      let mut container = Container {
        pos: math::Vector2 { x: 0.0, y: 0.0 },
        size: math::Vector2 { x: 150.0, y: 100.0 },
        color: graphics::WHITE,
      };
      container.build(&mut build_context);
    }

    graphics::present(ctx)?;
    Ok(())
  }
}