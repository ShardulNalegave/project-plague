
use crate::ui::{BuildContext, Widget};
use cgmath as math;
use ggez::{
  self,
  GameResult,
  graphics::{self, Color},
};

pub struct ScaffoldConfig {
  pub screen_dim: math::Vector2<f32>,
}

pub struct Scaffold<'a> {
  build_context: BuildContext<'a>,
  config: ScaffoldConfig,
  body: Box<dyn Widget>,
}

impl<'a> Scaffold<'a> {
  pub fn new(ctx: BuildContext, config: ScaffoldConfig, body: Box<dyn Widget>) -> Scaffold {
    Scaffold { config, build_context: ctx, body }
  }

  pub fn render(&mut self) {
    self.body.render(&mut self.build_context);
  }
}