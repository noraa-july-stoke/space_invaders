#![allow(dead_code)]
use piston_window::*;
use rand::random;

const PLAYER_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const ENEMY_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BULLET_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const PLAYER_SPEED: f64 = 220.0; // in pixels per second
const BULLET_SPEED: f64 = 300.0; // in pixels per second
const ENEMY_SPEED: f64 = 50.0; // in pixels per second
const PLAYER_SPEED_BOOSTED: f64 = 300.0; // in pixels per second
const BOOST_TIME: f64 = 30.0; // in seconds
const POWERUP_SPEED: f64 = 60.0; // in pixels per second
const POWERUP_SPAWN_TIME: f64 = 20.0; // in seconds

const MAX_ENEMIES_ON_GROUND: usize = 5;

#[derive(Clone, PartialEq)]
struct Entity {
    x: f64,
    y: f64,
}

pub struct Game {
    player: Entity,
    bullets: Vec<Entity>,
    enemies: Vec<Entity>,
    enemy_spawn_timer: f64,
    window_width: f64,
    window_height: f64,
    key_state: Option<Key>,
}

impl Game {
    pub fn new(window_width: f64, window_height: f64) -> Game {
        Game {
            player: Entity {
                x: window_width / 2.0,
                y: window_height - 20.0,
            },
            bullets: Vec::new(),
            enemies: Vec::new(),
            enemy_spawn_timer: 0.0,
            window_width,
            window_height,
            key_state: None,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        self.key_state = Some(key);
        if let Some(Key::Space) = self.key_state {
            self.bullets.push(Entity {
                x: self.player.x,
                y: self.player.y - 20.0,
            });
        }
    }

    pub fn key_released(&mut self, _key: Key) {
        self.key_state = None;
    }

    pub fn update(&mut self, dt: f64) {
        match self.key_state {
            Some(Key::Left) => self.player.x -= PLAYER_SPEED * dt,
            Some(Key::Right) => self.player.x += PLAYER_SPEED * dt,
            _ => (),
        }

        // spawn enemies
        self.enemy_spawn_timer += dt;
        if self.enemy_spawn_timer > 1.3 {
            self.enemies.push(Entity {
                x: random::<f64>() * self.window_width,
                y: 0.0,
            });
            self.enemy_spawn_timer = 0.0;
        }

        // update bullets
        for bullet in &mut self.bullets {
            bullet.y -= BULLET_SPEED * dt;
        }

        // update enemies
        for enemy in &mut self.enemies {
            enemy.y += ENEMY_SPEED * dt;
        }

        // check bullet-enemy collisions
        let mut collision_indices = Vec::new();
        for (b_index, bullet) in self.bullets.iter().enumerate() {
            for (e_index, enemy) in self.enemies.iter().enumerate() {
                let dx = bullet.x - enemy.x;
                let dy = bullet.y - enemy.y;
                if (dx * dx + dy * dy).sqrt() < 10.0 {
                    collision_indices.push((b_index, e_index));
                }
            }
        }

        // Sort indices in reverse order for safe removal
        collision_indices.sort_by(|a, b| b.cmp(a));

        // Remove collided bullets and enemies
        for (b_index, e_index) in collision_indices {
            self.bullets.remove(b_index);
            self.enemies.remove(e_index);
        }

        // check if too many enemies on ground
        let enemies_on_ground = self
            .enemies
            .iter()
            .filter(|e| e.y >= self.window_height - 20.0)
            .count();

        if enemies_on_ground >= MAX_ENEMIES_ON_GROUND {
            self.reset();
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.y -= BULLET_SPEED * dt;
        }

        // Update enemies
        for enemy in &mut self.enemies {
            enemy.y += ENEMY_SPEED * dt;
        }

        // Remove off-screen bullets
        self.bullets.retain(|bullet| bullet.y > 0.0);

        // Remove off-screen enemies
        self.enemies.retain(|enemy| enemy.y < self.window_height);
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        // clear the screen
        clear([0.0, 0.0, 0.0, 1.0], g);

        // draw player
        let (player_x, player_y) = (self.player.x, self.player.y);
        rectangle(
            PLAYER_COLOR,
            [player_x - 20.0, player_y - 10.0, 20.0, 20.0],
            c.transform,
            g,
        );
        rectangle(
            PLAYER_COLOR,
            [player_x, player_y - 10.0, 20.0, 20.0],
            c.transform,
            g,
        );

        // draw bullets
        for bullet in &self.bullets {
            let (bullet_x, bullet_y) = (bullet.x, bullet.y);
            rectangle(
                BULLET_COLOR,
                [bullet_x - 2.0, bullet_y - 10.0, 4.0, 20.0],
                c.transform,
                g,
            );
        }

        // draw enemies
        for enemy in &self.enemies {
            let (enemy_x, enemy_y) = (enemy.x, enemy.y);
            rectangle(
                ENEMY_COLOR,
                [enemy_x - 10.0, enemy_y - 10.0, 20.0, 20.0],
                c.transform,
                g,
            );
        }
    }
    pub fn reset(&mut self) {
        self.player = Entity {
            x: self.window_width / 2.0,
            y: self.window_height - 20.0,
        };
        self.bullets.clear();
        self.enemies.clear();
        self.enemy_spawn_timer = 0.0;
    }
}

pub fn main() {
    let (width, height) = (640, 480);
    let mut window: PistonWindow = WindowSettings::new("Space Invaders", (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width as f64, height as f64);

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            game.key_released(key);
        }

        if let Some(update_args) = e.update_args() {
            game.update(update_args.dt);
        }

        window.draw_2d(&e, |c, g, _| {
            game.draw(&c, g);
        });
    }
}
