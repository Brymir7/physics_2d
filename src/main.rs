use macroquad::prelude::*;
use cgmath::Vector2;
use ::rand::Rng;
const SIZE: f32 = 512.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        window_width: SIZE as i32,
        window_height: SIZE as i32,
        ..Default::default()
    }
}
struct Square {
    position: cgmath::Vector2<f32>,
    velocity: cgmath::Vector2<f32>,
    size: f32,
}
impl Square {
    fn new(position: cgmath::Vector2<f32>, velocity: cgmath::Vector2<f32>, size: f32) -> Square {
        Square {
            position,
            velocity,
            size,
        }
    }
    fn bounds_check_and_resolve(&mut self, bounds: Bounds) {
        
        if self.position.x < bounds.left {
            self.velocity[0] *= -0.90;
        } else if self.position.x + self.size> bounds.right {
            self.velocity[0] *= -0.90;
        }
    
        if self.position.y + self.size < bounds.bottom {
            self.velocity[1] *= -0.90;
        } else if self.position.y> bounds.top {
            self.velocity[1] *= -0.90;
        }
    }
    fn check_collision<T>(&self, other: &T) -> bool {
        return false;
    }
    fn update(&mut self, bounds: Bounds,  dt: f32) {
        self.bounds_check_and_resolve(bounds);
        self.position += self.velocity * dt;
    }
    fn render (&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size,
            self.size,
            RED,
        )
    }
}
struct TransformationMatrix {

}
#[derive(Copy, Clone)]
struct Bounds {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}
impl Bounds {
    fn new(left: f32, right: f32, top: f32, bottom: f32) -> Bounds {
        Bounds {
            left, 
            right,
            top,
            bottom,
        }
    }
}
struct World {
    objects: Vec<Square>,
    size: f32,
}
impl World {
    fn new(amount_of_objects: Option<i32>, size: f32) -> World {
        let mut rng = ::rand::thread_rng();
        match amount_of_objects {
            Some(amount) => {
                let objects: Vec<Square> = (0..amount).map(|_| Square {
                    position: (
                        rng.gen_range(0.0..size - 30.0),
                        rng.gen_range(0.0..size - 30.0),
                    ).into(),
                    velocity: (
                        rng.gen_range(-10.0..30.0),
                        rng.gen_range(-10.0..30.0),
                    ).into(),
                    size: 30.0,
                })
                .collect();
                return World {
                    objects,
                    size,
                }
            }
            None => World{
                objects: Vec::new(),
                size,
            } 
        }
    }
    fn get_bounds(&self) -> Bounds {
        Bounds { left: 0.0, right: self.size, top: 0.0, bottom: self.size}
    }    
    
    fn render(&self) {
        for object in &self.objects {
            object.render();
        }
    }
    fn update(&mut self, dt: f32) {
        let bounds = self.get_bounds();
        for object in &mut self.objects {
            object.update(bounds, dt );
        }
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut World = World::new(Some(150), SIZE);
    let dt = 0.05;
    loop {
        clear_background(BLACK);
        World.update(dt);
        World.render();
        next_frame().await;
    }
}
