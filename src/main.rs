// Tetris
// render display vector function
// update display vector function

// TODO:
// - add score
// - add level
// - add rotation

mod tetrominoe;
mod tetris_lib;

use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use tetris_lib::{full_line, gravity, handle_input, init, new_piece, render};

fn main() {
    let mut stdin = termion::async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    const WIDTH: i32 = 10;
    const HEIGHT: i32 = 20;

    let mut display: Vec<Vec<char>> = init(WIDTH, HEIGHT);

    print!("{}", termion::cursor::Hide);
    new_piece(&mut display);

    let mut counter: usize = 0;

    // main loop
    loop {
        let prev_display = display.clone();
        // handle input
        let key = if let Some(Ok(key)) = stdin.next() {
            match key {
                Key::Char('q') => 'q', // quit
                Key::Left => 'l',      // left
                Key::Right => 'r',     // right
                Key::Char(' ') => 's', // down with spacebar
                Key::Down => 'd',      // down
                _ => ' ',
            }
        } else {
            ' '
        };

        // quit
        if key == 'q' {
            break;
        }

        // gravity
        if counter == 3 {
            if gravity(&mut display) {
                break;
            }
            counter = 0;
        }

        // handle input
        handle_input(&mut display, key);

        // full line
        full_line(&mut display);

        // check if display was changed
        let is_updated = display != prev_display;

        // render
        render(&display, is_updated);
        sleep(Duration::from_millis(50));
        stdout.flush().unwrap();
        counter += 1;
    }

    print!("{}\r\n\r\n\r\n", termion::cursor::Show);
}