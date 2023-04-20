// #![windows_subsystem = "windows"]

mod physics;

use macroquad::{prelude::*};
use std::time::Instant;

pub const WIDTH: f32 = 1200.;
pub const HEIGHT: f32 = 800.;
pub const RADIUS: f32 = 400.; 
pub const GRID_SIZE: f32 = 20.;

fn window_config() -> Conf {
    Conf {
        window_title: "Physics".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut engine = physics::PhysicsEngine::new(WIDTH / 2., HEIGHT / 2., RADIUS);

    let mut now = Instant::now();

    loop {
        let dt = now.elapsed().as_micros() as f32 / 1_000_000.0;
        now = Instant::now();

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();

            for x in 0..10 {
                for y in 0..10 {
                    let x_offset = (x as f32) * 16. - 20.;
                    let y_offset = (y as f32) * 16. - 20.;

                    engine.spawn_object(physics::Object::new(mouse_position.0 + x_offset, mouse_position.1 + y_offset, 4., WHITE, false));
                }
            }
        }
        
        if is_mouse_button_pressed(MouseButton::Right) {
            let mouse_position = mouse_position();

            engine.spawn_object(physics::Object::new(mouse_position.0, mouse_position.1, 8., RED, true));
        }

        
        if is_mouse_button_pressed(MouseButton::Middle) {
            let mouse_position = mouse_position();
            let mouse_position = Vec2::new(mouse_position.0, mouse_position.1);

            engine.spawn_object(physics::Object::new(mouse_position.x, mouse_position.y, 8., RED, true));

            for i in 1..20 {
                let pos = mouse_position + i as f32 * Vec2::new(32., 0.);

                if i == 19 {
                    engine.spawn_object(
                        physics::Object::new(pos.x, pos.y, 8., RED, true)
                    );
                } else {
                    engine.spawn_object(
                        physics::Object::new(pos.x, pos.y, 8., BLUE, false)
                    );
                }

                engine.link_last_two(32.);
            }
        }

        // if is_key_down(KeyCode::Space) || is_key_pressed(KeyCode::Right) {
        //     engine.update(dt);
        // }
        engine.update(dt);

        draw_poly(WIDTH / 2., HEIGHT / 2., 100, RADIUS, 0., BLACK);

        clear_background(GRAY);

        draw_poly(WIDTH / 2., HEIGHT / 2., 100, RADIUS, 0., BLACK);
    
        engine.render();

        next_frame().await;
    }
}
