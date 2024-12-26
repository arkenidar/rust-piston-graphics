extern crate piston_window;

use piston_window::*;

#[derive(Clone)]
struct Square {
    pos: [f64; 2],
    size: [f64; 2],
    selected: bool,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Draggable Squares", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut cursor_pos = [0.0, 0.0];
    let mut squares: Vec<Square> = Vec::new();
    let mut dragging: Option<usize> = None;
    let square_size = [50.0, 50.0];

    while let Some(event) = window.next() {
        if let Some(pos) = event.mouse_cursor_args() {
            cursor_pos = pos;
            if let Some(idx) = dragging {
                squares[idx].pos = pos;
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let clicked_square = squares.iter().position(|square| {
                cursor_pos[0] >= square.pos[0]
                    && cursor_pos[0] <= square.pos[0] + square.size[0]
                    && cursor_pos[1] >= square.pos[1]
                    && cursor_pos[1] <= square.pos[1] + square.size[1]
            });

            if let Some(idx) = clicked_square {
                dragging = Some(idx);
                squares[idx].selected = true;
            } else {
                squares.push(Square {
                    pos: [
                        cursor_pos[0] - square_size[0] / 2.0,
                        cursor_pos[1] - square_size[1] / 2.0,
                    ],
                    size: square_size,
                    selected: false,
                });
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            if let Some(idx) = dragging {
                squares[idx].selected = false;
            }
            dragging = None;
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);

            for square in &mut squares {
                let color = if square.selected {
                    [1.0, 0.0, 0.0, 1.0] // Red when selected
                } else {
                    [0.0, 0.0, 1.0, 1.0] // Blue when not selected
                };

                if square.selected {
                    square.pos[0] = cursor_pos[0] - square.size[0] / 2.0;
                    square.pos[1] = cursor_pos[1] - square.size[1] / 2.0;
                }

                rectangle(
                    color,
                    [square.pos[0], square.pos[1], square.size[0], square.size[1]],
                    c.transform,
                    g,
                );
            }
        });
    }
}
