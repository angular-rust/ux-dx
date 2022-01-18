use std::rc::Rc;

use crate::engine::d2::asset::Asset;

use super::Playback;

/// A loaded sound file.
pub trait Sound: Asset {
    /// The length of the sound in seconds.
    fn duration(&self) -> f32;

    /// Plays the sound once, suitable for one-shot sound effects.
    ///  *
    /// @param volume The playback volume between 0 (silence) and 1 (full volume). Defaults to 1.
    /// @returns A playback that can be used to control the sound.
    // volume: f32 = 1.0
    fn play(&self, volume: f32) -> Rc<dyn Playback>;

    /// Loops the sound forever, suitable for background music.
    ///  *
    /// @param volume The playback volume between 0 (silence) and 1 (full volume). Defaults to 1.
    /// @returns A playback that can be used to control the sound.
    // volume: f32 = 1.0
    fn play_with_loop(&self, volume: f32) -> Rc<dyn Playback>;
}
