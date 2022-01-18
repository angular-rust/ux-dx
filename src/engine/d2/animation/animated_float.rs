use std::{cell::RefCell, fmt, rc::Rc};

use crate::engine::d2::util::{Listener2, Value};

use super::{Behavior, Binding, BindingFunction, EaseFunction, Tween};

struct AnimatedFloatProps {
    pub value: Value<f32>,

    // The behavior that is currently animating the value, or None if the value is not being
    // animated.
    behavior: Option<Rc<dyn Behavior>>,
}

/// A Float value that may be animated over time.

#[derive(Clone)]
pub struct AnimatedFloat {
    props: Rc<RefCell<AnimatedFloatProps>>,
}

impl AnimatedFloat {
    pub fn new(value: f32, listener: Option<Listener2<f32, f32>>) -> Self {
        Self {
            props: Rc::new(RefCell::new(AnimatedFloatProps {
                value: Value::new(value, listener),
                behavior: None,
            })),
        }
    }

    #[inline]
    pub fn get(&self) -> f32 {
        *self.props.borrow().value.get()
    }

    // override
    pub fn set(&self, value: f32) {
        let mut props = self.props.borrow_mut();
        props.behavior = None;
        props.value.set(value);
    }

    pub fn update(&self, dt: f32) {
        let mut props = self.props.borrow_mut();
        let result = props.behavior.as_ref().map(|behavior| {
            let value = behavior.update(dt);
            let is_complete = behavior.is_complete();
            (value, is_complete)
        });

        if let Some((value, is_complete)) = result {
            props.value.set(value);

            if is_complete {
                props.behavior = None;
            }
        }
    }

    /// Animates between the two given values.
    ///  *
    /// @param from The initial value.
    /// @param to The target value.
    /// @param seconds The animation duration, in seconds.
    /// @param easing The easing fn to use, defaults to `Ease.linear`.
    pub fn animate(&self, from: f32, to: f32, seconds: f32, easing: Option<EaseFunction>) {
        self.set(from);
        self.animate_to(to, seconds, easing);
    }

    /// Animates between the current value and the given value.
    ///  *
    /// @param to The target value.
    /// @param seconds The animation duration, in seconds.
    /// @param easing The easing fn to use, defaults to `Ease.linear`.
    pub fn animate_to(&self, to: f32, seconds: f32, easing: Option<EaseFunction>) {
        let mut props = self.props.borrow_mut();
        props.behavior = Some(Rc::new(Tween::new(*props.value.get(), to, seconds, easing)));
    }

    /// Animates the current value by the given delta.
    ///  *
    /// @param by The delta added to the current value to get the target value.
    /// @param seconds The animation duration, in seconds.
    /// @param easing The easing fn to use, defaults to `Ease.linear`.
    pub fn animate_by(&self, by: f32, seconds: f32, easing: Option<EaseFunction>) {
        let mut props = self.props.borrow_mut();
        props.behavior = Some(Rc::new(Tween::new(*props.value.get(), *props.value.get() + by, seconds, easing)));
    }

    #[inline]
    pub fn bind_to(&self, to: Value<f32>, function: Option<BindingFunction>) {
        let mut props = self.props.borrow_mut();
        props.behavior = Some(Rc::new(Binding::new(to, function)));
    }

    pub fn set_behavior(&self, behavior: Option<Rc<dyn Behavior>>) {
        let mut props = self.props.borrow_mut();
        props.behavior = behavior;
        self.update(0.0);
    }

    #[inline]
    pub fn behavior(&self) -> Option<Rc<dyn Behavior>> {
        let props = self.props.borrow();
        props.behavior.clone()
    }
}

impl Default for AnimatedFloat {
    fn default() -> Self {
        Self {
            props: Rc::new(RefCell::new(AnimatedFloatProps {
                value: Value::new(0.0, None),
                behavior: None,
            })),
        }
    }
}

// TODO:
impl fmt::Display for AnimatedFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.props.borrow().value.value)
    }
}

impl fmt::Debug for AnimatedFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnimatedFloat")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}

// TODO: need some way to clone boxed closures
// impl Clone for AnimatedFloat {
//     fn clone(&self) -> Self {
//         Self {
//             inner: Value::new(self.inner.value, None),
//             behavior: None,
//         }
//     }
// }

// impl AsRef<Value<f32>> for AnimatedFloat {
//     fn as_ref(&self) -> &Value<f32> {
//         &self.props.borrow().value
//     }
// }
