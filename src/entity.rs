use sfml::{
    graphics::RenderWindow,
    system::Vector2,
    window::{ContextSettings, Style},
};

pub struct Entity {
    pub window: RenderWindow,
    pub position: Vector2<f32>,
    prev_position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub size: Vector2<u32>,
    pub half_size: Vector2<u32>,
    pub name: String,
}

impl Entity {
    pub fn new(position: Vector2<f32>, width: u32, height: u32, name: String) -> Entity {
        let mut window = RenderWindow::new(
            (width, height),
            "",
            Style::CLOSE,
            &ContextSettings::default(),
        );

        let half_size: Vector2<u32> = Vector2::new(width / 2, height / 2);

        window.set_position(Vector2::new(
            position.x as i32 - half_size.x as i32,
            position.y as i32 - half_size.y as i32,
        ));

        Entity {
            window,
            position,
            prev_position: position,
            velocity: Vector2::new(0.0, 0.0),
            size: Vector2::new(width, height),
            half_size,
            name,
        }
    }

    pub fn r#move(&mut self) {
        self.prev_position = self.position;

        self.position += self.velocity;

        self.window.set_position(Vector2::new(
            self.position.x as i32 - self.half_size.x as i32,
            self.position.y as i32 - self.half_size.y as i32,
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
            self.position.x as i32 - self.half_size.x as i32,
            self.position.y as i32 - self.half_size.y as i32,
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
}
