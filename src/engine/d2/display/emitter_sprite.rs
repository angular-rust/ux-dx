use std::rc::Rc;

use crate::engine::d2::{animation::AnimatedFloat, math::Math};

use super::{EmitterMold, EmitterType, Graphics, Sprite, Texture};

/// A sprite that displays a particle system.
#[derive(Default, Clone, Debug)]
pub struct EmitterSprite {
    pub inner: Sprite,
    /// The particle texture, must be square.
    pub texture: Option<Rc<dyn Texture>>,

    /// The current number of particles being shown.
    pub num_particles: usize,

    pub max_particles: i32,

    pub type_: EmitterType,

    /// How long the emitter should remain enabled, or <= 0 to never expire.
    pub duration: f32,

    /// Whether new particles are being actively emitted.
    pub enabled: bool, // = true;

    pub emit_x: AnimatedFloat,
    pub emit_x_variance: AnimatedFloat,

    pub emit_y: AnimatedFloat,
    pub emit_y_variance: AnimatedFloat,

    pub alpha_start: AnimatedFloat,
    pub alpha_start_variance: AnimatedFloat,

    pub alpha_end: AnimatedFloat,
    pub alpha_end_variance: AnimatedFloat,

    pub angle: AnimatedFloat,
    pub angle_variance: AnimatedFloat,

    pub gravity_x: AnimatedFloat,
    pub gravity_y: AnimatedFloat,

    pub max_radius: AnimatedFloat,
    pub max_radius_variance: AnimatedFloat,

    pub min_radius: AnimatedFloat,

    pub lifespan_variance: AnimatedFloat,
    pub lifespan: AnimatedFloat,

    pub rotate_per_second: AnimatedFloat,
    pub rotate_per_second_variance: AnimatedFloat,

    pub rotation_start: AnimatedFloat,
    pub rotation_start_variance: AnimatedFloat,

    pub rotation_end: AnimatedFloat,
    pub rotation_end_variance: AnimatedFloat,

    pub size_start: AnimatedFloat,
    pub size_start_variance: AnimatedFloat,

    pub size_end: AnimatedFloat,
    pub size_end_variance: AnimatedFloat,

    pub speed: AnimatedFloat,
    pub speed_variance: AnimatedFloat,

    pub radial_accel: AnimatedFloat,
    pub radial_accel_variance: AnimatedFloat,

    pub tangential_accel: AnimatedFloat,
    pub tangential_accel_variance: AnimatedFloat,

    // The particle pool
    particles: Vec<Particle>,

    // Time passed since the last emission
    emit_elapsed: f32,

    total_elapsed: f32,
}

impl EmitterSprite {
    pub fn new(mold: &EmitterMold) -> Self {
        let mut instance = Self {
            inner: Sprite::default(),

            texture: mold.texture.clone(),
            type_: mold.type_,

            alpha_end: AnimatedFloat::new(mold.alpha_end, None),
            alpha_end_variance: AnimatedFloat::new(mold.alpha_end_variance, None),
            alpha_start: AnimatedFloat::new(mold.alpha_start, None),
            alpha_start_variance: AnimatedFloat::new(mold.alpha_start_variance, None),
            angle: AnimatedFloat::new(mold.angle, None),
            angle_variance: AnimatedFloat::new(mold.angle_variance, None),
            duration: mold.duration,
            emit_x_variance: AnimatedFloat::new(mold.emit_x_variance, None),
            emit_y_variance: AnimatedFloat::new(mold.emit_y_variance, None),
            gravity_x: AnimatedFloat::new(mold.gravity_x, None),
            gravity_y: AnimatedFloat::new(mold.gravity_y, None),
            max_radius: AnimatedFloat::new(mold.max_radius, None),
            max_radius_variance: AnimatedFloat::new(mold.max_radius_variance, None),
            min_radius: AnimatedFloat::new(mold.min_radius, None),
            lifespan: AnimatedFloat::new(mold.lifespan, None),
            lifespan_variance: AnimatedFloat::new(mold.lifespan_variance, None),
            radial_accel: AnimatedFloat::new(mold.radial_accel, None),
            radial_accel_variance: AnimatedFloat::new(mold.radial_accel_variance, None),
            rotate_per_second: AnimatedFloat::new(mold.rotate_per_second, None),
            rotate_per_second_variance: AnimatedFloat::new(mold.rotate_per_second_variance, None),
            rotation_end: AnimatedFloat::new(mold.rotation_end, None),
            rotation_end_variance: AnimatedFloat::new(mold.rotation_end_variance, None),
            rotation_start: AnimatedFloat::new(mold.rotation_start, None),
            rotation_start_variance: AnimatedFloat::new(mold.rotation_start_variance, None),
            size_end: AnimatedFloat::new(mold.size_end, None),
            size_end_variance: AnimatedFloat::new(mold.size_end_variance, None),
            size_start: AnimatedFloat::new(mold.size_start, None),
            size_start_variance: AnimatedFloat::new(mold.size_start_variance, None),
            speed: AnimatedFloat::new(mold.speed, None),
            speed_variance: AnimatedFloat::new(mold.speed_variance, None),
            tangential_accel: AnimatedFloat::new(mold.tangential_accel, None),
            tangential_accel_variance: AnimatedFloat::new(mold.tangential_accel_variance, None),

            emit_x: AnimatedFloat::new(0.0, None),
            emit_y: AnimatedFloat::new(0.0, None),
            emit_elapsed: 0.0,
            particles: Vec::new(),
            total_elapsed: 0.0,
            enabled: true,
            max_particles: 0,
            num_particles: 0,
        };

        instance.inner.blend_mode = mold.blend_mode.unwrap_or_default();

        instance.particles = Vec::with_capacity(mold.max_particles);
        let mut idx = 0;
        let ll = instance.particles.len();
        while idx < ll {
            instance.particles[idx] = Particle::new();
            idx += 1;
        }

        instance
    }

    pub fn restart(&mut self) {
        self.enabled = true;
        self.total_elapsed = 0.0;
    }

    // override
    pub fn on_update(&mut self, dt: f32) {
        self.inner.on_update(dt);

        self.alpha_end.update(dt);
        self.alpha_end_variance.update(dt);
        self.alpha_start.update(dt);
        self.alpha_start_variance.update(dt);
        self.angle.update(dt);
        self.angle_variance.update(dt);
        self.emit_x.update(dt);
        self.emit_x_variance.update(dt);
        self.emit_y.update(dt);
        self.emit_y_variance.update(dt);
        self.gravity_x.update(dt);
        self.gravity_y.update(dt);
        self.lifespan.update(dt);
        self.lifespan_variance.update(dt);
        self.max_radius.update(dt);
        self.max_radius_variance.update(dt);
        self.min_radius.update(dt);
        self.radial_accel.update(dt);
        self.radial_accel_variance.update(dt);
        self.rotate_per_second.update(dt);
        self.rotate_per_second_variance.update(dt);
        self.rotation_end.update(dt);
        self.rotation_end_variance.update(dt);
        self.rotation_start.update(dt);
        self.rotation_start_variance.update(dt);
        self.size_end.update(dt);
        self.size_end_variance.update(dt);
        self.size_start.update(dt);
        self.size_start_variance.update(dt);
        self.speed.update(dt);
        self.speed_variance.update(dt);
        self.tangential_accel.update(dt);
        self.tangential_accel_variance.update(dt);

        // Update existing particles
        let gravity_type = self.type_ == EmitterType::Gravity;
        let mut idx = 0;
        while idx < self.num_particles {
            let mut particle = self.particles[idx];
            if particle.life > dt {
                if gravity_type {
                    particle.x += particle.vel_x * dt;
                    particle.y += particle.vel_y * dt;

                    let mut accel_x = self.gravity_x.get();
                    let mut accel_y = -self.gravity_y.get();

                    if particle.radial_accel != 0.0 || particle.tangential_accel != 0.0 {
                        let dx = particle.x - particle.emit_x;
                        let dy = particle.y - particle.emit_y;
                        let distance = (dx * dx + dy * dy).sqrt();

                        // Apply radial force
                        let radial_x = dx / distance;
                        let radial_y = dy / distance;
                        accel_x += radial_x * particle.radial_accel;
                        accel_y += radial_y * particle.radial_accel;

                        // Apply tangential force
                        let tangential_x = -radial_y;
                        let tangential_y = radial_x;
                        accel_x += tangential_x * particle.tangential_accel;
                        accel_y += tangential_y * particle.tangential_accel;
                    }

                    particle.vel_x += accel_x * dt;
                    particle.vel_y += accel_y * dt;
                } else {
                    particle.radial_rotation += particle.vel_radial_rotation * dt;
                    particle.radial_radius += particle.vel_radial_radius * dt;

                    let radius = particle.radial_radius;
                    particle.x = self.emit_x.get() - particle.radial_rotation.cos() * radius;
                    particle.y = self.emit_y.get() - particle.radial_rotation.sin() * radius;

                    if radius < self.min_radius.get() {
                        particle.life = 0.0; // Kill it
                    }
                }

                particle.scale += particle.vel_scale * dt;
                particle.rotation += particle.vel_rotation * dt;
                particle.alpha += particle.vel_alpha * dt;

                particle.life -= dt;
                idx += 1;
            } else {
                // Kill it, and swap it with the last living particle, so that alive particles are
                // packed to the front of the pool
                self.num_particles -= 1;
                if idx != self.num_particles {
                    self.particles[idx] = self.particles[self.num_particles];
                    self.particles[self.num_particles] = particle;
                }
            }
        }

        // Check whether we should continue to the emit step
        if !self.enabled {
            return;
        }

        if self.duration > 0.0 {
            self.total_elapsed += dt;
            if self.total_elapsed >= self.duration {
                self.enabled = false;
                return;
            }
        }

        // Emit new particles
        let emit_delay = self.lifespan.get() / self.particles.len() as f32;
        self.emit_elapsed += dt;
        while self.emit_elapsed >= emit_delay {
            if self.num_particles < self.particles.len() {
                let mut particle = self.particles[self.num_particles];
                if self.init_particle(&mut particle) {
                    self.num_particles += 1;
                }
            }
            self.emit_elapsed -= emit_delay;
        }
    }

    // override
    pub fn draw(&self, gfx: &Box<dyn Graphics>) {
        if let Some(ref texture) = self.texture {
            // Assumes that the texture is always square
            let offset = -texture.width() as f32 / 2.0;

            let mut idx = 0;
            let ll = self.num_particles;
            while idx < ll {
                let particle = self.particles[idx];
                gfx.save();
                gfx.translate(particle.x, particle.y);

                if particle.alpha < 1.0 {
                    gfx.multiply_alpha(particle.alpha);
                }

                if particle.rotation != 0.0 {
                    gfx.rotate(particle.rotation);
                }

                if particle.scale != 1.0 {
                    gfx.scale(particle.scale, particle.scale);
                }

                gfx.draw_texture(texture, offset, offset);
                gfx.restore();

                idx += 1;
            }
        } else {
            log::warn!("no texture for particle emmiter");
        }
    }

    fn init_particle(&mut self, particle: &mut Particle) -> bool {
        particle.life = Self::random(self.lifespan.get(), self.lifespan_variance.get());
        if particle.life <= 0.0 {
            return false; // Dead on arrival
        }

        // Don't include the variance here
        particle.emit_x = self.emit_x.get();
        particle.emit_y = self.emit_y.get();

        let angle = -Math::to_radians(Self::random(self.angle.get(), self.angle_variance.get()));
        let speed = Self::random(self.speed.get(), self.speed_variance.get());
        particle.vel_x = speed * angle.cos();
        particle.vel_y = speed * angle.sin();

        particle.radial_accel = Self::random(self.radial_accel.get(), self.radial_accel_variance.get());
        particle.tangential_accel = Self::random(self.tangential_accel.get(), self.tangential_accel_variance.get());

        particle.radial_radius = Self::random(self.max_radius.get(), self.max_radius_variance.get());
        particle.vel_radial_radius = -particle.radial_radius / particle.life;
        particle.radial_rotation = angle;
        particle.vel_radial_rotation =
            Math::to_radians(Self::random(self.rotate_per_second.get(), self.rotate_per_second_variance.get()));

        if self.type_ == EmitterType::Gravity {
            particle.x = Self::random(self.emit_x.get(), self.emit_x_variance.get());
            particle.y = Self::random(self.emit_y.get(), self.emit_y_variance.get());
        } else {
            // type == Radial
            let radius = particle.radial_radius;
            particle.x = self.emit_x.get() - particle.radial_rotation.cos() * radius;
            particle.y = self.emit_y.get() - particle.radial_rotation.sin() * radius;
        }

        // Assumes that the texture is always square
        let width = self.texture.as_ref().unwrap().width() as f32;
        let scale_start = Self::random(self.size_start.get(), self.size_start_variance.get()) / width;
        let scale_end = Self::random(self.size_end.get(), self.size_end_variance.get()) / width;
        particle.scale = scale_start;
        particle.vel_scale = (scale_end - scale_start) / particle.life;

        let rotation_start = Self::random(self.rotation_start.get(), self.rotation_start_variance.get());
        let rotation_end = Self::random(self.rotation_end.get(), self.rotation_end_variance.get());
        particle.rotation = rotation_start;
        particle.vel_rotation = (rotation_end - rotation_start) / particle.life;

        let alpha_start = Self::random(self.alpha_start.get(), self.alpha_start_variance.get());
        let alpha_end = Self::random(self.alpha_end.get(), self.alpha_end_variance.get());
        particle.alpha = alpha_start;
        particle.vel_alpha = (alpha_end - alpha_start) / particle.life;

        true
    }

    #[inline]
    fn max_particles(&self) -> usize {
        self.particles.len()
    }

    fn set_max_particles(&mut self, max_particles: usize) {
        // Grow the pool
        let mut old_length = self.particles.len();
        self.particles.resize(max_particles, Default::default());
        while old_length < max_particles {
            self.particles[old_length] = Particle::new();
            old_length += 1;
        }

        if self.num_particles > max_particles {
            self.num_particles = max_particles;
        }
    }

    // static
    fn random(base: f32, variance: f32) -> f32 {
        if variance != 0.0 {
            return base + variance * (2.0 * rand::random::<f32>() - 1.0);
        }

        base
    }
}

impl AsRef<Sprite> for EmitterSprite {
    fn as_ref(&self) -> &Sprite {
        &self.inner
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Particle {
    // Where the emitter was when the particle was spawned
    pub emit_x: f32,
    pub emit_y: f32,

    pub x: f32,
    pub vel_x: f32,

    pub y: f32,
    pub vel_y: f32,

    pub radial_radius: f32,
    pub vel_radial_radius: f32,

    pub radial_rotation: f32,
    pub vel_radial_rotation: f32,

    pub radial_accel: f32,
    pub tangential_accel: f32,

    pub scale: f32,
    pub vel_scale: f32,

    pub rotation: f32,
    pub vel_rotation: f32,

    pub alpha: f32,
    pub vel_alpha: f32,

    pub life: f32,
}

impl Particle {
    pub fn new() -> Self {
        Default::default()
    }
}
