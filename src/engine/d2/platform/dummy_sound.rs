use std::rc::Rc;

use crate::engine::d2::{
    animation::AnimatedFloat,
    asset::Asset,
    sound::{Playback, Sound},
    util::{Disposable, Value},
};

use super::BasicAsset;

/// An empty sound used in environments that don't support audio.
#[derive(Clone)]
pub struct DummySound {
    pub inner: Rc<BasicAsset<DummySound>>,
    // static
    // _instance: DummySound,
    playback: Option<DummyPlayback>,
}

impl DummySound {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(BasicAsset::<DummySound>::new()),
            playback: None,
        }
    }

    // override
    fn copy_from(&self, asset: DummySound) {
        // Nothing at all
    }

    // override
    fn on_disposed(&self) {
        // Nothing at all
    }

    // static
    pub fn instance() -> DummySound {
        // if _instance.is_none() {
        //     _instance = DummySound::new();
        // }

        // _instance
        unimplemented!()
    }
}

impl AsRef<BasicAsset<DummySound>> for DummySound {
    fn as_ref(&self) -> &BasicAsset<DummySound> {
        self.inner.as_ref()
    }
}

impl Sound for DummySound {
    // volume: f32 = 1.0
    fn play(&self, volume: f32) -> Rc<dyn Playback> {
        Rc::new(DummyPlayback::new(Rc::new(self.clone())))
    }

    // volume: f32 = 1.0
    fn play_with_loop(&self, volume: f32) -> Rc<dyn Playback> {
        Rc::new(DummyPlayback::new(Rc::new(self.clone())))
    }

    fn duration(&self) -> f32 {
        0.0
    }
}

impl Asset for DummySound {
    fn reload_count(&self) -> usize {
        self.inner.reload_count()
    }
}

impl Disposable for DummySound {
    fn dispose(&self) {
        self.inner.dispose()
    }
}

// This should be immutable too
#[derive(Clone)]
pub struct DummyPlayback {
    pub volume: AnimatedFloat,
    sound: Rc<dyn Sound>,
    complete: Value<bool>,
}

impl DummyPlayback {
    pub fn new(sound: Rc<dyn Sound>) -> Self {
        Self {
            sound,
            volume: AnimatedFloat::new(0.0, None), // A little quirky? All DummyPlaybacks share the same volume
            complete: Value::<bool>::new(true, None),
        }
    }

    pub fn set_paused(&self, paused: bool) {}
}

impl Playback for DummyPlayback {
    fn sound(&self) -> Rc<dyn Sound> {
        self.sound.clone()
    }

    fn volume(&self) -> AnimatedFloat {
        self.volume.clone()
    }

    fn paused(&self) -> bool {
        true
    }

    fn complete(&self) -> Value<bool> {
        self.complete.clone()
    }

    fn position(&self) -> f32 {
        0.0
    }

    fn set_complete(&self, val: Value<bool>) {
        todo!()
    }

    fn set_paused(&self, val: bool) {
        todo!()
    }

    fn set_volume(&self, val: AnimatedFloat) {
        todo!()
    }

    fn set_position(&self, pos: f32) {
        todo!()
    }
}

impl Disposable for DummyPlayback {
    fn dispose(&self) {
        // Nothing
    }
}
