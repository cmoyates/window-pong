mod entity;

use std::time::{Duration, Instant};

use once_cell::sync::Lazy;
use sfml::{
    graphics::{Color, Font, RenderTarget, Text, Transformable},
    system::{Vector2, Vector2f},
    window::{Event, Key, VideoMode},
};

use entity::Entity;

static SCREEN_WIDTH: Lazy<u32> = Lazy::new(|| VideoMode::desktop_mode().width);
static SCREEN_HEIGHT: Lazy<u32> = Lazy::new(|| VideoMode::desktop_mode().height);

fn main() {
    let mut playing: bool = false;

    const MAX_PLAYER_SPEED: f32 = 15.0;
    let mut max_ball_speed: f32 = 15.0;
    const INIT_BALL_SPEED: f32 = 20.0;

    let mut up_pressed: bool = false;
    let mut down_pressed: bool = false;

    const PLAYER_WINDOW_WIDTH: u32 = 75;
    const PLAYER_WINDOW_HEIGHT: u32 = 300;

    const BALL_SIDE: u32 = 100;

    let ball_center_pos: Vector2<f32> =
        Vector2::new((*SCREEN_WIDTH / 2) as f32, (*SCREEN_HEIGHT / 2) as f32);

    let mut input: i8;

    let mut shoot_timer: u8 = 0;
    const MAX_SHOOT_TIMER: u8 = 10;
    let mut shoot_buffer: u8 = 0;
    const MAX_SHOOT_BUFFER: u8 = 10;

    let mut score: (u8, u8) = (0, 0);

    const IMPACT_SCALE: f32 = 2.0;

    // Player setup

    let mut player: Entity = Entity::new(
        Vector2::new(
            (PLAYER_WINDOW_WIDTH + (PLAYER_WINDOW_WIDTH * 1)) as f32,
            (*SCREEN_HEIGHT / 2) as f32,
        ),
        PLAYER_WINDOW_WIDTH,
        PLAYER_WINDOW_HEIGHT,
        String::from("Player"),
        Color::BLUE,
    );

    // AI setup

    let mut ai: Entity = Entity::new(
        Vector2::new(
            (*SCREEN_WIDTH - PLAYER_WINDOW_WIDTH - (PLAYER_WINDOW_WIDTH * 1)) as f32,
            (*SCREEN_HEIGHT / 2) as f32,
        ),
        PLAYER_WINDOW_WIDTH,
        PLAYER_WINDOW_HEIGHT,
        String::from("AI"),
        Color::RED,
    );

    // Ball setup

    let mut ball: Entity = Entity::new(
        ball_center_pos,
        BALL_SIDE,
        BALL_SIDE,
        String::from("Ball"),
        Color::WHITE,
    );

    // Score window setup

    let mut score_window = Entity::new(
        Vector2::new((*SCREEN_WIDTH / 2) as f32, 150.0),
        250,
        100,
        String::from("Score"),
        Color::WHITE,
    );

    let font = Font::from_file("assets/Roboto-Regular.ttf").unwrap();
    let score_string = format!("{} - {}", score.0, score.1);
    let mut score_text = Text::new(&score_string, &font, 75);
    score_text.set_fill_color(Color::BLACK);
    let text_rect = score_text.local_bounds();
    score_text.set_origin(Vector2::new(text_rect.width / 2.0, text_rect.height / 1.2));
    score_text.set_position(Vector2::new(
        score_window.half_size.x as f32,
        score_window.half_size.y as f32,
    ));

    // Game loop

    let mut last_update = Instant::now();
    let frame_duration = Duration::from_secs_f64(1.0 / 60.0); // 1/60th of a second
    let mut delay_multiplier: u32 = 1;

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
                            ball.velocity.x = INIT_BALL_SPEED;
                            playing = true;
                        } else {
                            if shoot_timer > 0 {
                                shoot_timer = 0;
                                ball.velocity.x = max_ball_speed * ball.velocity.x.signum();
                                ball.velocity.y = 0.0;
                                ball.color = Color::YELLOW;
                                delay_multiplier = 5;
                                player.color = Color::YELLOW;
                            } else {
                                shoot_buffer = MAX_SHOOT_BUFFER;
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
                player.velocity.y.abs().clamp(0.0, MAX_PLAYER_SPEED) * velocity_sign;
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

        // AI Logic

        if ball.velocity.x > 0.0 {
            let ball_overlap = Entity::get_overlap(&ai, &ball);
            if ball_overlap.y <= ball.size.y as i32 {
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
                ai.velocity.y = ai.velocity.y.abs().clamp(0.0, MAX_PLAYER_SPEED) * velocity_sign;
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

        // Ball Logic

        if playing {
            ball.r#move();

            let other_entities = [&mut player, &mut ai, &mut score_window];

            // Ball collision

            for entity in other_entities {
                let ball_player_overlap: Vector2<i32> = Entity::get_overlap(&ball, entity);
                if ball_player_overlap.x > 0 && ball_player_overlap.y > 0 {
                    println!("Ball collided with {0}!", entity.name);

                    let prev_overlap: Vector2<i32> = Entity::get_prev_overlap(&ball, entity);
                    let adjustment_sign: Vector2<f32> = Vector2::new(
                        if entity.position.x > ball.position.x {
                            1.0
                        } else {
                            -1.0
                        },
                        if entity.position.y < ball.position.y {
                            1.0
                        } else {
                            -1.0
                        },
                    );
                    if prev_overlap.y > 0 {
                        ball.velocity.x *= -1.0;
                        let adjustment: f32 =
                            ball.position.x - (ball_player_overlap.x as f32 * adjustment_sign.x);
                        ball.set_position(Some(adjustment), None);
                    } else if prev_overlap.x > 0 {
                        ball.velocity.y *= -1.0;
                        let adjustment: f32 =
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

                    ball.velocity += entity.velocity;
                    ball.velocity = normalize_vector(ball.velocity) * max_ball_speed;

                    if entity.name == "Player" {
                        if shoot_buffer > 0 {
                            shoot_buffer = 0;
                            ball.velocity.x = max_ball_speed * ball.velocity.x.signum();
                            ball.velocity.y = 0.0;
                            ball.color = Color::YELLOW;

                            delay_multiplier = 5;
                        } else {
                            shoot_timer = MAX_SHOOT_TIMER;
                            ball.color = Color::WHITE;

                            delay_multiplier = 3;
                        }
                    } else {
                        println!("AI shot the ball!");
                        ball.color = Color::WHITE;
                        delay_multiplier = 3;
                    }
                    entity.color = ball.color;

                    if entity.name == "Score" && entity.velocity.length_sq() == 0.0 {
                        entity.acceleration = ball.velocity;
                        entity.set_scale(IMPACT_SCALE);
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

            if (ball.position.x - ball.half_size.x as f32) < 0.0
                || ball.position.x + ball.half_size.x as f32 > *SCREEN_WIDTH as f32
            {
                println!("Point!");

                if (ball.position.x - ball.half_size.x as f32) < 0.0 {
                    score.1 += 1;
                } else {
                    score.0 += 1;
                }
                let score_string = format!("{} - {}", score.0, score.1);
                score_text.set_string(&score_string);
                let text_rect = score_text.local_bounds();
                score_text.set_origin(Vector2::new(text_rect.width / 2.0, text_rect.height / 1.2));

                playing = false;
                ball.velocity.x = 0.0;
                ball.velocity.y = 0.0;
                ball.set_position(Some(ball_center_pos.x), Some(ball_center_pos.y));
                max_ball_speed = INIT_BALL_SPEED;
                ball.color = Color::WHITE;

                player.set_scale(1.0);
                player.set_position(None, Some((*SCREEN_HEIGHT / 2) as f32 - 24.0));

                ai.velocity.y = 0.0;
                ai.set_position(None, Some((*SCREEN_HEIGHT / 2) as f32 - 24.0));
                ai.set_scale(1.0);

                delay_multiplier = 30;
            }
        }

        ball.window.clear(ball.color);
        ball.window.display();

        // Display player and AI with appropriate color

        player.window.clear(player.color);
        player.window.display();

        println!(
            "Player window size: {0}x{1}",
            player.window.size().x,
            player.window.size().y
        );

        ai.window.clear(ai.color);
        ai.window.display();

        println!(
            "AI window size: {0}x{1}",
            ai.window.size().x,
            ai.window.size().y
        );

        // Score window logic

        if (score_window.velocity.x > 0.0) == (score_window.acceleration.x > 0.0) {
            score_window.acceleration *= 0.5;
        }
        score_window.velocity += score_window.acceleration;
        score_window.r#move();
        if (score_window.init_position - score_window.position).length_sq() < 2.0
            && score_window.velocity.length_sq() < 0.1
        {
            score_window.velocity.x = 0.0;
            score_window.velocity.y = 0.0;
            score_window.position = score_window.init_position;
        }
        score_window.acceleration = score_window.init_position - score_window.position;

        score_window.window.clear(score_window.color);

        score_window.window.draw(&mut score_text);

        score_window.window.display();

        score_window.set_scale(1.0);

        // score_window.set_scale(1.0);

        // Focus on player window
        player.window.request_focus();

        // Timers
        if shoot_buffer > 0 {
            shoot_buffer -= 1;
        }
        if shoot_timer > 0 {
            shoot_timer -= 1;
        }

        if playing {
            max_ball_speed += 0.005;
            player.set_scale_xy(
                None,
                Some((player.scale.y - 0.0001).clamp(0.25, IMPACT_SCALE)),
            );
            ai.set_scale_xy(
                None,
                Some((player.scale.y - 0.0001).clamp(0.25, IMPACT_SCALE)),
            );
        }

        // Wait for next frame
        if let Some(sleep_duration) = (frame_duration * delay_multiplier)
            .checked_sub(Instant::now().duration_since(last_update))
        {
            std::thread::sleep(sleep_duration);
            if delay_multiplier > 1 {
                delay_multiplier = 1;

                if player.color != Color::BLUE {
                    player.color = Color::BLUE;
                }

                if ai.color != Color::RED {
                    ai.color = Color::RED;
                }

                score_window.color = Color::WHITE;
            }
        }
        last_update = Instant::now();
    }
}

/// Normalizes a vector
fn normalize_vector(vector: Vector2f) -> Vector2f {
    let length = (vector.x.powi(2) + vector.y.powi(2)).sqrt();
    if length != 0.0 {
        Vector2f::new(vector.x / length, vector.y / length)
    } else {
        vector
    }
}
