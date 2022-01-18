use std::{collections::HashMap, fmt, rc::Rc};

use crate::engine::d2::{
    swf::{Library, MovieSprite},
    util::{Disposable, Value},
    Component, Entity,
};

use super::Symbol;

/// A convenient controller to play though multiple different movies. Designed for characters and
/// objects that have a separate Flump symbol for each of their animations, and need to switch
/// between them. The played movies will be added to a new child entity of the owner.

// TODO: should implement value for movie.
// Aka watch the movie sprite
#[derive(Default, Clone)]
pub struct MoviePlayer {
    pub inner: Component,
    /// The movie currently being shown.
    pub movie: Option<MovieSprite>, // Option<Value<MovieSprite>>,

    /// Whether the current movie is being looped.
    pub looping: bool,

    library: Library,
    root: Entity,

    oneshot_sprite: Option<MovieSprite>,
    looping_sprite: Option<MovieSprite>,

    decorator: Option<Rc<dyn Fn(MovieSprite)>>,
    cache: Option<HashMap<String, MovieSprite>>,
}

impl MoviePlayer {
    pub fn new(lib: Library) -> Self {
        Self {
            inner: Default::default(),
            movie: None,
            looping: false,
            library: lib,
            root: Entity::new(),
            looping_sprite: None,
            oneshot_sprite: None,
            decorator: None,
            cache: Some(HashMap::new()),
        }
    }

    /// Configures whether this MoviePlayer will keep a cache of all its MovieSprites, rather than
    /// creating a new instance for each play. This makes switching movies faster, at the expense of
    /// memory. By default, the cache is enabled. If this MoviePlayer plays lots of different movies,
    /// but doesn't match through them too often, consider disabling the cache.
    /// @returns This instance, for chaining.
    pub fn set_cache(&mut self, cache: bool) -> &Self {
        self.cache = if cache { Some(HashMap::new()) } else { None };

        self
    }

    /// Configures the callback used to decorate newly created MovieSprites, if any. This can be used
    /// to dress up avatars or other custom initialization.
    /// @returns This instance, for chaining.
    pub fn set_decorator(&mut self, decorator: Option<Rc<dyn Fn(MovieSprite)>>) -> &Self {
        self.decorator = decorator;

        self
    }

    /// Shows a movie that plays once. When it completes, the last looping movie is returned to. It
    /// is an error to call this without starting a loop() first.
    /// @param name The symbol name of the movie to play.
    /// @param restart If this movie is already being played, whether it will restart it from the
    ///   beginning.
    /// @returns This instance, for chaining.
    // restart :bool = true
    pub fn play(&mut self, name: String, restart: bool) -> &Self {
        assert!(self.looping_sprite.is_some(), "A loop must be started before the first call to play()");

        let should_update = self
            .oneshot_sprite
            .as_ref()
            .map(|sprite| sprite.symbol.name().map(|x| x != name).unwrap_or_default())
            .unwrap_or_default();

        if restart || should_update {
            self.oneshot_sprite = self.play_from_cache(name);
        }

        self
    }

    /// Shows a movie that loops forever.
    /// @param name The symbol name of the movie to loop.
    /// @param restart If this movie is already being looped, whether it will restart it from the
    ///   beginning.
    /// @returns This instance, for chaining.
    // restart :bool = true
    pub fn play_with_loop(&mut self, name: String, restart: bool) -> &Self {
        let should_update = self
            .looping_sprite
            .as_ref()
            .map(|sprite| sprite.symbol.name().map(|x| x != name).unwrap_or_default())
            .unwrap_or_default();

        if restart || should_update {
            self.oneshot_sprite = None;
            self.looping_sprite = self.play_from_cache(name);
        }

        self
    }

    // override
    pub fn on_added(&self) {
        if let Some(owner) = self.inner.owner {
            owner.entity().add_child(self.root.clone(), true, None);
        }
    }

    // override
    pub fn on_removed(&mut self) {
        self.root.dispose();
        self.oneshot_sprite = None;
        self.looping_sprite = None;
        self.movie = None;
    }

    // override
    pub fn on_update(&mut self, dt: f32) {
        // If this update would end the oneshot movie, replace it with the looping movie
        if let Some(ref oneshot_sprite) = self.oneshot_sprite {
            if oneshot_sprite.position() + dt > oneshot_sprite.symbol.duration {
                self.oneshot_sprite = None;
                self.set_current(self.looping_sprite.clone());
            }
        }
    }

    fn play_from_cache(&mut self, name: String) -> Option<MovieSprite> {
        match self.cache {
            Some(ref mut cache) => {
                match cache.get_mut(&name) {
                    Some(sprite) => {
                        // Rewind it
                        sprite.set_position(0.0);

                        // set_current
                        self.root.add(sprite.inner.component.clone());
                        self.movie = Some(sprite.clone());

                        return Some(sprite.clone());
                    }
                    None => {
                        // Not in the cache, create the new entry

                        if let Some(ref sprite) = self.library.create_movie(name.clone(), true) {
                            if let Some(ref decorator) = self.decorator {
                                (decorator)(sprite.clone());
                            }

                            cache.insert(name, sprite.clone());
                            self.set_current(Some(sprite.clone()));
                            Some(sprite.clone())
                        } else {
                            None
                        }
                    }
                }
            }
            None => {
                // Caching disabled, create a new movie each time
                self.create_movie(name).map(|ref sprite| {
                    self.set_current(Some(sprite.clone()));
                    sprite.clone()
                })
            }
        }
    }

    fn create_movie(&self, name: String) -> Option<MovieSprite> {
        // let sprite = self.library.create_movie(name, true);
        // if let Some(ref decorator) = self.decorator {
        //     (decorator)(sprite);
        // }

        // sprite
        unimplemented!()
    }

    fn looping(&self) -> bool {
        self.oneshot_sprite.is_none() && self.looping_sprite.is_some()
    }

    fn set_current(&mut self, current: Option<MovieSprite>) {
        if let Some(ref sprite) = current {
            self.root.add(sprite.inner.component.clone());
        }

        self.movie = current;
    }
}

impl PartialEq for MoviePlayer {
    fn eq(&self, other: &Self) -> bool {
        self.root == self.root
    }
}

impl fmt::Debug for MoviePlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MoviePlayer")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}

impl AsRef<Component> for MoviePlayer {
    fn as_ref(&self) -> &Component {
        &self.inner
    }
}
