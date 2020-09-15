
use ggez::GameResult;

mod application;
use application::Application;

// mod human;
// use human::Human;

fn main() -> GameResult<()> {
  let app = Application::new();
  app.run()
}