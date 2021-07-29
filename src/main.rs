use ggez::{
    conf,
    event,
    graphics,
    input,
    timer,
    Context,
    ContextBuilder,
    GameResult,
};
use ggez::mint::{Point2};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{Color, Text};

mod stars2;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("2D Parallax Stars", "B0ney")
        .window_mode(conf::WindowMode::default().dimensions(WIDTH as f32, HEIGHT as f32))
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    starfield: stars2::Stars,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            starfield: stars2::Stars::new(100, WIDTH as f32, HEIGHT as f32),
        }
    }
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        if input::keyboard::is_key_pressed(&_ctx, KeyCode::Up) {
            self.starfield.add_star(10);
        };

        if input::keyboard::is_key_pressed(&_ctx, KeyCode::Down) {
            self.starfield.delete_star(10);
        };
        
        let mouse_pos = input::mouse::position(_ctx);

        let mouse_x = (mouse_pos.x - WIDTH as f32 / 2.0) / 20.0;
        let mouse_y = (mouse_pos.y - HEIGHT as f32 / 2.0) / 20.0;

        self.starfield.step(mouse_x, mouse_y);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        
        if self.starfield.stars.len() > 0 { // program will crash if it tries to build a mesh with < 3 vertices..
            let mb = &mut graphics::MeshBuilder::new();
            for (star_x, star_y, _) in &self.starfield.stars {
                mb.circle(
                    graphics::DrawMode::fill(),
                    Point2{x: *star_x, y: *star_y},
                    1.0,
                    1.2,
                    Color::WHITE
                )?;
            };

            let m = mb.build(ctx)?;

            graphics::draw(ctx, &m, graphics::DrawParam::new())?;
        };

        let text = Text::new(format!(
            "Up - Add Star\nDown - Delete Star\nStars: {}\nFPS: {}",
            &self.starfield.stars.len(),
            timer::fps(&ctx)
        )); 

        graphics::draw(ctx, &text, graphics::DrawParam::new())?;

        graphics::present(ctx)
    }
}