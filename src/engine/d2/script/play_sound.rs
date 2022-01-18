use std::{cell::RefCell, rc::Rc};

use crate::engine::d2::{
    sound::{Playback, Sound},
    Entity,
};

use super::Action;

struct PlaySoundProps {
    sound: Box<dyn Sound>,
    volume: f32,
    playback: Option<Rc<dyn Playback>>,
}

/// An action that plays a sound and waits for it to complete.
///
// ```
// script.run(Sequence::new([
//     // Play a sound
//     PlaySound::new(sound1),
//
//     // Then wait 2 seconds
//     Delay::new(2),
//
//     // Then play another sound
//     PlaySound::new(sound2),
// ]));
// ```
pub struct PlaySound {
    props: RefCell<PlaySoundProps>,
}

impl PlaySound {
    /// @param sound The sound to play.
    /// @param volume The volume to pass to `Sound.play`.
    //  volume: Option<f32> = 1.0
    pub fn new(sound: Box<dyn Sound>, volume: Option<f32>) -> Self {
        Self {
            props: RefCell::new(PlaySoundProps {
                sound,
                volume: volume.unwrap_or(1.0),
                playback: None,
            }),
        }
    }
}

impl Action for PlaySound {
    fn update(&self, dt: f32, actor: &mut Entity) -> f32 {
        let mut props = self.props.borrow_mut();
        let volume = props.volume;
        if props.playback.is_none() {
            props.playback = Some(props.sound.play(volume));
        }

        if let Some(ref playback) = props.playback {
            if *playback.complete().get() {
                props.playback = None;
                return 0.0; // Finished
            }
        }

        -1.0 // Keep waiting
    }
}
