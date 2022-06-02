use crate::engine::d2::{display::Graphics, input::Key, System};

use super::OverdrawGraphics;

/// Internal helper for shared debug logic across all platforms.
pub struct DebugLogic {
    // platform: Box<dyn Platform>,
    /// The normal Graphics saved by toggleOverdrawGraphics so it can restored.
    saved_graphics: Option<Box<dyn Graphics>>,
}

impl DebugLogic {
    pub fn init() {
        let instance = Self {
            // platform: Box::new(platform),
            saved_graphics: None,
        };

        System::keyboard().down_signal().connect(
            Box::new(move |event| {
                if let Some(key) = event.key {
                    if key == Key::O && System::keyboard().is_down(Key::Control) {
                        if instance.toggle_overdraw_graphics() {
                            log::info!(
                                "Enabled overdraw visualizer, press Ctrl-O again to disable"
                            );
                        }
                    }
                }
            }),
            false,
        );

        // instance
    }

    /// Toggles the overdraw debug renderer.
    /// @return Whether the overdraw renderer was enabled.
    pub fn toggle_overdraw_graphics(&self) -> bool {
        // let renderer = self.platform.renderer();

        if self.saved_graphics.is_some() {
            //     renderer.set_graphics(self.saved_graphics);
            //     self.saved_graphics = None;
            // } else if renderer.graphics().is_some() {
            //     self.saved_graphics = renderer.get_graphics();
            //     renderer.set_graphics(OverdrawGraphics::new(self.saved_graphics));
            //     return true;
        }

        false
    }
}
