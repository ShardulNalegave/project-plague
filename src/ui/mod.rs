
use ggez;

mod scaffold;
pub use scaffold::Scaffold;
pub use scaffold::ScaffoldConfig;

mod container;
pub use container::Container;

pub trait Widget {
  fn render(&mut self, _ctx: &mut BuildContext) -> ggez::GameResult<()>;
}

pub struct BuildContext<'a> {
  pub draw_context: &'a mut ggez::Context,
}