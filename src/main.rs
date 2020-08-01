use ggez::{graphics, mint, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::Color;

fn main() {
    // Make a Context.
    let window_mode: WindowMode = WindowMode::default()
        .dimensions(1800.0, 1000.0); // 1080p with some padding

    let window_setup: WindowSetup = WindowSetup::default()
        .title("Hammer Bros");

    let (mut context, mut event_loop) = ContextBuilder::new("hammer_bros", "This Boiz Games")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut context);

    // Run!
    match event::run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

pub struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_context: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        let grey: Color = [0.5, 0.5, 0.5, 1.0].into();
        let turquoise: Color = [0.00, 0.42, 0.5, 1.0].into();

        graphics::clear(context, grey);
        // Draw code here...
        let circle = graphics::Mesh::new_circle(
            context,
            graphics::DrawMode::fill(),
            mint::Point2{x: 400.0, y: 500.0},
            200.0,
            0.1,
            turquoise,
        )?;
        graphics::draw(context, &circle, graphics::DrawParam::default())?;

        graphics::present(context)?;
        Ok(())
    }
}
