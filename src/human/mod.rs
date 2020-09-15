
use ggez;
use ggez::GameResult;
use ggez::graphics;
use ggez::nalgebra as na;
use cgmath as math;
use rand::Rng;

pub struct Human {
  pos: math::Vector2<f32>,
}

impl Human {
  pub fn new(pos: math::Vector2<f32>) -> Self {
    Self { pos }
  }

  pub fn walk(&mut self) {
    self.pos = math::Vector2 {
      x: self.pos.x + rand::thread_rng().gen::<f32>(),
      y: self.pos.y + rand::thread_rng().gen::<f32>(),
    };
  }

  pub fn render(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
      ctx,
      graphics::DrawMode::fill(),
      na::Point2::new(0.0, 0.0),
      10.0,
      1.0,
      graphics::WHITE,
    )?;
    graphics::draw(ctx, &circle, (na::Point2::new(self.pos.x, self.pos.y),))
  }
}