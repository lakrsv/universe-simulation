const SOLAR_MASS:f32 = 1.989e+30_f32;

pub const SUN: CelestialBodyConfig = CelestialBodyConfig {
    mass: SOLAR_MASS,
    radius: 696340000.0, // meters
    angular_velocity: 2.0e-7_f32, // rad/s
};

pub const SAGITTARIUS_A_STAR: CelestialBodyConfig = CelestialBodyConfig {
    mass: 4_100_000.0 * SOLAR_MASS,
    radius: 1.2e+7_f32,
    angular_velocity: 0.0, // TODO: What is the rad/s of Sagittarius A*?
};

pub struct CelestialBodyConfig {
    pub mass: f32,
    pub radius: f32,
    pub angular_velocity: f32,
}