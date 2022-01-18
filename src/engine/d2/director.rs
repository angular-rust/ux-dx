use once_cell::sync::OnceCell;
use std::{fmt, sync::Arc, sync::Mutex};

use super::{
    scene::{Scene2D, Transition},
    subsystem::*,
    util::Disposable,
    Component, Entity, EntityManager, System,
};

/// Manages a stack of scenes. Only the front-most scene receives game updates.

// TODO -> Major robustness cleanup and testing needed here

#[derive(Default)]
struct DirectorProps {
    inner: Component,

    /// The front-most scene
    top_scene: Option<Entity>,

    /// The complete list of scenes managed by this director, from back to front
    scenes: Vec<Entity>,

    /// The scenes that are partially occluded by a transparent or transitioning scene,
    /// from back to front. These scenes are not updated, but they're still drawn
    occluded_scenes: Vec<Entity>,

    /// The container for the current scene
    root: Entity,

    transitor: Option<Transitor>,

    /// The ideal width of the director's scenes. Used by some transitions
    width: f32, // = -1;

    /// The ideal height of the director's scenes. Used by some transitions
    height: f32, // = -1;
}

unsafe impl Send for DirectorProps {}

#[derive(Default)] // Clone, Debug
pub struct Director {
    props: Arc<Mutex<DirectorProps>>,
}

impl Director {
    fn new() -> Self {
        Self {
            props: Arc::new(Mutex::new(Default::default())),
            // inner: Component::default(),
            // top_scene: None,
            // scenes: Vec::new(),
            // occluded_scenes: Vec::new(),
            // root: Entity::new(),
            // height: 0.0,
            // width: 0.0,
            // transitor: None,
        }
    }

    /// Retrieves the singleton instance of `Settings`
    ///
    /// # Returns
    ///
    /// the instance of `Settings`. The
    ///  returned object is owned by internals and it should not be unreferenced
    ///  directly
    fn global() -> &'static Self {
        static DIRECTOR_INSTANCE: OnceCell<Director> = OnceCell::new();
        DIRECTOR_INSTANCE.get_or_init(Self::new)
    }

    /// Sets the ideal size of the scenes in this director. By default, the size is the full stage
    /// width and height. This size is used by some transitions, such as SlideTransition.
    pub fn set_size(width: f32, height: f32) {
        match Self::global().props.lock() {
            Ok(mut props) => {
                props.width = width;
                props.height = height;
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    pub fn push_scene(scene: Entity, transition: Option<Transition>) {
        let director = Self::global();
        director.complete_transition();
        if let Some(old_top) = director.top_scene() {
            let old_top = old_top.clone();
            director.play_transition(
                old_top.clone(),
                scene,
                transition,
                Some(Arc::new(move || {
                    Self::hide(old_top);
                })),
            );
        } else {
            director.add(scene);
            director.invalidate_visibility();
        }
    }

    pub fn pop_scene(transition: Option<Transition>) {
        let director = Self::global();

        director.complete_transition();

        if let Some(old_top) = director.top_scene() {
            match director.props.lock() {
                Ok(mut props) => {
                    props.scenes.pop(); // Pop old_top
                }
                Err(_) => {
                    panic!("Context poisoned")
                }
            }

            if let Some(new_top) = director.top_scene() {
                director.play_transition(
                    old_top,
                    new_top,
                    transition,
                    Some(Arc::new(move || {
                        Self::hide_and_dispose(old_top);
                    })),
                );
            } else {
                Self::hide_and_dispose(old_top);
                director.invalidate_visibility();
            }
        }
    }

    /// Pops the scene stack until the given entity is the top scene, or adds the scene if the stack
    /// becomes empty while popping.
    pub fn unwind_to_scene(&self, scene: Entity, transition: Option<Transition>) {
        let director = Self::global();

        director.complete_transition();

        if let Some(old_top) = director.top_scene() {
            if old_top == scene {
                return; // We're already there
            }

            match director.props.lock() {
                Ok(mut props) => {
                    props.scenes.pop(); // Pop old_top
                    while props.scenes.len() > 0 && props.scenes[props.scenes.len() - 1] != scene {
                        if let Some(entity) = props.scenes.pop() {
                            entity.dispose(); // Don't emit a hide, just dispose them
                        }
                    }
                }
                Err(_) => {
                    panic!("Context poisoned")
                }
            }

            director.play_transition(
                old_top,
                scene,
                transition,
                Some(Arc::new(move || {
                    Self::hide_and_dispose(old_top);
                })),
            );
        } else {
            Self::push_scene(scene, transition);
        }
    }

    // override
    pub fn on_added(&self) {
        let director = Self::global();

        match director.props.lock() {
            Ok(props) => {
                if let Some(owner) = props.inner.owner {
                    owner.entity().add_child(props.root.clone(), true, None);
                }
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    // override
    pub fn on_removed(&self) {
        let director = Self::global();

        director.complete_transition();

        match director.props.lock() {
            Ok(mut props) => {
                for scene in props.scenes.iter_mut() {
                    scene.dispose();
                }
                props.scenes = Vec::new();
                props.occluded_scenes = Vec::new();

                props.root.dispose();
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    // override
    pub fn on_update(&self, dt: f32) {
        let director = Self::global();
        match director.props.lock() {
            Ok(props) => {
                if let Some(ref transitor) = props.transitor {
                    if transitor.update(dt) {
                        director.complete_transition();
                    }
                }
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    fn top_scene(&self) -> Option<Entity> {
        match self.props.lock() {
            Ok(props) => props.scenes.last().map(|scene| scene.clone()),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    /// Whether the director is currently transitioning between scenes.
    #[inline]
    fn transitioning(&self) -> bool {
        match self.props.lock() {
            Ok(props) => props.transitor.is_some(),
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    fn add(&self, scene: Entity) {
        match self.props.lock() {
            Ok(mut props) => {
                if let Some(ref old_top) = self.top_scene() {
                    props.root.remove_child(old_top);
                }

                if let Some(idx) = props.scenes.iter().position(|item| item == &scene) {
                    props.scenes.remove(idx);
                }
                props.scenes.push(scene.clone());
                props.root.add_child(scene, true, None);
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    fn hide(scene: Entity) {
        if let Some(ref events) = EntityManager::<Scene2D>::get(&scene) {
            events.hidden.emit();
        }
    }

    fn hide_and_dispose(scene: Entity) {
        Self::hide(scene);
        if EntityManager::<Scene2D>::get(&scene).is_some() {
            scene.dispose();
        }
    }

    fn show(scene: &Entity) {
        if let Some(ref events) = EntityManager::<Scene2D>::get(scene) {
            events.shown.emit();
        }
    }

    fn invalidate_visibility(&self) {
        // Find the last index of an opaque scene, or 0
        match self.props.lock() {
            Ok(mut props) => {
                let mut idx = props.scenes.len();
                while idx > 0 {
                    idx -= 1;
                    if let Some(scene) = props.scenes.get(idx) {
                        if let Some(ref comp) = EntityManager::<Scene2D>::get(scene) {
                            if comp.opaque {
                                break;
                            }
                        }
                    } else {
                        break; // DV
                    }
                }

                // All visible scenes, excluding the top scene
                props.occluded_scenes = if props.scenes.len() > 0 {
                    let mut out = props.scenes.to_owned();
                    out.truncate(props.scenes.len() - 1);
                    out
                } else {
                    Vec::new()
                };

                // Notify the new top scene that it's being shown
                if let Some(scene) = self.top_scene() {
                    Self::show(&scene);
                }
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    // Notes:
    // - Any method that modifies the scene stack should immediately call completeTransition.
    fn complete_transition(&self) {
        let should_invalidate = match self.props.lock() {
            Ok(mut props) => {
                if let Some(ref transitor) = props.transitor {
                    transitor.complete();
                    props.transitor = None;
                    true
                } else {
                    false
                }
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        };

        if should_invalidate {
            self.invalidate_visibility();
        }
    }

    fn play_transition(
        &self,
        from: Entity,
        to: Entity,
        transition: Option<Transition>,
        on_complete: Option<Arc<dyn Fn()>>,
    ) {
        match self.props.lock() {
            Ok(mut props) => {
                self.complete_transition();
                self.add(to);

                if let Some(transition) = transition {
                    props.occluded_scenes.push(from);

                    let transitor = Transitor::new(from, to, transition, on_complete);
                    transitor.init();

                    props.transitor = Some(transitor);
                } else {
                    if let Some(func) = on_complete {
                        func();
                    }
                    self.invalidate_visibility();
                }
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    pub fn width() -> f32 {
        match Self::global().props.lock() {
            Ok(props) => {
                if props.width < 0.0 {
                    System::stage().width() as f32
                } else {
                    props.width
                }
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }

    pub fn height() -> f32 {
        match Self::global().props.lock() {
            Ok(props) => {
                if props.height < 0.0 {
                    System::stage().height() as f32
                } else {
                    props.height
                }
            }
            Err(_) => {
                panic!("Context poisoned")
            }
        }
    }
}

// impl AsRef<Component> for Director {
//     fn as_ref(&self) -> &Component {
//         match self.props.read() {
//             Ok(props) => &props.inner,
//             Err(_) => {
//                 panic!("Context poisoned")
//             }
//         }
//     }
// }

#[derive(Default)] // Clone,
pub struct Transitor {
    from: Entity,
    to: Entity,
    transition: Transition,

    on_complete: Option<Arc<dyn Fn()>>,
}

impl Transitor {
    pub fn new(
        from: Entity,
        to: Entity,
        transition: Transition,
        on_complete: Option<Arc<dyn Fn()>>,
    ) -> Self {
        Self {
            from,
            to,
            transition,
            on_complete,
        }
    }

    pub fn init(&self) {
        // self.transition.init(self.from.clone(), self.to.clone());
    }

    pub fn update(&self, dt: f32) -> bool {
        self.transition.update(dt)
    }

    pub fn complete(&self) {
        self.transition.complete();
        if let Some(ref on_complete) = self.on_complete {
            (on_complete)();
        }
    }
}

unsafe impl Send for Transitor {}

impl fmt::Debug for Transitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Transitor")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}
