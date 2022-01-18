use crate::engine::d2::{input::TouchPoint, subsystem::TouchSystem, util::Signal1};

pub struct DummyTouch {
    // pub supported: bool,
    // pub maxPoints: i32,
    // pub points: Vec<TouchPoint>,
    pub down: Signal1<TouchPoint>,
    pub move_: Signal1<TouchPoint>,
    pub up: Signal1<TouchPoint>,
}

impl DummyTouch {
    pub fn new() -> Self {
        Self {
            down: Signal1::new(None),
            move_: Signal1::new(None),
            up: Signal1::new(None),
        }
    }
}

impl TouchSystem for DummyTouch {
    fn is_supported(&self) -> bool {
        return false;
    }

    fn max_points(&self) -> i32 {
        return 0;
    }

    fn down_signal(&self) -> &Signal1<TouchPoint> {
        &self.down
    }

    fn move_signal(&self) -> &Signal1<TouchPoint> {
        &self.move_
    }

    fn up_signal(&self) -> &Signal1<TouchPoint> {
        &self.up
    }

    fn points(&self) -> Vec<TouchPoint> {
        Vec::new()
    }
}
