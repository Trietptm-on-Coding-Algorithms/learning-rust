use std::fmt;
use std::fmt::{Display, Formatter};
use std::{thread, time};

const ESC: &str = "\x1B";

fn clear() {
    print!("\x1B[H\x1B[2J\x1B[3J");
}

fn cursor_move_right(fmt: &mut Formatter, n: u32) -> fmt::Result {
    write!(fmt, "{}[{}C", ESC, n)
}

fn cursor_move_down(fmt: &mut Formatter, n: u32) -> fmt::Result {
    write!(fmt, "{}[{}B", ESC, n)
}

fn cursor_move(fmt: &mut Formatter, x: u32, y: u32) -> fmt::Result {
    write!(fmt, "{}[{};{}f", ESC, y, x)
}

#[derive(Debug)]
enum VDirection {
    Up,
    Down,
}

#[derive(Debug)]
enum HDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct Ball {
    x: u32,
    y: u32,
    v_direction: VDirection,
    h_direction: HDirection,
}

#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        let frame = Frame {
            width: 60,
            height: 15,
        };
        let ball = Ball {
            x: frame.width / 2,
            y: frame.height / 2,
            v_direction: VDirection::Up,
            h_direction: HDirection::Left,
        };
        Game { frame, ball }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let write_row = |fmt: &mut Formatter| {
            write!(fmt, "+")?;
            for _ in 0..self.frame.width {
                write!(fmt, "-")?;
            }
            writeln!(fmt, "+")
        };

        write_row(fmt)?;
        for y in (0..self.frame.height).rev() {
            write!(fmt, "|")?;
            for x in 0..self.frame.width {
                let c = if self.ball.x == x && self.ball.y == y {
                    "o"
                } else {
                    " "
                };
                write!(fmt, "{}", c)?;
            }
            writeln!(fmt, "|")?;
        }
        write_row(fmt)
    }
}

impl Display for Frame {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let write_row = |fmt: &mut Formatter| {
            write!(fmt, "+")?;
            for _ in 0..self.width {
                write!(fmt, "-")?;
            }
            writeln!(fmt, "+")
        };

        write_row(fmt)?;
        for y in (0..self.height).rev() {
            cursor_move(fmt, 1, y + 2)?;
            write!(fmt, "|")?;
            cursor_move(fmt, self.width + 2, y + 2)?;
            write!(fmt, "|")?;
        }
        cursor_move(fmt, 1, self.height + 2)?;
        write_row(fmt)
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        // Switch horizontal direction. i.e. x.
        if self.x == 0 {
            self.h_direction = HDirection::Right;
        } else if self.x == frame.width - 1 {
            self.h_direction = HDirection::Left;
        }
        // Switch veritical direction. i.e. y.
        if self.y == 0 {
            self.v_direction = VDirection::Up;
        } else if self.y == frame.height - 1 {
            self.v_direction = VDirection::Down;
        }
    }

    fn mv(&mut self) {
        // Move in horizontal direction.
        match self.h_direction {
            HDirection::Left => self.x -= 1,
            HDirection::Right => self.x += 1,
        }
        // Move in vertical direction.
        match self.v_direction {
            VDirection::Down => self.y -= 1,
            VDirection::Up => self.y += 1,
        }
    }
}

fn main() {
    let mut game = Game::new();
    let do_game = false;
    if do_game {
        println!("Game initial => {:#?}", game);
        for _ in 1..100 {
            clear();
            println!("{}", game);
            game.step();
            thread::sleep(time::Duration::from_millis(33));
        }
        println!("Game final => {:#?}", game);
    } else {
        clear();
        print!("{}", game.frame);
    }
}
