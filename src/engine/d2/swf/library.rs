#![allow(unused_imports)]
use std::{collections::HashMap, rc::Rc};

use crate::engine::d2::{
    asset::{AssetPack, File},
    display::Sprite,
    swf::Format,
};

use super::{BitmapSymbol, Flipbook, KeyframeFormat, MovieFormat, MovieSprite, MovieSymbol, Symbol};

/// An exported Flump library containing movies and bitmaps.
#[derive(Default, Clone, Debug)]
pub struct Library {
    /// The original frame rate of movies in this library.
    pub frame_rate: f32,

    file: Option<Rc<dyn File>>,
    symbols: HashMap<String, Rc<dyn Symbol<MovieSprite>>>,
}

impl Library {
    /// Creates a library using files in an AssetPack.
    /// @param baseDir The directory in the pack containing Flump's library.json and texture atlases.
    pub fn new(pack: impl AssetPack, base_dir: String) -> Self {
        let instance = Self {
            file: pack.file(base_dir + "/library.json", true),
            symbols: HashMap::new(),
            frame_rate: 0.0,
        };

        // let json: Format = Json::parse(instance._file.toString());
        // instance.frameRate = json.frameRate;

        // let movies = Vec::new();
        // for movieObject in json.movies {
        //     let movie = MovieSymbol::new(instance, movieObject);
        //     movies.push(movie);
        //     instance._symbols.set(movie.name, movie);
        // }

        // let groups = json.textureGroups;
        // if groups[0].scaleFactor != 1 || groups.len() > 1 {
        //     log::warn!("Dx doesn't support Additional Scale Factors. Use Base Scales and load from different asset packs instead." );
        // }
        // let atlases = groups[0].atlases;
        // for atlasObject in atlases {
        //     let atlas = pack.get_texture(baseDir + "/" + atlasObject.file.removeFileExtension());
        //     for textureObject in atlasObject.textures {
        //         let bitmap = BitmapSymbol::new(textureObject, atlas);
        //         instance._symbols.set(bitmap.name, bitmap);
        //     }
        // }

        // // Now that all symbols have been parsed, go through keyframes and resolve references
        // for movie in movies {
        //     for layer in movie.layers {
        //         let keyframes = layer.keyframes;
        //         let ll = keyframes.len();
        //         for ii in 0..ll {
        //             let kf = keyframes[ii];
        //             if kf.symbolName.is_some() {
        //                 let symbol = instance._symbols.get(kf.symbolName);
        //                 assert!(symbol.is_some());
        //                 kf.setSymbol(symbol);
        //             }

        //             // Specially handle "stop frames". These are one-frame keyframes that preceed an
        //             // invisible or empty keyframe. Dx animates at 60 FPS, which can cause unexpected motion/flickering as those
        //             // one-frame keyframes are interpolated. So, assume that these frames are never
        //             // meant to actually be displayed and hide them.
        //             if kf.tweened && kf.duration == 1 && ii + 1 < ll {
        //                 let nextKf = keyframes[ii + 1];
        //                 if !nextKf.visible || nextKf.symbolName.is_none() {
        //                     kf.setVisible(false);
        //                 }
        //             }
        //         }
        //     }
        // }
        instance
    }

    /// Creates a library procedurally using a set of Flipbook definitions. Each flipbook will be
    /// converted to a movie that can be instanciated with `createMovie()`.
    ///  *
    // ```
    // let lib = Library.fromFlipbooks([
    //     // A walk animation from a 5x3 sprite sheet, that lasts 5 seconds
    //     Flipbook::new("walk", spriteSheet.split(5, 3)).set_duration(5).set_anchor(10, 10),
    //  *
    //     // Another animation where each frame comes from a separate image (Jump1.png, Jump2.png, ..)
    //     Flipbook::new("jump", [for (frame in 1..10) pack.get_texture("Jump"+frame)]),
    // ]);
    // let movie = lib.createMovie("walk");
    // ```
    // static
    pub fn from_flipbooks(flipbooks: Vec<Flipbook>) -> Library {
        // let lib = Type::createEmptyInstance(Library);
        // lib._symbols = HashMap::new();
        // lib.frameRate = 60;
        // lib._file = None;

        // for flipbook in flipbooks {
        //     // Fake up some Flump metadata to create a movie symbol
        //     let keyframes: Vec<KeyframeFormat> = Vec::new();
        //     for frame in flipbook.frames {
        //         // keyframes.push({
        //         //     duration: frame.duration*lib.frameRate,
        //         //     label: frame.label,
        //         //     pivot: [frame.anchorX, frame.anchorY],
        //         //     ref_: "", // Hack so that this keyframe doesn't get marked as empty
        //         // });
        //     }
        //     let movie = MovieSymbol::new(
        //         lib, MovieFormat::default()//,
        //             // {
        //             //     id: flipbook.name,
        //             //     layers: [{
        //             //         name: "flipbook",
        //             //         flipbook: true,
        //             //         keyframes: keyframes,
        //             //     }],
        //             // }
        //     );
        //     lib._symbols.set(flipbook.name, movie);

        //     // Assign symbols at each keyframe
        //     let keyframes = movie.layers[0].keyframes;
        //     for ii in 0..flipbook.frames.len() {
        //         keyframes[ii].setSymbol(flipbook.frames[ii].toSymbol());
        //     }
        // }

        // lib
        unimplemented!()
    }

    /// Disposes the source library.json File used to create this Library. This can free up some
    /// memory, if you don't intend to recreate this Library later from the same AssetPack.
    ///  *
    /// @returns This instance, for chaining.
    pub fn dispose_files(&self) -> &Self {
        if let Some(ref file) = self.file {
            file.dispose();
        }

        self
    }

    /// Retrieve a name symbol from this library, or None if not found.
    #[inline]
    pub fn symbol(&self, symbol_name: String) -> Option<&Rc<dyn Symbol<MovieSprite>>> {
        self.symbols.get(&symbol_name)
    }

    /// Creates a sprite from a symbol name, it'll either be a movie or a bitmap.
    /// @param required If true and the symbol is not in this library, an error is thrown.
    //  required :bool = true
    pub fn create_sprite(&self, symbol_name: String, required: bool) -> Option<Sprite> {
        // if let Some(symbol) = self.symbols.get(&symbolName) {
        //     Some(symbol.create_sprite())
        // } else {
        //     if required {
        //         panic!("Missing symbol {}", symbolName);
        //     }
        //     return None;
        // }
        unimplemented!()
    }

    /// Creates a movie sprite from a symbol name.
    /// @param required If true and the symbol is not in this library, an error is thrown.
    //  required :bool = true
    #[inline]
    pub fn create_movie(&self, symbol_name: String, required: bool) -> Option<MovieSprite> {
        // self.create_sprite(symbolName, required)
        unimplemented!()
    }

    /// Iterates over all the symbols in this library.
    // #[inline]
    // pub fn iter(&self) -> impl Iterator<Item = (&'_ String, &'_ Box<dyn Symbol<MovieSprite>>)> {
    //     self.symbols.iter()
    // }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Rc<dyn Symbol<MovieSprite>>)> + '_ {
        self.symbols.iter()
    }
}
