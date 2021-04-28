#![allow(dead_code)]
pub const TEX_SIZE: usize = 256;
pub const X_SIZE: usize = 256;
pub const Y_SIZE: usize = 256;
pub const Z_SIZE: usize = 4;
pub const SIZE: [usize; 3] = [X_SIZE, Y_SIZE, Z_SIZE];

#[derive(Clone, Copy, Default)]
pub struct Vel {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct Pos {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub struct Grid<'a, const M: usize> {
    pub x_vel: &'a mut [f32; M],
    pub y_vel: &'a mut [f32; M],
    pub z_vel: &'a mut [f32; M],
}

impl<'a, const M: usize> Grid<'a, M> {
    pub fn new(
        vx_grid: &'a mut [f32; M],
        vy_grid: &'a mut [f32; M],
        vz_grid: &'a mut [f32; M],
    ) -> Self {
        // Velocity grids
        Self {
            x_vel: vx_grid,
            y_vel: vy_grid,
            z_vel: vz_grid,
        }
    }
    pub fn get_index(&self, pos: Pos) -> usize {
        return pos.x + (X_SIZE - 1) * (pos.y + (Y_SIZE - 1) * pos.z);
    }

    pub fn get(&self, pos: Pos) -> Vel {
        let index = self.get_index(pos);
        let x = self.x_vel[index];
        let y = self.y_vel[index];
        let z = self.y_vel[index];
        Vel { x, y, z }
    }
    pub fn add_velocity_source(&mut self, pos: Pos, vel: Vel) {
        let index = self.get_index(pos);
        self.x_vel[index] = vel.x;
        self.y_vel[index] = vel.y;
        self.z_vel[index] = vel.z;
    }
    pub fn arrays(&'a self) -> (&'a [f32; M], &'a [f32; M], &'a [f32; M]) {
        (&self.x_vel, &self.y_vel, &self.z_vel)
    }
}

/// A source of fluid.
pub struct Source {
    /// The index of the source in the grid array
    pub ix: usize,
    /// Value between 0 and 1, density of the fluid at source.
    pub density: f32,
}

impl Source {
    /// Create a new source, given an index into a float array for its position.
    pub fn new(ix: usize, density: f32) -> Source {
        Source { ix, density }
    }
}
