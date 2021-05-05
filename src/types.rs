#![allow(dead_code)]
pub const X_SIZE: usize = 96;
pub const Y_SIZE: usize = 96;
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
    pub fn add_density_source(&mut self, pos: Pos, dens: f32) {
        let index = Self::get_index(pos.x as usize, pos.y as usize, pos.z as usize);
        self.density[index] = dens;
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
