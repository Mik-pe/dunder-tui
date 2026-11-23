use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{
        Color, Style,
        palette::{material, tailwind},
    },
};

fn rgb(color: Color) -> u32 {
    let Color::Rgb(r, g, b) = color else {
        panic!("expected RGB")
    };
    u32::from_be_bytes([0, r, g, b])
}

fn main() {
    let mut colors = Vec::new();
    macro_rules! shades {
        ($family:expr; $($field:ident),+) => { colors.extend([$($family.$field),+]); };
    }
    for family in [
        tailwind::SLATE,
        tailwind::GRAY,
        tailwind::ZINC,
        tailwind::NEUTRAL,
        tailwind::STONE,
        tailwind::RED,
        tailwind::ORANGE,
        tailwind::AMBER,
        tailwind::YELLOW,
        tailwind::LIME,
        tailwind::GREEN,
        tailwind::EMERALD,
        tailwind::TEAL,
        tailwind::CYAN,
        tailwind::SKY,
        tailwind::BLUE,
        tailwind::INDIGO,
        tailwind::VIOLET,
        tailwind::PURPLE,
        tailwind::FUCHSIA,
        tailwind::PINK,
        tailwind::ROSE,
    ] {
        shades!(family; c50, c100, c200, c300, c400, c500, c600, c700, c800, c900, c950);
    }
    colors.extend([tailwind::BLACK, tailwind::WHITE]);
    for family in [
        material::RED,
        material::PINK,
        material::PURPLE,
        material::DEEP_PURPLE,
        material::INDIGO,
        material::BLUE,
        material::LIGHT_BLUE,
        material::CYAN,
        material::TEAL,
        material::GREEN,
        material::LIGHT_GREEN,
        material::LIME,
        material::YELLOW,
        material::AMBER,
        material::ORANGE,
        material::DEEP_ORANGE,
    ] {
        shades!(family; c50, c100, c200, c300, c400, c500, c600, c700, c800, c900);
    }
    for family in [material::BROWN, material::GRAY, material::BLUE_GRAY] {
        shades!(family; c50, c100, c200, c300, c400, c500, c600, c700, c800, c900);
    }
    for family in [
        material::RED,
        material::PINK,
        material::PURPLE,
        material::DEEP_PURPLE,
        material::INDIGO,
        material::BLUE,
        material::LIGHT_BLUE,
        material::CYAN,
        material::TEAL,
        material::GREEN,
        material::LIGHT_GREEN,
        material::LIME,
        material::YELLOW,
        material::AMBER,
        material::ORANGE,
        material::DEEP_ORANGE,
    ] {
        shades!(family; a100, a200, a400, a700);
    }
    colors.extend([material::BLACK, material::WHITE]);
    assert_eq!(colors.len(), 500);
    let mut buffer = Buffer::empty(Rect::new(0, 0, 25, 20));
    for (i, cell) in buffer.content.iter_mut().enumerate() {
        cell.set_symbol("▀").set_style(
            Style::new()
                .fg(colors[i])
                .bg(colors[(i + 37) % 500])
                .underline_color(colors[(i + 193) % 500])
                .add_modifier(ratatui::style::Modifier::UNDERLINED),
        );
    }
    for c in buffer.content {
        println!(
            "{} {} {} {} {}",
            c.symbol(),
            rgb(c.fg),
            rgb(c.bg),
            rgb(c.underline_color),
            c.modifier.contains(ratatui::style::Modifier::UNDERLINED)
        );
    }
}
