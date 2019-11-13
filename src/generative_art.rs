extern crate rand;

use ggez::*;
use rand::{thread_rng, Rng};

enum Shape {
    Circle(mint::Point2<f32>, f32, graphics::Color),
    Rectangle(graphics::Rect, graphics::Color),
}

struct State {
    shapes: Vec<Shape>,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for shape in &self.shapes {
            let mesh = match shape {
                &Shape::Rectangle(rect, color) => {
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?
                }
                &Shape::Circle(origin, radius, color) => graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    origin,
                    radius,
                    0.1,
                    color,
                )?,
            };

            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let mut shapes = Vec::new();
    for _ in 0..8 {
        let color = graphics::Color::new(
            thread_rng().gen(),
            thread_rng().gen(),
            thread_rng().gen(),
            thread_rng().gen(),
        );
        if thread_rng().gen_range(0, 2) % 2 == 0 {
            shapes.push(Shape::Rectangle(
                ggez::graphics::Rect::new(
                    thread_rng().gen_range(0.0, 800.0),
                    thread_rng().gen_range(0.0, 600.0),
                    thread_rng().gen_range(0.0, 800.0),
                    thread_rng().gen_range(0.0, 600.0),
                ),
                color,
            ));
        } else {
            shapes.push(Shape::Circle(
                mint::Point2 {
                    x: thread_rng().gen_range(0.0, 800.0),
                    y: thread_rng().gen_range(0.0, 600.0),
                },
                thread_rng().gen_range(0.0, 300.0),
                color,
            ));
        }
    }
    let state = &mut State { shapes };
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
