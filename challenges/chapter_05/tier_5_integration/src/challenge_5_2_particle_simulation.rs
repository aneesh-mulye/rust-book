// Challenge 5.2 - Particle Simulation
//
// Define `Vec2` and `Particle` with methods described in the prompt.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn scale(&self, factor: f64) -> Vec2 {
        Vec2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Particle {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, mass: f64) -> Particle {
        Particle {
            position: Vec2 { x, y },
            velocity: Vec2 { x: vx, y: vy },
            mass,
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.position = self.position.add(&self.velocity.scale(dt));
    }

    pub fn kinetic_energy(&self) -> f64 {
        let svmag = self.velocity.magnitude();
        0.5 * self.mass * svmag * svmag
    }

    pub fn distance_to(&self, other: &Particle) -> f64 {
        self.position.add(&other.position.scale(-1.0)).magnitude()
    }
}

pub fn simulate_steps(
    mut p1: Particle,
    mut p2: Particle,
    steps: u32,
    dt: f64,
) -> Vec<(Vec2, Vec2, f64)> {
    let mut posns: Vec<(Vec2, Vec2, f64)> = Vec::new();
    for _ in 0..steps {
        p1.step(dt);
        p2.step(dt);
        posns.push((p1.position, p2.position, p1.distance_to(&p2)));
    }
    posns
}

// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .

#[cfg(test)]
mod tests {
    use super::{Particle, Vec2, simulate_steps};

    #[test]
    fn vec2_math_is_correct() {
        let a = Vec2::new(3.0, 4.0);
        assert!(
            (a.x - 3.0).abs() < 1e-12 && (a.y - 4.0).abs() < 1e-12,
            "Vec2::new(3,4) should set x=3, y=4. Got {:?}.",
            a
        );
        assert!(
            (a.magnitude() - 5.0).abs() < 1e-12,
            "Magnitude of (3,4) should be 5. Got {}.",
            a.magnitude()
        );

        let b = Vec2::new(1.0, 2.0);
        let sum = a.add(&b);
        assert_eq!(
            sum,
            Vec2 { x: 4.0, y: 6.0 },
            "Vec addition result incorrect. Expected (4,6), got {:?}.",
            sum
        );

        let scaled = b.scale(2.5);
        assert_eq!(
            scaled,
            Vec2 { x: 2.5, y: 5.0 },
            "Vec scaling result incorrect. Expected (2.5,5), got {:?}.",
            scaled
        );
    }

    #[test]
    fn particle_step_updates_position_by_velocity_times_dt() {
        let mut p = Particle::new(0.0, 0.0, 10.0, -5.0, 2.0);
        p.step(0.1);

        assert_eq!(
            p.position,
            Vec2 { x: 1.0, y: -0.5 },
            "After dt=0.1 with velocity (10,-5), expected position (1,-0.5). Got {:?}.",
            p.position
        );
    }

    #[test]
    fn kinetic_energy_and_distance_are_computed() {
        let p1 = Particle::new(0.0, 0.0, 3.0, 4.0, 2.0);
        let p2 = Particle::new(6.0, 8.0, 0.0, 0.0, 1.0);

        assert!(
            (p1.kinetic_energy() - 25.0).abs() < 1e-10,
            "KE should be 0.5 * 2 * 5^2 = 25. Got {}.",
            p1.kinetic_energy()
        );
        assert!(
            (p1.distance_to(&p2) - 10.0).abs() < 1e-10,
            "Distance between (0,0) and (6,8) should be 10. Got {}.",
            p1.distance_to(&p2)
        );
    }

    #[test]
    fn simulation_outputs_one_record_per_step() {
        let p1 = Particle::new(0.0, 0.0, 1.0, 0.0, 1.0);
        let p2 = Particle::new(0.0, 1.0, 0.0, -1.0, 1.0);
        let trace = simulate_steps(p1, p2, 5, 0.1);

        assert_eq!(
            trace.len(),
            5,
            "simulate_steps should produce one record per step. Expected 5, got {}.",
            trace.len()
        );
    }
}
