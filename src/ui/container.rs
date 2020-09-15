
use cgmath as math;
use ggez;
use ggez::graphics;
use crate::ui::{Widget, BuildContext};

pub struct Container {
  pub pos: math::Vector2<f32>,
  pub color: graphics::Color,
  pub size: math::Vector2<f32>,
}

impl Widget for Container {
  fn render(&mut self, ctx: &mut BuildContext) -> ggez::GameResult<()> {
    let mut rect = graphics::Mesh::new_rectangle(
      ctx.draw_context,
      graphics::DrawMode::fill(),
      graphics::Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y),
      self.color,
    )?;
    graphics::draw(ctx.draw_context, &mut rect, graphics::DrawParam::default())
  }
}