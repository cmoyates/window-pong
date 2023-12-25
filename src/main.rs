use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow, Text},
    system::Vector2,
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

fn main() {
    let speed = 0.25;

    let mut up_pressed = false;
    let mut down_pressed = false;

    let screen_height: u32 = VideoMode::desktop_mode().height;

    const PLAYER_WINDOW_WIDTH: u32 = 100;
    const PLAYER_WINDOW_HEIGHT: u32 = 300;

    let mut input: i8 = 0;
    let mut rw_y_pos = 24.0;

    let mut rw = RenderWindow::new(
        (PLAYER_WINDOW_WIDTH, PLAYER_WINDOW_HEIGHT),
        "",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    rw.set_position(Vector2::new(PLAYER_WINDOW_WIDTH as i32, rw_y_pos as i32));

    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Up => {
                        up_pressed = true;
                    }
                    Key::Down => {
                        down_pressed = true;
                    }
                    Key::Escape => {
                        rw.close();
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

        rw.set_position(Vector2::new(PLAYER_WINDOW_WIDTH as i32, rw_y_pos as i32));

        rw.clear(Color::BLACK);

        rw.display();
    }
}
