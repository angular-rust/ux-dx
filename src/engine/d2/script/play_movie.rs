use std::cell::RefCell;

use crate::engine::d2::{
    swf::{MoviePlayer, MovieSprite},
    Entity, EntityManager,
};

use super::Action;

struct PlayMovieProps {
    name: String,
    movie: Option<MovieSprite>,
}

/// An action that plays a movie once using the actor's MoviePlayer, completing when the movie
/// finishes.
pub struct PlayMovie {
    props: RefCell<PlayMovieProps>,
}

impl PlayMovie {
    /// @param The name of the movie to play.
    pub fn new(name: String) -> Self {
        Self {
            props: RefCell::new(PlayMovieProps { name, movie: None }),
        }
    }
}

impl Action for PlayMovie {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        let mut props = self.props.borrow_mut();
        if let Some(mut player) = EntityManager::<MoviePlayer>::get(actor) {
            match props.movie {
                Some(ref movie) => match player.movie {
                    Some(ref player_movie) => {
                        // if movie != player_movie.get() { // DV watching movie progress
                        //     self.movie = None;
                        //     return 0.0;
                        // }

                        if movie != player_movie {
                            // DV without watching movie progress
                            props.movie = None;
                            return 0.0;
                        }
                    }
                    None => {
                        props.movie = None;
                        return 0.0;
                    }
                },
                None => {
                    player.play(props.name.clone(), true);
                    // self.movie = player.movie.map(|v| v.get()); // DV watching movie progress
                    props.movie = player.movie; // DV without watching movie progress
                }
            }
        }

        -1.0 // Keep going
    }
}
