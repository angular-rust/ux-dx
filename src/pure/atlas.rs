pub enum AtlasFlags {
  ClearTexture     = (1 << 0),
  DisableMigration = (1 << 1)
}

pub struct AtlasRepositionData {
//   // The current user data for this texture */
//   void *user_data;
//   // The old and new positions of the texture */
//   CoglRectangleMapEntry old_position;
//   CoglRectangleMapEntry new_position;
}

pub struct GetRectanglesData {
//   CoglAtlasRepositionData *textures;
//   // Number of textures found so far */
//   unsigned int n_textures;
}

pub struct Atlas {
//   CoglObject _parent;

//   CoglRectangleMap *map;

//   CoglTexture *texture;
//   CoglPixelFormat texture_format;
//   CoglAtlasFlags flags;

//   CoglAtlasUpdatePositionCallback update_position_cb;

//   GHookList pre_reorganize_callbacks;
//   GHookList post_reorganize_callbacks;
}

impl Atlas {
    fn new(texture_format: PixelFormat, flags: AtlasFlags /* CoglAtlasUpdatePositionCallback update_position_cb */) -> Self {
        // CoglAtlas *atlas = g_new (CoglAtlas, 1);

        // atlas->update_position_cb = update_position_cb;
        // atlas->map = NULL;
        // atlas->texture = NULL;
        // atlas->flags = flags;
        // atlas->texture_format = texture_format;
        // g_hook_list_init (&atlas->pre_reorganize_callbacks, sizeof (GHook));
        // g_hook_list_init (&atlas->post_reorganize_callbacks, sizeof (GHook));

        // return _cogl_atlas_object_new (atlas);
    }
}