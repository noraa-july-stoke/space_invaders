use piston_window::*;

mod game;
mod drawing;
use game::Game;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Space Shooter", [WIDTH, HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let (width, height) = (640.0, 480.0);
    let mut game = Game::new(width, height);

    let mut events = window.events;
    while let Some(event) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        if let Some(args) = event.update_args() {
            game.update(args.dt);
        }

        if let Some(_args) = event.render_args() {
            window.draw_2d(&event, |c, g, _| game.draw(&c, g));
        }
    }
}
