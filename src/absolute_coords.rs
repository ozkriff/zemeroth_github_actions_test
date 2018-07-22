extern crate ggez;

use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode, Point2, Rect};
use ggez::conf;
use ggez::event;

struct State {
    image: graphics::Image,
}

impl State {
    fn new(context: &mut Context) -> GameResult<Self> {
        let image = graphics::Image::new(context, "/imp.png")?;
        let mut this = State { image };
        {
            let (w, h) = graphics::get_drawable_size(context);
            this.resize(context, w, h);
        }
        Ok(this)
    }

    // sets [-1.0..1.0] range for height and [-aspect..+aspect] for width
    fn resize(&mut self, context: &mut Context, w: u32, h: u32) {
        let aspect_ratio = w as f32 / h as f32;
        let coordinates = Rect::new(-aspect_ratio, -1.0, aspect_ratio * 2.0, 2.0);
        graphics::set_screen_coordinates(context, coordinates).unwrap();
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context);

        let dest = graphics::Point2::new(0.0, 0.0);

        let height = 0.3;
        let s = height / self.image.height() as f32;
        let scale = Point2::new(s, s);

        graphics::set_color(context, graphics::WHITE)?;

        let basic_params = graphics::DrawParam {
            dest,
            scale,
            ..Default::default()
        };

        // draw an image without offset
        graphics::draw_ex(
            context,
            &self.image,
            basic_params,
        )?;

        // draw an image with offset
        graphics::draw_ex(
            context,
            &self.image,
            graphics::DrawParam {
                offset: Point2::new(0.5, 0.5), // moves sprite to an unexpected screen position O.o
                ..basic_params
            },
        )?;

        // mark [0, 0]
        let red = (0, 255, 0).into();
        graphics::set_color(context, red)?;
        let radius = 0.05;
        let tolerance = 0.01;
        let width = 0.01;
        graphics::circle(context, DrawMode::Line(width), dest, radius, tolerance)?;

        graphics::present(context);
        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    let name = "absolute_coords test";
    let window_conf = conf::WindowSetup::default()
        // .resizable(true)
        .title(name);
    let mut context = ggez::ContextBuilder::new(name, "ozkriff")
        .window_setup(window_conf)
        .add_resource_path("assets")
        .build()?;
    let mut state = State::new(&mut context)?;
    event::run(&mut context, &mut state)
}
