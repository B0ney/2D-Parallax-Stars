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
mod vector;
use vector::{
    Vec3D,
    Vec2D
};
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
static STAR_COLORS: [(f32,f32,f32,f32); 3] = [
    (0.966, 0.786, 1.0, 1.0), // light pink magenta
    //(0.752, 1.0, 1.0, 0.8), // light 
    (1.0, 1.0, 0.851, 1.0), 
    (1.0, 1.0, 1.0, 1.0), // white
];
fn main() {

    let (mut ctx, event_loop) = ContextBuilder::new("2D Parallax Stars", "B0ney")
        .window_mode(
            conf::WindowMode::default()
                .dimensions(WIDTH as f32, HEIGHT as f32)
                //.fullscreen_type(conf::FullscreenType::True)
                //.resizable(true)
            )
        .window_setup(
            conf::WindowSetup::default()
                .title("Stars")
                //.icon("icon.png")
        )
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    starfield: stars2::Stars,
    camera: Vec3D,
    orientation: Vec3D,
    render_mode: usize,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            starfield: stars2::Stars::new(100, WIDTH as f32, HEIGHT as f32),
            camera: Vec3D::new(-1.0, -0.75, 0.0),
            orientation: Vec3D::new(0.0, 0.0, 0.0),
            render_mode: 1,
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

        if input::keyboard::is_key_pressed(&_ctx, KeyCode::Key1) {
            self.render_mode = 1;
        };
        if input::keyboard::is_key_pressed(&_ctx, KeyCode::Key2) {
            self.render_mode = 2;
        };

        if self.render_mode == 1 {
            let mouse_pos = input::mouse::position(_ctx);
            let mouse_x = (mouse_pos.x - WIDTH as f32 * 0.5) / 50000.0;
            let mouse_y = (mouse_pos.y - HEIGHT as f32 * 0.5) / 50000.0;

            self.starfield.step(mouse_x , mouse_y );

        } else {
            if input::keyboard::is_key_pressed(&_ctx, KeyCode::S) {
                self.camera.y += 0.005;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::W) {
                self.camera.y -= 0.005;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::D) {
                self.camera.x += 0.0050;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::A) {
                self.camera.x -= 0.005;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::Z) {
                self.camera.z += 0.005;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::X) {
                self.camera.z -= 0.005;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::J) {
                self.orientation.y += 0.005;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::L) {
                self.orientation.y -= 0.005;
            }

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::N) { // :)
                self.orientation.z += 0.005;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::M) { // :)
                self.orientation.z -= 0.005;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::I) {
                self.orientation.x += 0.005;
            };

            if input::keyboard::is_key_pressed(&_ctx, KeyCode::K) {
                self.orientation.x -= 0.005;
            };

        };
              
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        
        if self.starfield.stars.len() > 0 { // program will crash if it tries to build a mesh with < 3 vertices..
            let mb = &mut graphics::MeshBuilder::new();
            let mut point: Vec2D;
            
            for (index, (x, y, z)) in self.starfield.stars.iter().enumerate() {
                if self.render_mode == 1 {
                    point = Vec2D::new((*x + 1.0) * (WIDTH as f32 * 0.5), (*y + 1.0) * (HEIGHT as f32 * 0.5) );
                
                } else {
                    point = Vec3D::project2(
                        &Vec3D::from((*x,*y,*z)),
                        self.orientation,
                        self.camera,
                        30.0,
                        WIDTH as f32,
                        HEIGHT as f32,
                        0.1,
                        1000.0,
                        );
                }
                mb.circle(
                    graphics::DrawMode::fill(),
                    Point2{x: point.x, y: point.y},
                    1.0,
                    1.2,
                    STAR_COLORS[index % STAR_COLORS.len()].into()
                )?;
            };
            let m = mb.build(ctx)?;

            graphics::draw(ctx, &m, graphics::DrawParam::new())?;
        };

        let text = Text::new(format!(
            " Stars: {}\n FPS: {}\n Up - Add Star\n Down - Delete Star\n\n 1 - 2D parallax Mode\n 2 - 3D mode\n\n W,A,S,D - Move 3D camera\n Z,X - 3D camera depth\n\n I,K - Pitch\n J,L - Yaw\n N,M - Roll ",
            &self.starfield.stars.len(),
            timer::fps(&ctx)
            )
        ); 
        graphics::draw(ctx, &text, graphics::DrawParam::new())?;

        graphics::present(ctx)
    }




}