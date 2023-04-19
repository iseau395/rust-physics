use crate::{GRID_SIZE, RADIUS};

use super::{Object};
use macroquad::prelude::Vec2;

const GRID_WIDTH: usize = (RADIUS * 2.5 / GRID_SIZE) as usize;
const GRID_HEIGHT: usize = (RADIUS * 2.5 / GRID_SIZE) as usize;

pub struct Link(usize, usize, f32);

pub struct PhysicsEngine {
    objects: Vec<Object>,
    cells: [Vec<usize>; GRID_WIDTH * GRID_HEIGHT],

    links: Vec<Link>,

    constraint_center: Vec2,
    radius: f32,

    gravity: Vec2,
}

impl PhysicsEngine {
    pub fn new(center_x: f32, center_y: f32, radius: f32) -> PhysicsEngine {
        const VAL: Vec<usize> = vec![];

        PhysicsEngine {
            objects: vec![],
            cells: [VAL; GRID_WIDTH * GRID_HEIGHT],

            links: vec![],

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

            for cell_index in 0..self.cells.len() {
                for obj_index in 0..self.cells[cell_index].len() {
                    self.audit_object_cell(cell_index, obj_index);
                }
            }

            // Calculate collisions
            self.calculate_collisions();
        }

        // let mut total_objects = 0;
        // for cell in self.cells.iter() {
        //     total_objects += cell.len();
        // }
        // println!("{}", total_objects);
    }

    fn update_objects(&mut self, dt: f32) {
        for obj_id in 0..self.objects.len() {
            let obj = &mut self.objects[obj_id];

            obj.accelerate(self.gravity);

            obj.update_position(dt);

            self.constrain_border(obj_id);
        }

        for link in self.links.iter() {
            let obj_a = self.objects[link.0];
            let obj_b = self.objects[link.1];

            let axis = obj_a.position - obj_b.position;
            let distance = axis.length();
            
            let normalized = axis.normalize();
            let delta = link.2 - distance;

            let a_size_ratio = if obj_a.pinned {
                0.
            } else if obj_b.pinned {
                1.
            } else {
                0.5
            };
            let b_size_ratio = if obj_b.pinned {
                0.
            } else if obj_a.pinned {
                1.
            } else {
                0.5
            };

            let obj_a = &mut self.objects[link.0];
            obj_a.position += delta * normalized * a_size_ratio;

            let obj_b = &mut self.objects[link.1];
            obj_b.position -= delta * normalized * b_size_ratio;

        }
    }

    fn constrain_border(&mut self, obj_id: usize) {
        let obj = &mut self.objects[obj_id];

        let relative_position = obj.position - self.constraint_center;

        let distance = relative_position.length();

        if distance > self.radius - obj.radius {
            let normalized = relative_position.normalize();
            obj.position = self.constraint_center + normalized * (self.radius - obj.radius);
        }
    }

    fn calculate_collisions(&mut self) {
        for cell_a_index in 0..self.cells.len() {
            for cell_b_x in 0..=2 {
                let cell_b_x = cell_b_x - 1;

                for cell_b_y in 0..=2 {
                    let cell_b_y = cell_b_y - 1;

                    let cell_b_index = cell_a_index + cell_b_x + cell_b_y * GRID_WIDTH;

                    if cell_b_index % GRID_WIDTH == 0 || cell_b_index % GRID_WIDTH == GRID_WIDTH - 1
                    {
                        continue;
                    }
                    if cell_b_index >= self.cells.len() {
                        continue;
                    }

                    let mut obj_a_index = 0;
                    'obj_a_loop: loop {
                        let mut obj_b_index = 0;
                        'obj_b_loop: loop {
                            if obj_a_index >= self.cells[cell_a_index].len() {
                                break 'obj_a_loop;
                            }

                            if obj_b_index >= self.cells[cell_b_index].len() {
                                break 'obj_b_loop;
                            }

                            if obj_a_index == obj_b_index && cell_a_index == cell_b_index {
                                obj_b_index += 1;
                                continue 'obj_b_loop;
                            }

                            let obj_a_id = self.cells[cell_a_index][obj_a_index];
                            let obj_b_id = self.cells[cell_b_index][obj_b_index];

                            let obj_a = &self.objects[obj_a_id];
                            let obj_b = &self.objects[obj_b_id];

                            let collision_axis = obj_a.position - obj_b.position;
                            let distance = collision_axis.length();

                            let min_distance = obj_a.radius + obj_b.radius;

                            if distance < min_distance {
                                let normalized = collision_axis.normalize();
                                let delta = min_distance - distance;

                                let a_size_ratio = if obj_a.pinned {
                                    0.
                                } else if obj_b.pinned {
                                    1.
                                } else {
                                    obj_a.radius / (obj_a.radius + obj_b.radius)
                                };
                                let b_size_ratio = if obj_b.pinned {
                                    0.
                                } else if obj_a.pinned {
                                    1.
                                } else {
                                    obj_b.radius / (obj_a.radius + obj_b.radius)
                                };

                                let obj_a = &mut self.objects[obj_a_id];
                                obj_a.position += delta * normalized * a_size_ratio;

                                let obj_b = &mut self.objects[obj_b_id];
                                obj_b.position -= delta * normalized * b_size_ratio;

                                if cell_a_index == cell_b_index {
                                    let old_length = self.cells[cell_a_index].len();

                                    self.audit_object_cell(cell_a_index, obj_a_index);
                                    if obj_b_index == old_length - 1
                                        && old_length != self.cells[cell_a_index].len()
                                    {
                                        self.audit_object_cell(cell_a_index, obj_a_index);
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
        let cell = &mut self.cells[cell_index];

        let cell_x = cell_index as f32 % GRID_WIDTH as f32 * GRID_SIZE
            + (self.constraint_center.x - GRID_WIDTH as f32 / 2. * GRID_SIZE);
        let cell_y = (cell_index as f32 / GRID_WIDTH as f32).floor() * GRID_SIZE
            + (self.constraint_center.y - GRID_HEIGHT as f32 / 2. * GRID_SIZE);

        let obj = cell.get(obj_index);
        if obj.is_none() {
            return;
        }
        let obj = self.objects[*obj.unwrap()];

        let x = obj.position.x;
        let y = obj.position.y;

        if x < cell_x || y < cell_y || x > cell_x + GRID_SIZE || y > cell_y + GRID_SIZE {
            let obj = cell.swap_remove(obj_index);

            self.assign_object_cell(obj);
        }
    }

    fn assign_object_cell(&mut self, obj: usize) {
        let position = self.objects[obj].position;

        let mut relative_x =
            position.x - (self.constraint_center.x - GRID_WIDTH as f32 / 2. * GRID_SIZE);
        let mut relative_y =
            position.y - (self.constraint_center.y - GRID_HEIGHT as f32 / 2. * GRID_SIZE);

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

        self.cells[grid_x + grid_y * GRID_WIDTH].push(obj);
    }

    pub fn spawn_object(&mut self, obj: Object) {
        self.objects.push(obj);

        self.assign_object_cell(self.objects.len() - 1);
    }

    pub fn add_link(&mut self, obj_a_id: usize, obj_b_id: usize, length: f32) {
        self.links.push(Link(obj_a_id, obj_b_id, length));
    }

    pub fn link_last_two(&mut self, length: f32) {
        self.add_link(self.objects.len() - 2, self.objects.len() - 1, length);
    }

    pub fn render(&self) {
        for object in self.objects.iter() {
            object.render();
        }
    }
}
