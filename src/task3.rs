use std::ops::{Add, AddAssign, Div};



const G: f32 = 6.67430e-11;


fn task3() {
    let mut particle_vec = vec![
        Particle { position: FVec2::new(1., 1.), velocity: FVec2::new(0., 0.), mass: 100. },
        Particle { position: FVec2::new(1., -1.), velocity: FVec2::new(0., 0.), mass: 100. },
        Particle { position: FVec2::new(-1., -1.), velocity: FVec2::new(0., 0.), mass: 100. },
        Particle { position: FVec2::new(-1., 1.), velocity: FVec2::new(0., 0.), mass: 100. },
    ];

    // Calculate gravitational acceleration for each particle in the particle_vec
    for particle in &mut particle_vec {

        let mut total_force = FVec2::new(0., 0.);
        for other in &particle_vec {  // Cant iterate over particle_vec since we already have a mutable reference to it
            if particle == other { continue }
            total_force += calculate_force_components(particle, other);
        }

        let acceleration = total_force/particle.mass;
        particle.velocity += acceleration;
    }
}


/// Calculate the gravitational force p2 exerts on p1
fn calculate_force_components(p1: &Particle, p2: &Particle) -> FVec2 {
    let dx = p2.position.x - p1.position.x;
    let dy = p2.position.y - p1.position.y;
    let distance_squared = dx * dx + dy * dy;
    let distance = distance_squared.sqrt();
    let force_magnitude = (p1.mass * p2.mass) / distance_squared;
    let fx = force_magnitude * (dx / distance);
    let fy = force_magnitude * (dy / distance);
    FVec2::new(fx, fy)
}





// Boilerplate implementations of Particle and FVec2. Not important in the context of the compilation problem

#[derive(PartialEq)]
struct Particle {
    position: FVec2,
    velocity: FVec2,
    mass: f32,
}

#[derive(PartialEq)]
struct FVec2 {
    pub x: f32,
    pub y: f32
}
impl FVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl Add for FVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x+rhs.x, y: self.y+rhs.y }
    }
}
impl AddAssign for FVec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Div<f32> for FVec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self { x: self.x/rhs, y: self.y/rhs }
    }
}