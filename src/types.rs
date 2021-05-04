#![allow(dead_code)]
pub const X_SIZE: usize = 256;
pub const Y_SIZE: usize = 256;
pub const Z_SIZE: usize = 4;
pub const SIZE_1D: usize = (X_SIZE+2)*(Y_SIZE+2)*(Z_SIZE+2);


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

#[derive(Clone, Debug, PartialEq)]
pub struct WindGrid {
    pub x_vel: Box<[f32]>,
    pub y_vel: Box<[f32]>,
    pub z_vel: Box<[f32]>,
    pub density: Box<[f32]>,
}

impl WindGrid {
    pub fn add_velocity_source(&mut self, pos: Pos, vel: Vel) {
        let p_x = pos.x as usize;
        let p_y = pos.y as usize;
        let p_z = pos.z as usize;

        let index = Self::get_index(p_x, p_y, p_z);

        self.x_vel[index] = vel.x;
        self.y_vel[index] = vel.y;
        self.z_vel[index] = vel.z;
    }

    pub fn get_index(x: usize, y: usize, z: usize) -> usize {
        x + (X_SIZE + 2) * (y + (Y_SIZE + 2) * z)
    }

    pub fn get_velocity(&self, pos: Pos) -> Vel {
        let p_x = pos.x as usize;
        let p_y = pos.y as usize;
        let p_z = pos.z as usize;

        let index = Self::get_index(p_x, p_y, p_z);
        let x = self.x_vel[index] as f32;
        let y = self.y_vel[index] as f32;
        let z = self.y_vel[index] as f32;

        Vel { x, y, z }
    }
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
        return pos.x + (X_SIZE + 2) * (pos.y + (Y_SIZE + 2) * pos.z);
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
