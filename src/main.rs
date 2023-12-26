use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Vector2,
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

fn main() {
    let mut playing: bool = false;

    let speed: f32 = 0.5;

    let mut up_pressed: bool = false;
    let mut down_pressed: bool = false;

    let screen_width: u32 = VideoMode::desktop_mode().width;
    let screen_height: u32 = VideoMode::desktop_mode().height;

    const PLAYER_WINDOW_WIDTH: u32 = 100;
    const PLAYER_WINDOW_HEIGHT: u32 = 300;

    const BALL_SIDE: u32 = 100;

    let ball_center_pos = Vector2::new(
        (screen_width / 2 - BALL_SIDE / 2) as i32,
        (screen_height / 2 - BALL_SIDE / 2) as i32,
    );

    let mut input: i8;
    let mut rw_y_pos: f32 = (screen_height / 2 - PLAYER_WINDOW_HEIGHT / 2) as f32 + 24.0;

    let mut ball_velocity: Vector2<f64> = Vector2::new(0.0, 0.0);
    let mut ball_pos: Vector2<f64>;

    let mut player_window = RenderWindow::new(
        (PLAYER_WINDOW_WIDTH, PLAYER_WINDOW_HEIGHT),
        "",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    player_window.set_position(Vector2::new(PLAYER_WINDOW_WIDTH as i32, rw_y_pos as i32));

    let mut ai_window = RenderWindow::new(
        (PLAYER_WINDOW_WIDTH, screen_height - 28),
        "",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    ai_window.set_position(Vector2::new(
        (screen_width - PLAYER_WINDOW_WIDTH - PLAYER_WINDOW_WIDTH) as i32,
        28,
    ));

    let mut ball_window = RenderWindow::new(
        (BALL_SIDE, BALL_SIDE),
        "",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    ball_pos = Vector2::new(ball_center_pos.x as f64, ball_center_pos.y as f64);
    ball_window.set_position(ball_center_pos);

    while player_window.is_open() {
        while let Some(ev) = player_window.poll_event() {
            match ev {
                Event::Closed => player_window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Up => {
                        up_pressed = true;
                    }
                    Key::Down => {
                        down_pressed = true;
                    }
                    Key::Escape => {
                        player_window.close();
                    }
                    Key::Space => {
                        if !playing {
                            println!("Starting the game!");
                            ball_velocity.x = 1.0;
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

        input = 0;

        if up_pressed {
            input -= 1;
        }
        if down_pressed {
            input += 1;
        }

        rw_y_pos += input as f32 * speed;
        rw_y_pos = rw_y_pos.clamp(28.0, (screen_height - PLAYER_WINDOW_HEIGHT) as f32);

        player_window.set_position(Vector2::new(PLAYER_WINDOW_WIDTH as i32, rw_y_pos as i32));
        player_window.clear(Color::WHITE);
        player_window.display();

        ai_window.clear(Color::WHITE);
        ai_window.display();

        if playing {
            ball_pos += ball_velocity;
            let new_ball_pos: Vector2<i32> = Vector2::new(ball_pos.x as i32, ball_pos.y as i32);

            ball_window.set_position(new_ball_pos);

            if check_collision(&ball_window, &ai_window) {
                ball_velocity.x *= -1.0;
            } else if check_collision(&ball_window, &player_window) {
                ball_velocity.x *= -1.0;

                if input != 0 {
                    println!("Spin from player!");
                    ball_velocity.y += input as f64 * speed as f64;
                    println!("New velocity: {:?}", ball_velocity);
                }
            }

            if new_ball_pos.y < 0 || new_ball_pos.y + BALL_SIDE as i32 > screen_height as i32 {
                println!("Bouncing! Vertical");
                ball_velocity.y *= -1.0;
            }

            if new_ball_pos.x < 0 || new_ball_pos.x + BALL_SIDE as i32 > screen_width as i32 {
                println!("Point!");
                playing = false;
                ball_velocity.x = 0.0;
                ball_velocity.y = 0.0;
                ball_pos = Vector2::new(ball_center_pos.x as f64, ball_center_pos.y as f64);
                ball_window.set_position(ball_center_pos);
            }
        }

        ball_window.clear(Color::WHITE);
        ball_window.display();

        player_window.request_focus();
    }
}

fn check_collision(window1: &RenderWindow, window2: &RenderWindow) -> bool {
    let window1_pos: Vector2<i32> = window1.position();
    let window2_pos: Vector2<i32> = window2.position();

    let window1_size: Vector2<u32> = window1.size();
    let window2_size: Vector2<u32> = window2.size();

    let mut colliding: bool = true;

    if window1_pos.x > window2_pos.x + window2_size.x as i32 {
        colliding = false;
    } else if window2_pos.x > window1_pos.x + window1_size.x as i32 {
        colliding = false;
    } else if window1_pos.y > window2_pos.y + window2_size.y as i32 {
        colliding = false;
    } else if window2_pos.y > window1_pos.y + window1_size.y as i32 {
        colliding = false;
    }

    return colliding;
}
