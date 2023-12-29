use rand::Rng;
use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2,
    window::{ContextSettings, Style},
};

use crate::utils::{interpolate_angle, normalize_vector};

pub struct Entity<'a> {
    // Window
    pub window: RenderWindow,

    // Position
    pub position: Vector2<f32>,
    pub init_position: Vector2<f32>,
    prev_position: Vector2<f32>,

    // Movement
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,

    // Size
    pub size: Vector2<u32>,
    init_size: Vector2<u32>,
    pub half_size: Vector2<u32>,
    // Scale
    pub scale: Vector2<f32>,

    // Name
    pub name: String,

    // Body
    pub color: Color,
    // Eye
    _eye_white: CircleShape<'a>,
    _eye_pupil: CircleShape<'a>,
    _look_score_timer: i32,
    _max_look_score_timer: i32,
    _look_score_countdown: i32,
    _max_look_score_countdown: i32,
    _min_look_score_countdown: i32,
    _following_target: bool,
    _blink_timer: i32,
    _max_blink_timer: i32,
    _blink_countdown: i32,
    _max_blink_countdown: i32,
    _min_blink_countdown: i32,

    // Impact
    _offset: Vector2<f32>,
    _offset_velocity: Vector2<f32>,
    _offset_acceleration: Vector2<f32>,
    _spring_stiffness: f32,
    _spring_damping: f32,
}

impl Entity<'_> {
    pub fn new(
        position: Vector2<f32>,
        width: u32,
        height: u32,
        name: String,
        color: Color,
        spring_stiffness: f32,
        spring_damping: f32,
    ) -> Entity<'static> {
        let mut window = RenderWindow::new(
            (width, height),
            "",
            Style::CLOSE,
            &ContextSettings::default(),
        );

        let size: Vector2<u32> = Vector2::new(width, height);
        let half_size: Vector2<u32> = Vector2::new(width / 2, height / 2);

        window.set_position(Vector2::new(
            position.x as i32 - half_size.x as i32,
            position.y as i32 - half_size.y as i32,
        ));

        // Eye

        // Eye white
        let mut eye_white = CircleShape::new(20.0, 30);
        eye_white.set_fill_color(Color::WHITE);
        eye_white.set_origin(Vector2::new(20.0, 20.0));
        eye_white.set_position(Vector2::new(half_size.x as f32, 35.0));

        // Eye pupil
        let mut eye_pupil = CircleShape::new(7.5, 30);
        eye_pupil.set_fill_color(Color::BLACK);
        eye_pupil.set_origin(Vector2::new(-2.5, 7.5));
        eye_pupil.set_position(eye_white.position());

        let mut rng = rand::thread_rng();

        const MAX_LOOK_SCORE_COUNTDOWN: i32 = 600;
        const MIN_LOOK_SCORE_COUNTDOWN: i32 = 300;

        const MAX_BLINK_COUNTDOWN: i32 = 360;
        const MIN_BLINK_COUNTDOWN: i32 = 180;

        Entity {
            window,
            position,
            init_position: position,
            prev_position: position,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            size,
            init_size: size,
            half_size,
            scale: Vector2::new(1.0, 1.0),
            name,
            color,
            // Eye
            _eye_white: eye_white,
            _eye_pupil: eye_pupil,
            _look_score_timer: 0,
            _max_look_score_timer: 25,
            _look_score_countdown: rng
                .gen_range(MIN_LOOK_SCORE_COUNTDOWN..MAX_LOOK_SCORE_COUNTDOWN),
            _max_look_score_countdown: MAX_LOOK_SCORE_COUNTDOWN,
            _min_look_score_countdown: MIN_LOOK_SCORE_COUNTDOWN,
            _following_target: false,
            _blink_timer: 0,
            _max_blink_timer: 5,
            _blink_countdown: rng.gen_range(MIN_BLINK_COUNTDOWN..MAX_BLINK_COUNTDOWN),
            _max_blink_countdown: MAX_BLINK_COUNTDOWN,
            _min_blink_countdown: MIN_BLINK_COUNTDOWN,
            // Impact
            _offset: Vector2::new(0.0, 0.0),
            _offset_velocity: Vector2::new(0.0, 0.0),
            _offset_acceleration: Vector2::new(0.0, 0.0),
            _spring_stiffness: spring_stiffness,
            _spring_damping: spring_damping,
        }
    }

    pub fn r#move(&mut self) {
        self.prev_position = self.position;

        self.position += self.velocity;

        self.window.set_position(Vector2::new(
            self.position.x as i32 - self.half_size.x as i32 + self._offset.x as i32,
            self.position.y as i32 - self.half_size.y as i32 + self._offset.y as i32,
        ));
    }

    pub fn move_offset(&mut self) {
        self._offset += self._offset_velocity;

        self.window.set_position(Vector2::new(
            self.position.x as i32 - self.half_size.x as i32 + self._offset.x as i32,
            self.position.y as i32 - self.half_size.y as i32 + self._offset.y as i32,
        ));
    }

    pub fn set_position(&mut self, pos_x: Option<f32>, pos_y: Option<f32>) {
        match pos_x {
            Some(x) => self.position.x = x,
            None => (),
        }

        match pos_y {
            Some(y) => self.position.y = y,
            None => (),
        }

        self.window.set_position(Vector2::new(
            self.position.x as i32 - self.half_size.x as i32 + self._offset.x as i32,
            self.position.y as i32 - self.half_size.y as i32 + self._offset.y as i32,
        ));
    }

    pub fn set_scale(&mut self, scale: f32) {
        if self.scale.x == scale && self.scale.y == scale {
            return;
        }

        self.scale.x = scale;
        self.scale.y = scale;

        self.size = Vector2::new(
            (self.init_size.x as f32 * self.scale.x) as u32,
            (self.init_size.y as f32 * self.scale.y) as u32,
        );

        self.half_size = Vector2::new(self.size.x / 2, self.size.y / 2);

        self.window.set_size((self.size.x, self.size.y));

        self.window.set_position(Vector2::new(
            self.position.x as i32 - self.half_size.x as i32 + self._offset.x as i32,
            self.position.y as i32 - self.half_size.y as i32 + self._offset.y as i32,
        ));
    }

    pub fn set_scale_xy(&mut self, scale_x: Option<f32>, scale_y: Option<f32>) {
        match scale_x {
            Some(x) => self.scale.x = x,
            None => (),
        }

        match scale_y {
            Some(y) => self.scale.y = y,
            None => (),
        }

        self.size = Vector2::new(
            (self.init_size.x as f32 * self.scale.x) as u32,
            (self.init_size.y as f32 * self.scale.y) as u32,
        );

        self.half_size = Vector2::new(self.size.x / 2, self.size.y / 2);

        self.window.set_size((self.size.x, self.size.y));

        self.window.set_position(Vector2::new(
            self.position.x as i32 - self.half_size.x as i32 + self._offset.x as i32,
            self.position.y as i32 - self.half_size.y as i32 + self._offset.y as i32,
        ));
    }

    pub fn get_overlap(entity_1: &Entity, entity_2: &Entity) -> Vector2<i32> {
        let delta: Vector2<i32> = Vector2::new(
            ((entity_1.position.x - entity_2.position.x) as i32).abs(),
            ((entity_1.position.y - entity_2.position.y) as i32).abs(),
        );

        let overlap: Vector2<i32> = Vector2::new(
            (entity_1.half_size.x + entity_2.half_size.x) as i32 - delta.x,
            (entity_1.half_size.y + entity_2.half_size.y) as i32 - delta.y,
        );

        return overlap;
    }

    pub fn get_prev_overlap(entity_1: &Entity, entity_2: &Entity) -> Vector2<i32> {
        let delta: Vector2<i32> = Vector2::new(
            ((entity_1.prev_position.x - entity_2.prev_position.x) as i32).abs(),
            ((entity_1.prev_position.y - entity_2.prev_position.y) as i32).abs(),
        );

        let overlap: Vector2<i32> = Vector2::new(
            (entity_1.half_size.x + entity_2.half_size.x) as i32 - delta.x,
            (entity_1.half_size.y + entity_2.half_size.y) as i32 - delta.y,
        );

        return overlap;
    }

    pub fn draw(&mut self, score_board: &Entity, ball: &Entity) {
        self.window.clear(self.color);

        if self.color != Color::WHITE && self._blink_timer <= 0 {
            self.window.draw(&mut self._eye_white);

            let player_look_target = if self._look_score_timer > 0 {
                score_board.position
            } else {
                ball.position
            };

            let player_look_dir = normalize_vector(
                player_look_target
                    - (self.position + self._eye_white.position()
                        - Vector2::new(self.half_size.x as f32, self.half_size.y as f32)),
            );

            let player_look_angle: f32 = player_look_dir.y.atan2(player_look_dir.x).to_degrees();

            if self._following_target {
                // Have the eye track the target
                self._eye_pupil.set_rotation(player_look_angle);
            } else {
                // Have the eye rotate to the targets position
                self._eye_pupil.set_rotation(interpolate_angle(
                    self._eye_pupil.rotation(),
                    player_look_angle,
                    0.2,
                ));
                if (self._eye_pupil.rotation() - player_look_angle).abs() < 10.0 {
                    self._following_target = true;
                }
            }

            if self.color == Color::YELLOW {
                self._eye_pupil.set_fill_color(Color::GREEN);
            }

            self.window.draw(&mut self._eye_pupil);
            self._eye_pupil.set_fill_color(Color::BLACK);
        }

        self.window.display();
    }

    pub fn update_eye_timers(&mut self) {
        let mut rng = rand::thread_rng();

        //  Player look score

        if self._look_score_countdown > 0 {
            self._look_score_countdown -= 1;
        } else if self._look_score_countdown == 0 {
            self._look_score_timer = self._max_look_score_timer;
            self._look_score_countdown = -1;
            self._following_target = false;
        }

        if self._look_score_timer > 0 {
            self._look_score_timer -= 1;
        } else if self._look_score_timer == 0 {
            self._look_score_countdown =
                rng.gen_range(self._min_look_score_countdown..self._max_look_score_countdown);
            self._look_score_timer = -1;
            self._following_target = false;
        }

        // Player blink

        if self._blink_countdown > 0 {
            self._blink_countdown -= 1;
        } else if self._blink_countdown == 0 {
            self._blink_timer = self._max_blink_timer;
            self._blink_countdown = -1;
        }

        if self._blink_timer > 0 {
            self._blink_timer -= 1;
        } else if self._blink_timer == 0 {
            self._blink_countdown =
                rng.gen_range(self._min_blink_countdown..self._max_blink_countdown);
            self._blink_timer = -1;
        }
    }

    pub fn impact(&mut self, impact_velocity: &Vector2<f32>) {
        self._offset_velocity = *impact_velocity;
    }

    pub fn update_impact(&mut self) {
        // If it's not moving, do nothing
        if self._offset_velocity.x == 0.0 && self._offset_velocity.y == 0.0 {
            return;
        }

        let displacement = Vector2::new(0.0, 0.0) - self._offset;
        let spring_force = displacement * self._spring_stiffness;
        self._offset_velocity = (self._offset_velocity + spring_force) * self._spring_damping;

        if self._offset_velocity.x.abs() < 0.1 && self._offset_velocity.y.abs() < 0.1 {
            self._offset_velocity.x = 0.0;
            self._offset_velocity.y = 0.0;
            self._offset.x = 0.0;
            self._offset.y = 0.0;
        }

        self.move_offset();
    }
}
