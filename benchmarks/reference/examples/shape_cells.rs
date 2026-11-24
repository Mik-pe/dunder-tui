#[path = "../src/styled_grid.rs"]
mod styled_grid;
use ratatui::{
    layout::Rect,
    style::Color,
    symbols::Marker,
    text::Line as TextLine,
    widgets::{
        Widget,
        canvas::{Canvas, Line, Painter, Points, Shape},
    },
};
use std::cell::Cell as Counter;

struct Beacon {
    longitude: f64,
}

impl Shape for Beacon {
    fn draw(&self, p: &mut Painter) {
        Line::new(self.longitude, 0., self.longitude, 2., Color::Indexed(49)).draw(p);
        Points::new(&[(6., 2.)], Color::Red).draw(p);
    }
}

fn main() {
    let coords = [(6., 0.), (3., 1.), (2., 2.), (1., 2.), (0., 0.)];
    let a = Rect::new(3, 2, 7, 3);
    for flags in [false, true] {
        for area in [
            a,
            Rect::new(5, 2, 4, 3),
            Rect::new(9, 4, 1, 1),
            Rect::new(5, 3, 0, 1),
        ] {
            for marker in [
                Marker::Dot,
                Marker::Block,
                Marker::Bar,
                Marker::Braille,
                Marker::HalfBlock,
                Marker::Quadrant,
                Marker::Sextant,
                Marker::Octant,
                Marker::Custom('*'),
                Marker::Custom('界'),
                Marker::Custom(' '),
                Marker::Custom('\u{301}'),
                Marker::Custom('\t'),
                Marker::Custom('\0'),
            ] {
                let mut buf = styled_grid::seed(a, flags);
                let calls = Counter::new(0);
                for phase in 0..4 {
                    let bg = if phase % 2 == 0 {
                        Color::Reset
                    } else {
                        Color::Rgb(23, 35, 73)
                    };
                    Canvas::default()
                        .x_bounds([0., 6.])
                        .y_bounds([0., 2.])
                        .background_color(bg)
                        .marker(marker)
                        .paint(|ctx| {
                            calls.set(calls.get() + 1);
                            if phase == 0 {
                                return;
                            }
                            ctx.draw(&Points::new(&coords, Color::Cyan));
                            if phase >= 2 {
                                ctx.layer();
                                ctx.draw(&Beacon { longitude: 2. });
                            }
                            if phase == 3 {
                                ctx.marker(Marker::Block);
                                ctx.draw(&Points::new(&[(3., 1.)], Color::Blue));
                                ctx.print(0., 0., TextLine::styled("界Z", Color::Yellow));
                            }
                        })
                        .render(area, &mut buf);
                    assert_eq!(calls.get(), if area.is_empty() { 0 } else { phase + 1 });
                    println!("{}", serde_json::to_string(&buf).unwrap());
                }
            }
        }
    }
}
