extern crate ggez;

use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode, Point2};
use ggez::conf;
use ggez::event;

struct State {
    image: graphics::Image,
}

impl State {
    fn new(context: &mut Context) -> GameResult<Self> {
        let image = graphics::Image::new(context, "/imp.png")?;
        Ok(State { image })
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context);

        // find center
        let (w, h) = graphics::get_drawable_size(context);
        let dest = graphics::Point2::new(w as _, h as _) / 2.0;

        let s = 0.7;
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
                offset: Point2::new(0.5, 0.5), // works as expected
                ..basic_params
            },
        )?;

        // mark [0, 0]
        let red = (0, 255, 0).into();
        graphics::set_color(context, red)?;
        let radius = 10.00;
        let tolerance = 1.0;
        let width = 1.0;
        graphics::circle(context, DrawMode::Line(width), dest, radius, tolerance)?;

        graphics::present(context);
        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    let name = "window_coords test";
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
