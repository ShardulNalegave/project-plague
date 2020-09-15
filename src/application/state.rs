
use cgmath as math;
use ggez;
use ggez::{graphics, Context, GameResult};
use crate::ui::{BuildContext, Container, Scaffold, ScaffoldConfig};

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
      let screen_dim = graphics::screen_coordinates(ctx);
      Scaffold::new(
        BuildContext {
          draw_context: ctx,
        },
        ScaffoldConfig {
          screen_dim: math::Vector2 { x: screen_dim.w, y: screen_dim.h },
        },
        Box::new(Container {
          pos: math::Vector2 { x: (screen_dim.w / 2_f32) - 150.0, y: (screen_dim.h / 2_f32) - 125.0 },
          size: math::Vector2 { x: 300.0, y: 250.0 },
          color: graphics::WHITE,
        }),
      ).render();
    }

    graphics::present(ctx)?;
    Ok(())
  }
}