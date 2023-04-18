use crate::{GRID_SIZE, RADIUS};

use super::Object;
use macroquad::prelude::Vec2;

const GRID_WIDTH: usize = (RADIUS * 2.5 / GRID_SIZE) as usize;
const GRID_HEIGHT: usize = (RADIUS * 2.5 / GRID_SIZE) as usize;

pub struct PhysicsEngine {
    objects: [Vec<Object>; GRID_WIDTH * GRID_HEIGHT],

    constraint_center: Vec2,
    radius: f32,

    gravity: Vec2,
}

impl PhysicsEngine {
    pub fn new(center_x: f32, center_y: f32, radius: f32) -> PhysicsEngine {
        const VAL: Vec<Object> = vec![];

        PhysicsEngine {
            objects: [VAL; GRID_WIDTH * GRID_HEIGHT],

            constraint_center: Vec2::new(center_x, center_y),
            radius,

            gravity: Vec2 { x: 0., y: 1000. },
        }
    }

    pub fn update(&mut self, dt: f32) {
        let sub_steps = 8;

        for _ in 0..sub_steps {
            let dt = dt / sub_steps as f32;

            self.update_objects(dt);

            for cell_index in 0..self.objects.len() {
                for obj_index in 0..self.objects[cell_index].len() {
                    self.audit_object_cell(cell_index, obj_index);
                }
            }

            // Calculate collisions
            self.calculate_collisions();
        }

        let mut total_objects = 0;
        for cell in self.objects.iter() {
            total_objects += cell.len();
        }
        println!("{}", total_objects);
    }

    fn update_objects(&mut self, dt: f32) {
        for cell_index in 0..self.objects.len() {
            for obj_index in 0..self.objects[cell_index].len() {
                let cell = self.objects.get_mut(cell_index);
                let cell = cell.unwrap();

                let obj = cell.get_mut(obj_index);
                let obj = obj.unwrap();

                obj.accelerate(self.gravity);

                obj.update_position(dt);

                self.constrain_border(cell_index, obj_index)
            }
        }
    }

    fn constrain_border(&mut self, cell_index: usize, obj_index: usize) {
        let cell = &mut self.objects[cell_index];

        let obj = &mut cell[obj_index];

        let relative_position = obj.position - self.constraint_center;

        let distance = relative_position.length();

        if distance > self.radius - obj.radius {
            let normalized = relative_position.normalize();
            obj.position = self.constraint_center + normalized * (self.radius - obj.radius);

            // self.audit_object_cell(cell_index, obj_index);
        }
    }

    fn calculate_collisions(&mut self) {
        for cell_a_index in 0..self.objects.len() {
            for cell_b_x in 0..3 {
                let cell_b_x = cell_b_x - 1;

                for cell_b_y in 0..3 {
                    let cell_b_y = cell_b_y - 1;

                    let cell_b_index = cell_a_index + cell_b_x + cell_b_y * GRID_WIDTH;

                    if cell_b_index % GRID_WIDTH == 0 || cell_b_index % GRID_WIDTH == GRID_WIDTH - 1
                    {
                        continue;
                    }
                    if cell_b_index >= self.objects.len() {
                        continue;
                    }

                    let mut obj_a_index = 0;
                    'obj_a_loop: loop {
                        let mut obj_b_index = 0;
                        'obj_b_loop: loop {
                            if obj_a_index >= self.objects[cell_a_index].len() {
                                break 'obj_a_loop;
                            }

                            if obj_b_index >= self.objects[cell_b_index].len() {
                                break 'obj_b_loop;
                            }

                            if obj_a_index == obj_b_index && cell_a_index == cell_b_index {
                                obj_b_index += 1;
                                continue 'obj_b_loop;
                            }

                            let cell_a = self.objects.get(cell_a_index).unwrap();
                            let cell_b = self.objects.get(cell_b_index).unwrap();

                            let obj_a = &cell_a[obj_a_index];
                            let obj_b = &cell_b[obj_b_index];

                            let collision_axis = obj_a.position - obj_b.position;
                            let distance = collision_axis.length();

                            let min_distance = obj_a.radius + obj_b.radius;
                            if distance < min_distance {
                                let normalized = collision_axis.normalize();
                                let delta = min_distance - distance;

                                let a_size_ratio = obj_a.radius / (obj_a.radius + obj_b.radius);
                                let b_size_ratio = obj_b.radius / (obj_a.radius + obj_b.radius);

                                let obj_a = &mut self.objects[cell_a_index][obj_a_index];
                                obj_a.position += delta * normalized * a_size_ratio;

                                let obj_b = &mut self.objects[cell_b_index][obj_b_index];
                                obj_b.position -= delta * normalized * b_size_ratio;

                                if cell_a_index == cell_b_index {
                                    let old_length = self.objects[cell_a_index].len();

                                    self.audit_object_cell(cell_a_index, obj_a_index);
                                    if obj_b_index == old_length - 1
                                        && old_length != self.objects[cell_a_index].len()
                                    {
                                        self.audit_object_cell(cell_b_index, obj_a_index);
                                    } else {
                                        self.audit_object_cell(cell_b_index, obj_b_index);
                                    }
                                } else {
                                    self.audit_object_cell(cell_a_index, obj_a_index);
                                    self.audit_object_cell(cell_b_index, obj_b_index);
                                }
                            }
                            obj_b_index += 1;
                        }
                        obj_a_index += 1;
                    }
                }
            }
        }
    }

    fn audit_object_cell(&mut self, cell_index: usize, obj_index: usize) {
        let cell = &mut self.objects[cell_index];

        let cell_x = cell_index as f32 % GRID_WIDTH as f32 * GRID_SIZE
            + (self.constraint_center.x - GRID_WIDTH as f32 / 2. * GRID_SIZE);
        let cell_y = (cell_index as f32 / GRID_WIDTH as f32).floor() * GRID_SIZE
            + (self.constraint_center.y - GRID_HEIGHT as f32 / 2. * GRID_SIZE);

        let obj = cell.get(obj_index);
        if obj.is_none() {
            return;
        }
        let obj = obj.unwrap();

        let x = obj.position.x;
        let y = obj.position.y;

        if x < cell_x || y < cell_y || x > cell_x + GRID_SIZE || y > cell_y + GRID_SIZE {
            let obj = cell.swap_remove(obj_index);

            self.spawn_object(obj);
        }
    }

    pub fn spawn_object(&mut self, obj: Object) {
        let mut relative_x =
            obj.position.x - (self.constraint_center.x - GRID_WIDTH as f32 / 2. * GRID_SIZE);
        let mut relative_y =
            obj.position.y - (self.constraint_center.y - GRID_HEIGHT as f32 / 2. * GRID_SIZE);

        if relative_x < 0. {
            relative_x = 0. + 1.;
        }
        if relative_x > (GRID_WIDTH as f32 / 2. * GRID_SIZE) * 2. {
            relative_x = (GRID_WIDTH as f32 / 2. * GRID_SIZE) * 2. - 1.;
        }
        if relative_y < 0. {
            relative_y = 0. + 1.;
        }
        if relative_y > (GRID_HEIGHT as f32 / 2. * GRID_SIZE) * 2. {
            relative_y = (GRID_HEIGHT as f32 / 2. * GRID_SIZE) * 2. - 1.;
        }

        let grid_x = (relative_x / GRID_SIZE).floor() as usize;
        let grid_y = (relative_y / GRID_SIZE).floor() as usize;

        self.objects[grid_x + grid_y * GRID_WIDTH].push(obj);
    }

    pub fn render(&self) {
        for cell in self.objects.iter() {
            for obj in cell.iter() {
                obj.render();
            }
        }
    }
}
