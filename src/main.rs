use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Vector2,
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

fn main() {
    let mut playing = false;

    let speed = 0.5;

    let mut up_pressed = false;
    let mut down_pressed = false;

    let screen_width: u32 = VideoMode::desktop_mode().width;
    let screen_height: u32 = VideoMode::desktop_mode().height;

    const PLAYER_WINDOW_WIDTH: u32 = 100;
    const PLAYER_WINDOW_HEIGHT: u32 = 300;

    const BALL_SIDE: u32 = 50;

    let mut input: i8;
    let mut rw_y_pos = 24.0;

    let mut ball_velocity = Vector2::new(0.0, 0.0);

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
        rw_y_pos as i32,
    ));

    let mut ball_window = RenderWindow::new(
        (BALL_SIDE, BALL_SIDE),
        "",
        Style::CLOSE,
        &ContextSettings::default(),
    );

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
        rw_y_pos = rw_y_pos.clamp(24.0, (screen_height - PLAYER_WINDOW_HEIGHT) as f32);

        player_window.set_position(Vector2::new(PLAYER_WINDOW_WIDTH as i32, rw_y_pos as i32));
        player_window.clear(Color::WHITE);
        player_window.display();

        ai_window.clear(Color::WHITE);
        ai_window.display();

        if playing {
            let new_ball_pos: Vector2<i32> = Vector2::new(
                ball_window.position().x + ball_velocity.x as i32,
                ball_window.position().y + ball_velocity.y as i32,
            );

            ball_window.set_position(new_ball_pos);

            if check_collision(&ball_window, &player_window)
                || check_collision(&ball_window, &ai_window)
            {
                ball_velocity.x *= -1.0;
            }
        }

        ball_window.clear(Color::WHITE);
        ball_window.display();

        player_window.request_focus();
    }
}

fn check_collision(window1: &RenderWindow, window2: &RenderWindow) -> bool {
    let window1_pos = window1.position();
    let window2_pos = window2.position();

    let window1_size = window1.size();
    let window2_size = window2.size();

    let mut colliding = true;

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
