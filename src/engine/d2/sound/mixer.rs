use std::{cell::RefCell, fmt, rc::Rc};

use crate::engine::d2::{
    asset::Asset,
    platform::DummyPlayback,
    util::{Disposable, Value},
    Component,
};

use super::{Playback, Sound};

/// An easy way to manage the lifecycle of multiple sounds. A handle is created from a source sound
/// using `createSound()`, and all handle playbacks will be stopped when the Mixer is disposed.
pub struct Mixer {
    pub inner: Component,
    sounds: Vec<MixerSound>,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            inner: Component::default(),
            sounds: Vec::new(),
        }
    }

    /// Creates a sound handle from a source sound. Playbacks created using the handle will be
    /// stopped when this Mixer is disposed.
    ///  *
    /// @param channels The maximum number of times this sound should be able to play at once.
    //  channels: i32 = Math::INT_MAX
    pub fn create_sound(&mut self, source: Rc<dyn Sound>, channels: usize) -> impl Sound {
        let sound = MixerSound::new(source, channels);
        self.sounds.push(sound.clone());

        sound
    }

    /// Stop all the playbacks belonging to this Mixer.
    pub fn stop_all(&mut self) {
        for sound in self.sounds.iter_mut() {
            sound.dispose();
        }
    }

    // override
    pub fn on_removed(&mut self) {
        self.stop_all();
        self.sounds.clear();
    }
}

impl AsRef<Component> for Mixer {
    fn as_ref(&self) -> &Component {
        &self.inner
    }
}

struct MixerSoundProps {
    source: Rc<dyn Sound>,
    channels: usize,
    playbacks: Vec<Rc<dyn Playback>>,
}

#[derive(Clone)]
pub struct MixerSound {
    props: Rc<RefCell<MixerSoundProps>>,
}

impl MixerSound {
    pub fn new(source: Rc<dyn Sound>, channels: usize) -> Self {
        Self {
            props: Rc::new(RefCell::new(MixerSoundProps {
                source,
                channels,
                playbacks: Vec::new(),
            })),
        }
    }

    fn play_or_loop(&self, volume: f32, is_loop: bool) -> Rc<dyn Playback> {
        match self.find_open_channel() {
            Some(channel) => {
                let mut props = self.props.borrow_mut();
                let playback = if is_loop {
                    props.source.play_with_loop(volume)
                } else {
                    props.source.play(volume)
                };

                props.playbacks.insert(channel, playback.clone());

                playback
            }
            None => {
                // No channels remaining, don't play anything
                Rc::new(DummyPlayback::new(Rc::new(self.clone())))
            }
        }
    }

    fn find_open_channel(&self) -> Option<usize> {
        let props = self.props.borrow();
        for idx in 0..props.channels {
            match props.playbacks.get(idx) {
                Some(playback) => {
                    if *playback.complete().get() {
                        return Some(idx);
                    }
                }
                None => {
                    return Some(idx);
                }
            }
        }

        None
    }
}

impl Sound for MixerSound {
    fn duration(&self) -> f32 {
        let props = self.props.borrow();
        props.source.duration()
    }

    // volume: f32 = 1.0
    fn play(&self, volume: f32) -> Rc<dyn Playback> {
        self.play_or_loop(volume, false)
    }

    // volume: f32 = 1.0
    fn play_with_loop(&self, volume: f32) -> Rc<dyn Playback> {
        self.play_or_loop(volume, true)
    }
}

impl Asset for MixerSound {
    fn reload_count(&self) -> usize {
        let props = self.props.borrow();
        props.source.reload_count()
    }
}

impl Disposable for MixerSound {
    fn dispose(&self) {
        let mut props = self.props.borrow_mut();
        for playback in props.playbacks.iter() {
            playback.dispose();
        }
        props.playbacks.clear();
    }
}

impl fmt::Debug for MixerSound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MixerSound")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}
