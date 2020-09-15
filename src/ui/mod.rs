
use ggez;

mod container;
pub use container::Container;

pub trait Widget {
  fn build(&mut self, _ctx: &mut BuildContext) -> ggez::GameResult<()>;
}

pub struct BuildContext<'a> {
  pub draw_context: &'a mut ggez::Context,
}