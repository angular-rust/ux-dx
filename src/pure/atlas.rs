use super::{PixelFormat, Texture};

pub enum AtlasFlags {
  ClearTexture     = (1 << 0),
  DisableMigration = (1 << 1)
}

pub struct AtlasRepositionData {
//   // The current user data for this texture */
//   void *user_data;
//   // The old and new positions of the texture */
//   RectangleMapEntry old_position;
//   RectangleMapEntry new_position;
}

pub struct GetRectanglesData {
//   AtlasRepositionData *textures;
//   // Number of textures found so far */
//   unsigned int n_textures;
}

pub struct Atlas {
//   Object _parent;

//   RectangleMap *map;

  texture: Option<Texture>,
  texture_format: PixelFormat,
  flags: AtlasFlags,

//   AtlasUpdatePositionCallback update_position_cb;

//   GHookList pre_reorganize_callbacks;
//   GHookList post_reorganize_callbacks;
}

impl Atlas {
    fn new(texture_format: PixelFormat, flags: AtlasFlags /* AtlasUpdatePositionCallback update_position_cb */) -> Self {
        // Atlas *atlas = g_new (Atlas, 1);

        // atlas->update_position_cb = update_position_cb;
        // atlas->map = NULL;
        // atlas->texture = NULL;
        // atlas->flags = flags;
        // atlas->texture_format = texture_format;
        // g_hook_list_init (&atlas->pre_reorganize_callbacks, sizeof (GHook));
        // g_hook_list_init (&atlas->post_reorganize_callbacks, sizeof (GHook));

        // return _atlas_object_new (atlas);
        unimplemented!()
    }
}