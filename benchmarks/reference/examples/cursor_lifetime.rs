use ratatui::Terminal;
use ratatui::backend::{Backend, ClearType, WindowSize};
use ratatui::buffer::Cell;
use ratatui::layout::{Position, Size};
use std::io;

#[derive(Default)]
struct Probe {
    trace: String,
    fail_visibility: bool,
    fail_flush: bool,
}

impl Drop for Probe {
    fn drop(&mut self) {
        println!("[{}]", self.trace);
    }
}

impl Probe {
    fn visibility(&mut self, visible: bool) -> io::Result<()> {
        let action = if visible { 's' } else { 'h' };
        if std::mem::take(&mut self.fail_visibility) {
            self.trace.push(action.to_ascii_uppercase());
            return Err(io::Error::other("visibility"));
        }
        self.trace.push(action);
        Ok(())
    }
}

impl Backend for Probe {
    type Error = io::Error;
    fn draw<'a, I: Iterator<Item = (u16, u16, &'a Cell)>>(&mut self, _: I) -> io::Result<()> {
        Ok(())
    }
    fn hide_cursor(&mut self) -> io::Result<()> {
        self.visibility(false)
    }
    fn show_cursor(&mut self) -> io::Result<()> {
        self.visibility(true)
    }
    fn get_cursor_position(&mut self) -> io::Result<Position> {
        Ok(Position::ORIGIN)
    }
    fn set_cursor_position<P: Into<Position>>(&mut self, _: P) -> io::Result<()> {
        Ok(())
    }
    fn clear(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn clear_region(&mut self, _: ClearType) -> io::Result<()> {
        Ok(())
    }
    fn size(&self) -> io::Result<Size> {
        Ok(Size::new(3, 2))
    }
    fn window_size(&mut self) -> io::Result<WindowSize> {
        Ok(WindowSize {
            columns_rows: self.size()?,
            pixels: Size::ZERO,
        })
    }
    fn flush(&mut self) -> io::Result<()> {
        if self.fail_flush {
            Err(io::Error::other("flush"))
        } else {
            Ok(())
        }
    }
    #[cfg(feature = "terminal-oracle")]
    fn scroll_region_up(&mut self, _: std::ops::Range<u16>, _: u16) -> io::Result<()> {
        Ok(())
    }
    #[cfg(feature = "terminal-oracle")]
    fn scroll_region_down(&mut self, _: std::ops::Range<u16>, _: u16) -> io::Result<()> {
        Ok(())
    }
}

fn take(_term: Terminal<Probe>) -> io::Result<()> {
    Err(io::Error::other("return early"))
}

fn exercise(n: u8) -> io::Result<()> {
    let mut t = Terminal::new(Probe::default())?;
    match n {
        0 => {}
        1 => t.hide_cursor()?,
        2 => {
            t.hide_cursor()?;
            t.show_cursor()?;
        }
        3 => {
            t.show_cursor()?;
            t.hide_cursor()?;
        }
        4 => t.backend_mut().hide_cursor()?,
        5 => {
            t.apply_buffer()?;
        }
        6 => {
            t.apply_buffer_with_cursor(Some((0, 0).into()))?;
        }
        7 => {
            t.apply_buffer()?;
            assert!(t.try_draw(|_| Err(io::Error::other("render"))).is_err());
        }
        8 => {
            t.apply_buffer()?;
            t.apply_buffer_with_cursor(Some((1, 1).into()))?;
        }
        9 => {
            t.backend_mut().fail_visibility = true;
            assert!(t.hide_cursor().is_err());
        }
        10 => {
            t.hide_cursor()?;
            t.backend_mut().fail_visibility = true;
            assert!(t.show_cursor().is_err());
        }
        11 => {
            t.show_cursor()?;
            t.backend_mut().fail_visibility = true;
            assert!(t.apply_buffer().is_err());
        }
        12 => {
            t.backend_mut().fail_flush = true;
            assert!(t.apply_buffer().is_err());
        }
        13 => {
            t.hide_cursor()?;
            t.backend_mut().fail_flush = true;
            assert!(t.apply_buffer_with_cursor(Some((0, 0).into())).is_err());
        }
        14 => {
            let _ = t.get_frame();
            t.flush()?;
            t.swap_buffers();
        }
        15 => {
            t.hide_cursor()?;
            assert!(take(t).is_err());
        }
        16 => {
            t.hide_cursor()?;
            t.backend_mut().fail_visibility = true;
        }
        17 => {
            t.hide_cursor()?;
            t = Terminal::new(Probe::default())?;
            t.show_cursor()?;
        }
        18 => {
            t.hide_cursor()?;
            let mut terminals = vec![t];
            assert!(take(terminals.pop().unwrap()).is_err());
        }
        19 => {
            t.hide_cursor()?;
            t.backend_mut().show_cursor()?;
        }
        20 => {
            t.apply_buffer()?;
            t.get_frame().set_cursor_position((2, 1));
            t.flush()?;
            t.swap_buffers();
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn main() -> io::Result<()> {
    for n in 0..21 {
        exercise(n)?;
    }
    Ok(())
}
