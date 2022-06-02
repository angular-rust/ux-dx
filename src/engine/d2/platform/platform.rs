use crate::engine::d2::{
    asset::{AssetPack, Manifest},
    subsystem::*,
    util::Promise,
};

use super::WampClient;

pub trait Platform {
    fn init(&self);

    fn external(&self) -> Box<dyn ExternalSystem>;
    fn keyboard(&self) -> Box<dyn KeyboardSystem>;
    fn motion(&self) -> Box<dyn MotionSystem>;
    fn mouse(&self) -> Box<dyn MouseSystem>;
    fn pointer(&self) -> Box<dyn PointerSystem>;
    fn renderer(&self) -> Box<dyn RendererSystem>;
    fn stage(&self) -> Box<dyn StageSystem>;
    fn storage(&self) -> Box<dyn StorageSystem>;
    fn touch(&self) -> Box<dyn TouchSystem>;
    fn web(&self) -> Box<dyn WebSystem>;

    fn load_asset_pack(&self, manifest: Manifest) -> Promise<Box<dyn AssetPack>>;
    fn wamp_client(&self) -> Option<WampClient>;

    fn locale(&self) -> Option<String>;
    fn time(&self) -> f32;
}
