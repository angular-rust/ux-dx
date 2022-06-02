use bytes::Bytes;
use std::{fmt, rc::Rc};

use crate::engine::d2::{
    asset::Asset,
    display::{Graphics, SubTexture, Texture},
    util::{Disposable, Value},
};

use super::{BasicAsset, TextureRoot};

#[derive(Default, Clone, Debug)]
pub struct BasicTexture<R: TextureRoot + fmt::Debug> {
    // DV
    pub inner: BasicAsset<BasicTexture<R>>,
    pub graphics: Option<Rc<dyn Graphics>>,

    // pub parent: Option<Box<dyn Texture>>,
    pub root: R,

    // The global position of this texture from the root
    pub root_x: i32,
    pub root_y: i32,

    parent: Option<Box<BasicTexture<R>>>,

    // The texture bounds
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl<R: TextureRoot + fmt::Debug> AsRef<BasicAsset<BasicTexture<R>>> for BasicTexture<R> {
    fn as_ref(&self) -> &BasicAsset<BasicTexture<R>> {
        &self.inner
    }
}

impl<R: TextureRoot + fmt::Debug> BasicTexture<R> {
    fn new(root: R, width: i32, height: i32) -> Self {
        Self {
            inner: BasicAsset::new(),
            root,
            width,
            height,
            graphics: None,
            parent: None,
            root_x: 0,
            root_y: 0,
            x: 0,
            y: 0,
        }
    }

    // override
    fn copy_from(&self, other: BasicTexture<R>) {
        // self.inner.disposed = false;
        // self.inner.copy_from(other.root);

        // self.width = other.width;
        // self.height = other.height;

        // // We don't support mutability on the position, since it invalidates the rootX/Y of child
        // // subTextures. This assert should never fail with the asset reloader.
        // assert!(self.root_x == other.root_x && self.root_y == other.root_y && self.x == other.x && self.y == other.y);
        unimplemented!()
    }

    // override
    fn on_disposed(&mut self) {
        // Since subtextures shouldn't be able to accidently dispose their parents and siblings,
        // only dispose the root if this is the top-most subtexture.
        if self.parent.is_none() {
            self.root.dispose();
        }
    }
}

impl<R: TextureRoot + fmt::Debug> SubTexture for BasicTexture<R> {
    #[inline]
    fn parent(&self) -> Option<Box<dyn Texture>> {
        // self.parent
        unimplemented!()
    }

    #[inline]
    fn x(&self) -> i32 {
        self.x
    }

    #[inline]
    fn y(&self) -> i32 {
        self.y
    }
}

impl<R: TextureRoot + fmt::Debug> Texture for BasicTexture<R> {
    // FIXME -> Different textures that share the same root also share the same Graphics. This
    // is an unfortunate limitation imposed by Canvas2D only allowing one context per canvas. We'll
    // need to document this caveat somewhere, or change this API to workaround.
    fn graphics(&self) -> Box<dyn Graphics> {
        self.root.graphics()
    }

    #[inline]
    fn height(&self) -> i32 {
        self.height
    }

    #[inline]
    fn width(&self) -> i32 {
        self.width
    }

    fn read_pixels(&self, x: i32, y: i32, width: i32, height: i32) -> Bytes {
        self.root
            .read_pixels(self.root_x + x, self.root_y + y, width, height)
    }

    // tilesHigh: i32 = 1
    fn split(&self, tiles_wide: i32, tiles_high: i32) -> Vec<Rc<dyn SubTexture>> {
        let mut tiles = Vec::new();
        let tile_width = (self.width / tiles_wide) as i32;
        let tile_height = (self.height / tiles_high) as i32;
        for y in 0..tiles_high {
            for x in 0..tiles_wide {
                tiles.push(self.sub_texture(
                    x * tile_width,
                    y * tile_height,
                    tile_width,
                    tile_height,
                ));
            }
        }

        tiles
    }

    fn sub_texture(&self, x: i32, y: i32, width: i32, height: i32) -> Rc<dyn SubTexture> {
        // let sub: BasicTexture<R> = self.root.create_texture(width, height);
        // sub.parent = self;
        // sub.x = x;
        // sub.y = y;
        // sub.root_x = self.root_x + x;
        // sub.root_y = self.root_y + y;

        // sub
        unimplemented!()
    }

    fn write_pixels(&self, pixels: Bytes, x: i32, y: i32, source_w: i32, source_h: i32) {
        self.root
            .write_pixels(pixels, self.root_x + x, self.root_y + y, source_w, source_h);
    }
}

impl<R: TextureRoot + fmt::Debug> Asset for BasicTexture<R> {
    fn reload_count(&self) -> usize {
        // Delegate to the root, so that root reloads get propogated to all subtextures
        // self.root.get_reload_count()
        unimplemented!()
    }
}

impl<R: TextureRoot + fmt::Debug> Disposable for BasicTexture<R> {
    fn dispose(&self) {
        // FIXME:
    }
}
