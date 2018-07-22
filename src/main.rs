extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::graphics::{self, DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};

struct MainState {
    image1: graphics::Image,
    zoomlevel: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image1 = graphics::Image::new(ctx, "/imp.png")?;
        let s = MainState {
            image1,
            zoomlevel: 1.0,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.zoomlevel += 0.01;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        let origin = graphics::Point2::new(200.0, 200.0);
        let origin2 = graphics::Point2::new(400.0, 200.0);
        let point_middle = Point2::new(0.5, 0.5);

        /*
        graphics::draw_ex(
            ctx,
            &self.image1,
            graphics::DrawParam {
                dest: origin,
                scale: graphics::Point2::new(-1.0, 1.0),
                ..Default::default()
            },
        )?;
        */

        /*
        graphics::draw_ex(
            ctx,
            &self.image1,
            graphics::DrawParam {
                dest: origin,
                offset: point_middle,
                scale: graphics::Point2::new(-1.0, 1.0),
                ..Default::default()
            },
        )?;
        */

        /*
        graphics::draw_ex(
            ctx,
            &self.image1,
            graphics::DrawParam {
                dest: origin,
                offset: point_middle,
                scale: graphics::Point2::new(1.0, -1.0),
                rotation: 0.3,
                ..Default::default()
            },
        )?;
        */

        graphics::draw_ex(
            ctx,
            &self.image1,
            graphics::DrawParam {
                dest: origin,
                offset: point_middle,
                scale: graphics::Point2::new(0.5, 0.5),
                // rotation: 0.3,
                ..Default::default()
            },
        )?;

        graphics::draw_ex(
            ctx,
            &self.image1,
            graphics::DrawParam {
                dest: origin2,
                offset: point_middle,
                scale: graphics::Point2::new(-0.5, 0.5),
                // rotation: 0.3,
                ..Default::default()
            },
        )?;

        graphics::set_color(ctx, (0, 255, 0).into())?;
        graphics::circle(ctx, DrawMode::Line(2.0), origin, 10.0, 1.0)?;
        graphics::circle(ctx, DrawMode::Line(2.0), origin2, 10.0, 1.0)?;

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    // test test
    let window_conf = conf::WindowSetup::default()
        .resizable(true)
        .title("flip_scale_test");
    let mut ctx = ggez::ContextBuilder::new("flip_scale_test", "ozkriff")
        .window_setup(window_conf)
        .add_resource_path("assets")
        .build()?;
    println!("{}", graphics::get_renderer_info(&ctx)?);
    let state = &mut MainState::new(&mut ctx).unwrap();
    event::run(&mut ctx, state)
}
