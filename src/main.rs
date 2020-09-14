
use ggez::GameResult;

mod application;
use application::Application;

fn main() -> GameResult<()> {
  let app = Application::new();
  app.run()
}