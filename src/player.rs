use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};

const PLAYER_DIMENSIONS: f32 = 30.0;
const PLAYER_MOVEMENT_SPEED: f32 = 5.0;

pub struct Player {
    location: Vector2<f32>,
    width: f32,
    height: f32,
    mesh: Mesh,
}

impl Player {
    pub fn new(x: f32, y: f32, context: &mut Context, color: Color) -> GameResult<Player> {
        fn create_mesh(context: &mut Context, color: Color) -> GameResult<Mesh> {
            let mesh_bounds: Rect = Rect::new(0.0, 0.0, PLAYER_DIMENSIONS, PLAYER_DIMENSIONS);
            let mesh: Mesh = MeshBuilder::new()
                .rectangle(DrawMode::fill(), mesh_bounds, color)
                .build(context)?;

            Ok(mesh)
        }

        Ok(Player {
            location: Vector2::new(x, y),
            width: PLAYER_DIMENSIONS,
            height: PLAYER_DIMENSIONS,
            mesh: create_mesh(context, color)?,
        })
    }

    pub fn get_location(&self) -> Point2<f32> {
        Point2::new(self.location.x, self.location.y)
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        graphics::draw(
            context,
            &self.mesh,
            graphics::DrawParam::default().dest(self.get_location()),
        )?;

        Ok(())
    }

    pub fn arena_bounds(&mut self, arena_width: f32, arena_height: f32) {
        if self.location.y + self.height > arena_height {
            self.location.y = arena_height - self.height;
        }

        if self.location.y < 0.0 {
            self.location.y = 0.0;
        }

        if self.location.x + self.width > arena_width {
            self.location.x = arena_width - self.width;
        }

        if self.location.x < 0.0 {
            self.location.x = 0.0;
        }
    }

    pub fn up(&mut self) {
        self.location.y -= PLAYER_MOVEMENT_SPEED;
    }

    pub fn down(&mut self) {
        self.location.y += PLAYER_MOVEMENT_SPEED;
    }

    pub fn left(&mut self) {
        self.location.x -= PLAYER_MOVEMENT_SPEED;
    }

    pub fn right(&mut self) {
        self.location.x += PLAYER_MOVEMENT_SPEED;
    }
}
