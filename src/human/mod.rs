
use cgmath as math;

pub struct Human {
  pos: math::Vector2<f32>,
}

impl Human {
  pub fn new(pos: math::Vector2<f32>) -> Self {
    Self { pos }
  }

  pub fn render() {}
}