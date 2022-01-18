use std::collections::HashMap;

use crate::engine::d2::{input::TouchPoint, subsystem::TouchSystem, util::Signal1};

use super::BasicPointer;

pub struct BasicTouch {
    pub down: Signal1<TouchPoint>,
    pub move_: Signal1<TouchPoint>,
    pub up: Signal1<TouchPoint>,

    pointer: BasicPointer,
    pointer_touch: Option<TouchPoint>,

    max_points: i32,
    point_map: HashMap<i32, TouchPoint>,
    points: Vec<TouchPoint>,
}

impl BasicTouch {
    // maxPoints: i32 = 4
    pub fn new(pointer: BasicPointer, max_points: i32) -> Self {
        Self {
            pointer,
            max_points,
            point_map: HashMap::new(),
            points: Vec::new(),

            down: Signal1::new(None),
            move_: Signal1::new(None),
            up: Signal1::new(None),
            pointer_touch: None,
        }
    }

    pub fn submit_down(&mut self, id: i32, view_x: f32, view_y: f32) {
        if !self.point_map.contains_key(&id) {
            let mut point = TouchPoint::new(id);
            point.init(view_x, view_y);
            self.point_map.insert(id, point.clone());
            self.points.push(point.clone());

            if self.pointer_touch.is_none() {
                // Make this touch point the tracked pointer
                self.pointer_touch = Some(point.clone());
                if let Some(ref source) = point.source {
                    self.pointer.submit_down(view_x, view_y, *source.clone());
                }
            }
            self.down.emit(point);
        }
    }

    pub fn submit_move(&mut self, id: i32, view_x: f32, view_y: f32) {
        if let Some(point) = self.point_map.get_mut(&id) {
            point.init(view_x, view_y);

            if let Some(ref pointer_touch) = self.pointer_touch {
                if pointer_touch == point {
                    if let Some(ref source) = point.source {
                        self.pointer.submit_move(view_x, view_y, *source.clone());
                    }
                }
            }

            self.move_.emit(point.clone());
        }
    }

    pub fn submit_up(&mut self, id: i32, view_x: f32, view_y: f32) {
        if let Some(point) = self.point_map.get_mut(&id) {
            point.init(view_x, view_y);

            if let Some(idx) = self.points.iter().position(|x| x == point) {
                self.points.remove(idx);
            }

            if let Some(ref mut pointer_touch) = self.pointer_touch {
                if pointer_touch == point {
                    self.pointer_touch = None;
                    if let Some(ref source) = point.source {
                        self.pointer.submit_up(view_x, view_y, *source.clone());
                    }
                }
            }

            self.up.emit(point.clone());
        }

        self.point_map.remove(&id);
    }
}

impl TouchSystem for BasicTouch {
    fn down_signal(&self) -> &Signal1<TouchPoint> {
        &self.down
    }

    fn move_signal(&self) -> &Signal1<TouchPoint> {
        &self.move_
    }

    fn up_signal(&self) -> &Signal1<TouchPoint> {
        &self.up
    }

    fn is_supported(&self) -> bool {
        true
    }

    fn max_points(&self) -> i32 {
        self.max_points
    }

    fn points(&self) -> Vec<TouchPoint> {
        self.points.clone()
    }
}
