use crate::ext::error::{GameError, GameResult};
use winit::monitor::{MonitorHandle, VideoMode};
use winit::window::Fullscreen;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum FullscreenMode {
    Exclusive,
    Borderless,
}

impl FullscreenMode {
    pub(crate) fn from_raw(fullscreen: Fullscreen) -> Self {
        match fullscreen {
            Fullscreen::Exclusive(_) => Self::Exclusive,
            Fullscreen::Borderless(_) => Self::Borderless,
        }
    }

    pub(crate) fn into_raw(self, monitor: Option<MonitorHandle>) -> GameResult<Fullscreen> {
        match self {
            Self::Exclusive => get_preferred_video_mode(monitor)
                .map(|video_mode| Fullscreen::Exclusive(video_mode)),
            Self::Borderless => Ok(Fullscreen::Borderless(monitor)),
        }
    }
}

fn get_preferred_video_mode(monitor: Option<MonitorHandle>) -> GameResult<VideoMode> {
    let mut preferred_video_mode: Option<VideoMode> = None;
    if let Some(monitor) = monitor {
        for (_, video_mode) in monitor.video_modes().enumerate() {
            if let Some(current_video_mode) = &preferred_video_mode {
                let current_size = current_video_mode.size();
                let size = video_mode.size();
                if current_size.width * current_size.height < size.width * size.height {
                    preferred_video_mode = Some(video_mode);
                }
            } else {
                preferred_video_mode = Some(video_mode);
            }
        }
    }
    preferred_video_mode
        .ok_or_else(|| GameError::NotSupportedError("no available video mode".into()))
}