mod entity;

use std::time::{Duration, Instant};

use once_cell::sync::Lazy;
use sfml::{
    graphics::{Color, RenderTarget},
    system::Vector2,
    window::{Event, Key, VideoMode},
};

use entity::Entity;

static SCREEN_WIDTH: Lazy<u32> = Lazy::new(|| VideoMode::desktop_mode().width);
static SCREEN_HEIGHT: Lazy<u32> = Lazy::new(|| VideoMode::desktop_mode().height);

fn main() {
    let mut playing: bool = false;

    let speed: f32 = 10.0;

    let mut up_pressed: bool = false;
    let mut down_pressed: bool = false;

    const PLAYER_WINDOW_WIDTH: u32 = 100;
    const PLAYER_WINDOW_HEIGHT: u32 = 300;

    const BALL_SIDE: u32 = 100;

    let ball_center_pos: Vector2<f32> = Vector2::new(
        (*SCREEN_WIDTH / 2 - BALL_SIDE / 2) as f32,
        (*SCREEN_HEIGHT / 2 - BALL_SIDE / 2) as f32,
    );

    let mut input: i8;

    // Player setup

    let mut player: Entity = Entity::new(
        Vector2::new(
            (PLAYER_WINDOW_WIDTH + (PLAYER_WINDOW_WIDTH / 2)) as f32,
            (*SCREEN_HEIGHT / 2) as f32 - 24.0,
        ),
        PLAYER_WINDOW_WIDTH,
        PLAYER_WINDOW_HEIGHT,
        String::from("Player"),
    );

    // AI setup

    let mut ai: Entity = Entity::new(
        Vector2::new(
            (*SCREEN_WIDTH - PLAYER_WINDOW_WIDTH - (PLAYER_WINDOW_WIDTH / 2)) as f32,
            (*SCREEN_HEIGHT / 2) as f32 - 24.0,
        ),
        PLAYER_WINDOW_WIDTH,
        PLAYER_WINDOW_HEIGHT,
        String::from("AI"),
    );

    // Ball setup

    let mut ball: Entity = Entity::new(ball_center_pos, BALL_SIDE, BALL_SIDE, String::from("Ball"));

    // Game loop

    let mut last_update = Instant::now();
    let frame_duration = Duration::from_secs_f64(1.0 / 60.0); // 1/60th of a second

    while player.window.is_open() {
        // Event handling
        while let Some(evt) = player.window.poll_event() {
            match evt {
                Event::Closed => player.window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Up => {
                        up_pressed = true;
                    }
                    Key::Down => {
                        down_pressed = true;
                    }
                    Key::Escape => {
                        player.window.close();
                    }
                    Key::Space => {
                        if !playing {
                            println!("Starting the game!");
                            ball.velocity.x = 20.0;
                            playing = true;
                        }
                    }
                    _ => {}
                },
                Event::KeyReleased { code, .. } => match code {
                    Key::Up => {
                        up_pressed = false;
                    }
                    Key::Down => {
                        down_pressed = false;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Input handling

        input = 0;

        if up_pressed {
            input -= 1;
        }
        if down_pressed {
            input += 1;
        }

        // Player Logic

        player.velocity.y = input as f32 * speed;

        Entity::r#move(&mut player);

        let clamped_player_pos_y: f32 = player.position.y.clamp(
            (48 + player.half_size.y) as f32,
            (*SCREEN_HEIGHT - player.half_size.y) as f32,
        );

        player.set_position(None, Some(clamped_player_pos_y));

        player.window.clear(Color::WHITE);
        player.window.display();

        // AI Logic

        ai.window.clear(Color::WHITE);
        ai.window.display();

        // Ball Logic

        if playing {
            ball.r#move();

            let players = [&player, &ai];

            // Player collision

            for &player in &players {
                let ball_player_overlap: Vector2<i32> = Entity::get_overlap(&ball, player);
                if ball_player_overlap.x > 0 && ball_player_overlap.y > 0 {
                    println!("Ball collided with {0}!", player.name);
                    let prev_overlap: Vector2<i32> = Entity::get_prev_overlap(&ball, player);
                    let adjustment_sign: Vector2<f32> = Vector2::new(
                        if player.position.x > ball.position.x {
                            1.0
                        } else {
                            -1.0
                        },
                        if player.position.y < ball.position.y {
                            1.0
                        } else {
                            -1.0
                        },
                    );
                    if prev_overlap.y > 0 {
                        ball.velocity.x *= -1.0;

                        let adjustment =
                            ball.position.x - (ball_player_overlap.x as f32 * adjustment_sign.x);
                        ball.set_position(Some(adjustment), None);
                    } else if prev_overlap.x > 0 {
                        ball.velocity.y *= -1.0;
                        let adjustment =
                            ball.position.y + (ball_player_overlap.y as f32 * adjustment_sign.y);
                        ball.set_position(None, Some(adjustment));
                    } else {
                        if ball_player_overlap.y >= ball_player_overlap.x {
                            ball.velocity.x *= -1.0;
                            let adjustment = ball.position.x
                                - (ball_player_overlap.x as f32 * adjustment_sign.x);
                            ball.set_position(Some(adjustment), None);
                        } else {
                            ball.velocity.y *= -1.0;
                            let adjustment = ball.position.y
                                - (ball_player_overlap.y as f32 * adjustment_sign.y);
                            ball.set_position(None, Some(adjustment));
                        }
                    }

                    ball.velocity += player.velocity;
                }
            }

            if (ball.position.y - ball.half_size.y as f32) < 48.0
                || ball.position.y + ball.half_size.y as f32 > *SCREEN_HEIGHT as f32
            {
                println!("Ball collided with the edge of the screen!");
                ball.velocity.y *= -1.0;
                let clamped_ball_pos_y: f32 = ball.position.y.clamp(
                    (48 + ball.half_size.y) as f32,
                    (*SCREEN_HEIGHT - ball.half_size.y) as f32,
                );
                ball.set_position(None, Some(clamped_ball_pos_y));
            }

            if ball.position.x < 0.0 || ball.position.x + BALL_SIDE as f32 > *SCREEN_WIDTH as f32 {
                println!("Point!");
                playing = false;
                ball.velocity.x = 0.0;
                ball.velocity.y = 0.0;
                ball.set_position(Some(ball_center_pos.x), Some(ball_center_pos.y));
            }
        }

        ball.window.clear(Color::WHITE);
        ball.window.display();

        // Focus on player window
        player.window.request_focus();

        // Calculate how long to sleep
        if let Some(sleep_duration) =
            frame_duration.checked_sub(Instant::now().duration_since(last_update))
        {
            std::thread::sleep(sleep_duration);
        }

        // Update the time of the last update
        last_update = Instant::now();
    }
}
