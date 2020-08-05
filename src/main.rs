mod player;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, EventsLoop};
use ggez::graphics::Color;
use ggez::{graphics, Context, ContextBuilder, GameResult};

use crate::player::Player;

const GREY: Color = Color::new(0.50, 0.50, 0.50, 1.00);
const TURQUOISE: Color = Color::new(0.00, 1.00, 1.00, 1.00);
const ORANGE: Color = Color::new(1.00, 0.40, 0.10, 1.00);

fn main() {
    // 1080p window with some margin.
    let window_mode: WindowMode = WindowMode::default().dimensions(1800.0, 1000.0);

    let window_setup: WindowSetup = WindowSetup::default().title("Hammer Bros");

    let (mut context, mut event_loop): (Context, EventsLoop) =
        ContextBuilder::new("hammer_bros", "We Dem Boiz Games")
            .window_mode(window_mode)
            .window_setup(window_setup)
            .build()
            .expect("Hã!? Could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game: MyGame = MyGame::new(&mut context).unwrap();

    // Run!
    match event::run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

pub struct MyGame {
    // Your state here...
    players: Vec<Player>,
}

impl MyGame {
    pub fn new(context: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let player1: Player = Player::new(200.0, 200.0, context, TURQUOISE)?;
        let player2: Player = Player::new(800.0, 800.0, context, ORANGE)?;
        let players: Vec<Player> = vec![player1, player2];

        Ok(MyGame { players })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, GREY);

        for i in 0..self.players.len() {
            self.players[i].draw(context)?;
        }

        graphics::present(context)?;
        Ok(())
    }
}
