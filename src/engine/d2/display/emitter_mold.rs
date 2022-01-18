use std::rc::Rc;

use crate::engine::d2::asset::{AssetPack, File};

use super::{BlendMode, EmitterSprite, Texture};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmitterType {
    Gravity,
    Radial,
}

impl Default for EmitterType {
    fn default() -> Self {
        Self::Gravity
    }
}

/// A particle system configuration, that can be used to create emitter sprites. The configuration is
/// loaded from a .pex file, authored in a tool such as Particle Designer.
///  *
/// NOTE: There are some restrictions to keep in mind when using Particle Designer:
///  *
/// - Particle coloring is not supported.
/// - Only normal and additive blend modes are supported.
///  *
///  *
/// Also keep in mind that gratuitous particle systems are a great way to kill performance,
/// especially on mobile. Try to keep maxParticles as low as possible to achieve the desired effect.

#[derive(Default, Debug)]
pub struct EmitterMold {
    pub texture: Option<Rc<dyn Texture>>,

    pub max_particles: usize,

    pub type_: EmitterType,

    // pub emitX: f32,
    pub emit_x_variance: f32,

    // pub emitY: f32,
    pub emit_y_variance: f32,

    pub alpha_start: f32,
    pub alpha_start_variance: f32,

    pub alpha_end: f32,
    pub alpha_end_variance: f32,

    pub angle: f32,
    pub angle_variance: f32,

    pub duration: f32,

    pub gravity_x: f32,
    pub gravity_y: f32,

    pub max_radius: f32,
    pub max_radius_variance: f32,

    pub min_radius: f32,

    pub lifespan_variance: f32,
    pub lifespan: f32,

    pub rotate_per_second: f32,
    pub rotate_per_second_variance: f32,

    pub rotation_start: f32,
    pub rotation_start_variance: f32,

    pub rotation_end: f32,
    pub rotation_end_variance: f32,

    pub size_start: f32,
    pub size_start_variance: f32,

    pub size_end: f32,
    pub size_end_variance: f32,

    pub speed: f32,
    pub speed_variance: f32,

    pub radial_accel: f32,
    pub radial_accel_variance: f32,

    pub tangential_accel: f32,
    pub tangential_accel_variance: f32,

    pub blend_mode: Option<BlendMode>,

    file: Option<Box<dyn File>>,
}

impl EmitterMold {
    /// Creates an EmitterMold using files in an asset pack.
    /// @param name The path to the particle system within the asset pack, excluding the .pex suffix.
    pub fn new(pack: impl AssetPack, name: String) -> Self {
        let instance = Self::default();

        // let _file = pack.get_file(name + ".pex");
        // let xml = Xml::parse(_file.toString());

        // let blend_funcSource = 0;
        // let blend_funcDestination = 0;

        // // The basename of the pex file's path, where we'll find the textures
        // let idx = name.rfind("/");
        // let basePath = if idx >= 0 { name[..idx + 1] } else { "" };

        // for element in xml.firstElement().elements() {
        //     match element.nodeName.to_lowercase() {
        //         "texture" => instance.texture = pack.get_texture(basePath + element.get("name").remove_file_extension()),
        //         "angle" => instance.angle = Self::getValue(element),
        //         "anglevariance" => instance.angleVariance = Self::getValue(element),
        //         "blendfuncdestination" => blend_funcDestination = Self::getValue(element) as usize,
        //         "blendfuncsource" => blend_funcSource = Self::getValue(element) as usize,
        //         "duration" => instance.duration = Self::getValue(element),
        //         "emittertype" => {
        //             instance.type_ = if Self::getValue(element) as usize == 0 { EmitterType::Gravity } else { EmitterType::Radial };
        //         }
        //         "finishcolor" => instance.alphaEnd = Self::getFloat(element, "alpha"),
        //         "finishcolorvariance" => instance.alphaEndVariance = Self::getFloat(element, "alpha"),
        //         "finishparticlesize" => instance.sizeEnd = Self::getValue(element),
        //         "finishparticlesizevariance" => instance.sizeEndVariance = Self::getValue(element),
        //         "gravity" => {
        //             instance.gravityX = Self::getX(element);
        //             instance.gravityY = Self::getY(element);
        //         }
        //         "maxparticles" => instance.maxParticles = Self::getValue(element) as usize,
        //         "maxradius" => instance.maxRadius = Self::getValue(element),
        //         "maxradiusvariance" => instance.maxRadiusVariance = Self::getValue(element),
        //         "minradius" => instance.minRadius = Self::getValue(element),
        //         "particlelifespan" => instance.lifespan = Self::getValue(element),
        //         "particlelifespanvariance" => instance.lifespanVariance = Self::getValue(element),
        //         "radialaccelvariance" => instance.radialAccelVariance = Self::getValue(element),
        //         "radialacceleration" => instance.radialAccel = Self::getValue(element),
        //         "rotatepersecond" => instance.rotatePerSecond = Self::getValue(element),
        //         "rotatepersecondvariance" => instance.rotatePerSecondVariance = Self::getValue(element),
        //         "rotationend" => instance.rotationEnd = Self::getValue(element),
        //         "rotationendvariance" => instance.rotationEndVariance = Self::getValue(element),
        //         "rotationstart" => instance.rotation_start = Self::getValue(element),
        //         "rotationstartvariance" => instance.rotation_startVariance = Self::getValue(element),
        //         // "sourceposition" =>
        //         "sourcepositionvariance" => {
        //             instance.emitXVariance = Self::getX(element);
        //             instance.emitYVariance = Self::getY(element);
        //         }
        //         "speed" => instance.speed = Self::getValue(element),
        //         "speedvariance" => instance.speedVariance = Self::getValue(element),
        //         "startcolor" => instance.alphaStart = Self::getFloat(element, "alpha"),
        //         "startcolorvariance" => instance.alphaStartVariance = Self::getFloat(element, "alpha"),
        //         "startparticlesize" => instance.sizeStart = Self::getValue(element),
        //         "startparticlesizevariance" => instance.sizeStartVariance = Self::getValue(element),
        //         "tangentialaccelvariance" => instance.tangentialAccelVariance = Self::getValue(element),
        //         "tangentialacceleration" => instance.tangentialAccel = Self::getValue(element),
        //     }
        // }

        // // Handle weird Particle Designer output for emitters with a duration
        // if instance.lifespan <= 0 {
        //     instance.lifespan = instance.duration;
        // }

        // if blend_funcSource == 1 && blend_funcDestination == 1 {
        //     instance.blendMode = BlendMode::Add;
        // } else if blend_funcSource == 1 && blend_funcDestination == 771 {
        //     instance.blendMode = None; // Normal
        // } else if blend_funcSource != 0 || blend_funcDestination != 0 {
        //     // log::warn!(
        //     //     "Unsupported particle blend functions [emitter: {}, source: {}, dest {}"), name, blend_funcSource, blend_funcDestination)
        //     // );
        // }

        // instance
        unimplemented!()
    }

    /// Disposes the source .pex File used to create this EmitterMold. This can free up some memory,
    /// if you don't intend to recreate this EmitterMold later from the same AssetPack.
    ///  *
    /// @returns This instance, for chaining.
    pub fn dispose_files(&mut self) -> &Self {
        if let Some(ref mut file) = self.file {
            file.dispose();
        }

        self
    }

    /// Creates a new EmitterSprite using this mold.
    pub fn create_emitter(&self) -> EmitterSprite {
        EmitterSprite::new(self)
    }

    // // static
    // fn getFloat(xml: Xml, name: String) -> f32 {
    //     xml.get(name).parse::<f32>()
    // }

    // // static
    // #[inline]
    // fn getValue(xml: Xml) -> f32 {
    //     Self::getFloat(xml, "value")
    // }

    // // static
    // #[inline]
    // fn getX(xml: Xml) -> f32 {
    //     Self::getFloat(xml, "x")
    // }

    // // static
    // #[inline]
    // fn getY(xml: Xml) -> f32 {
    //     Self::getFloat(xml, "y")
    // }
}
