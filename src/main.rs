mod entity;

use std::time::{Duration, Instant};

use once_cell::sync::Lazy;
use sfml::{
    graphics::{Color, RenderTarget},
    system::{Vector2, Vector2f},
    window::{Event, Key, VideoMode},
};

use entity::Entity;

static SCREEN_WIDTH: Lazy<u32> = Lazy::new(|| VideoMode::desktop_mode().width);
static SCREEN_HEIGHT: Lazy<u32> = Lazy::new(|| VideoMode::desktop_mode().height);

fn main() {
    let mut playing: bool = false;

    let max_player_speed: f32 = 15.0;
    let max_ball_speed: f32 = 20.0;

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

    let mut shoot_timer: u8 = 0;
    const SHOOT_TIMER_MAX: u8 = 10;
    let mut shoot_buffer: u8 = 0;
    const SHOOT_BUFFER_MAX: u8 = 10;

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
                            ball.velocity.x = max_ball_speed;
                            playing = true;
                        } else {
                            if shoot_timer > 0 {
                                shoot_timer = 0;
                                ball.velocity.x = max_ball_speed * ball.velocity.x.signum();
                                ball.velocity.y = 0.0;
                            } else {
                                shoot_buffer = SHOOT_BUFFER_MAX;
                            }
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

        if input == 0 {
            // Decelerating
            let velocity_sign = player.velocity.y.signum();
            player.acceleration.y = -1.0 * velocity_sign;
            player.velocity.y += player.acceleration.y;
            player.velocity.y =
                player.velocity.y.abs().clamp(0.0, max_player_speed) * velocity_sign;
            if player.velocity.y.abs() < 2.5 {
                player.velocity.y = 0.0;
            }
        } else {
            // Accelerating
            player.acceleration.y = input as f32 * 1.0;
            player.velocity.y += player.acceleration.y;
        }

        Entity::r#move(&mut player);

        if player.position.y < 48.0 + player.half_size.y as f32
            || player.position.y > *SCREEN_HEIGHT as f32 - player.half_size.y as f32
        {
            player.velocity.y = 0.0;

            let clamped_player_pos_y: f32 = player.position.y.clamp(
                (48 + player.half_size.y) as f32,
                (*SCREEN_HEIGHT - player.half_size.y) as f32,
            );

            player.set_position(None, Some(clamped_player_pos_y));
        }

        player.window.clear(Color::WHITE);
        player.window.display();

        // AI Logic

        if ball.velocity.x > 0.0 {
            let ball_overlap = Entity::get_overlap(&ai, &ball);
            if ball_overlap.y <= 0 {
                if ball.position.y < ai.position.y {
                    ai.acceleration.y = -1.0 * 1.0;
                } else {
                    ai.acceleration.y = 1.0;
                }
                ai.velocity.y += ai.acceleration.y;
            } else {
                let velocity_sign = ai.velocity.y.signum();
                ai.acceleration.y = -1.0 * velocity_sign;
                ai.velocity.y += ai.acceleration.y;
                ai.velocity.y = ai.velocity.y.abs().clamp(0.0, max_player_speed) * velocity_sign;
                if ai.velocity.y.abs() < 2.5 {
                    ai.velocity.y = 0.0;
                }
            }
        }

        if ai.position.y < 48.0 + ai.half_size.y as f32
            || ai.position.y > *SCREEN_HEIGHT as f32 - ai.half_size.y as f32
        {
            ai.velocity.y = 0.0;

            let clamped_ai_pos_y: f32 = ai.position.y.clamp(
                (48 + ai.half_size.y) as f32,
                (*SCREEN_HEIGHT - ai.half_size.y) as f32,
            );

            ai.set_position(None, Some(clamped_ai_pos_y));
        }

        ai.r#move();

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
                    ball.velocity = normalize_vector(ball.velocity) * max_ball_speed;

                    if shoot_buffer > 0 {
                        shoot_buffer = 0;
                        ball.velocity.x = max_ball_speed * ball.velocity.x.signum();
                        ball.velocity.y = 0.0;
                    } else {
                        shoot_timer = SHOOT_TIMER_MAX;
                    }
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

                ai.velocity.y = 0.0;
                ai.set_position(None, Some((*SCREEN_HEIGHT / 2) as f32 - 24.0));
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

        if shoot_buffer > 0 {
            shoot_buffer -= 1;
            println!("Shoot buffer: {}", shoot_buffer);
        }
        if shoot_timer > 0 {
            shoot_timer -= 1;
            println!("Shoot timer: {}", shoot_timer);
        }
    }
}

fn normalize_vector(vector: Vector2f) -> Vector2f {
    let length = (vector.x.powi(2) + vector.y.powi(2)).sqrt();
    if length != 0.0 {
        Vector2f::new(vector.x / length, vector.y / length)
    } else {
        vector
    }
}
