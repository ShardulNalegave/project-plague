
use ggez;
use ggez::graphics;
use crate::ui::{Widget, BuildContext};

pub struct Container {}

impl Widget for Container {
  fn build(&mut self, ctx: &mut BuildContext) -> ggez::GameResult<()> {
    let mut rect = graphics::Mesh::new_rectangle(
      ctx.draw_context,
      graphics::DrawMode::fill(),
      graphics::Rect::new(0.0, 0.0, 100.0, 100.0),
      graphics::WHITE,
    )?;
    graphics::draw(ctx.draw_context, &mut rect, graphics::DrawParam::default())
  }
}