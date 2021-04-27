#![allow(dead_code)]
pub const TEX_SIZE: usize = 16;
pub const X_SIZE: usize = 16;
pub const Y_SIZE: usize = 16;
pub const Z_SIZE: usize = 16;
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
        return pos.x + (X_SIZE-1) * (pos.y + (Y_SIZE-1) * pos.z);
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

/// Macro for indexing into a 1D array using 3D coordinates.
macro_rules! IX {
    ( $x: expr, $y: expr,  $z: expr ) => {{
        $x as usize + X_SIZE * ($y as usize + Y_SIZE * $z as usize)
    }};
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

/// Add borders to the density grid. We can do this by just setting the density
/// of the outer cells to be equal to the density of the cells inset by 1. This
/// means the difference between the two cells is 0, so there will never be any
/// flow outwards.
/// # Params
/// * `b` - The type of border. 1 for vertical vel walls, 2 for hori vel walls, 0 for dens.
// fn set_borders<const N: usize>(grid: &mut [f32; N], b: u8) {
//     //HORIZONTAL WALLS
//     for ii in 0..X_SIZE {
//         for kk in 0..Z_SIZE {
//             let ix_top = IX!(ii, Y_SIZE - 1, kk);
//             let ix_top_inset = IX!(ii, Y_SIZE - 2, kk);
//             let ix_bot = IX!(ii, 0, kk);
//             let ix_bot_inset = IX!(ii, 1, kk);
//             grid[ix_top] = {
//                 if b == 2 {
//                     -grid[ix_top_inset]
//                 } else {
//                     grid[ix_top_inset]
//                 }
//             };
//             grid[ix_bot] = {
//                 if b == 2 {
//                     -grid[ix_bot_inset]
//                 } else {
//                     grid[ix_bot_inset]
//                 }
//             };
//         }
//     }
//     // SIDE WALLS
//     for jj in 0..Y_SIZE {
//         for kk in 0..Z_SIZE {
//             let ix_left = IX!(0, jj, kk);
//             let ix_left_inset = IX!(1, jj, kk);
//             let ix_right = IX!(X_SIZE - 1, jj, kk);
//             let ix_right_inset = IX!(X_SIZE - 2, jj, kk);
//             grid[ix_left] = {
//                 if b == 1 {
//                     -grid[ix_left_inset]
//                 } else {
//                     grid[ix_left_inset]
//                 }
//             };
//             grid[ix_right] = {
//                 if b == 1 {
//                     -grid[ix_right_inset]
//                 } else {
//                     grid[ix_right_inset]
//                 }
//             };
//         }
//     }
//     // BACK - FRONT WALLS
//     for ii in 0..X_SIZE {
//         for jj in 0..Y_SIZE {
//             let ix_front = IX!(ii, jj, 0);
//             let ix_front_inset = IX!(ii, jj, 1);
//             let ix_back = IX!(ii, jj, Z_SIZE - 1);
//             let ix_back_inset = IX!(ii, jj, Z_SIZE - 2);
//             grid[ix_front] = {
//                 if b == 3 {
//                     -grid[ix_front_inset]
//                 } else {
//                     grid[ix_front_inset]
//                 }
//             };
//             grid[ix_back] = {
//                 if b == 3 {
//                     -grid[ix_back_inset]
//                 } else {
//                     grid[ix_back_inset]
//                 }
//             };
//         }
//     }
//     grid[IX!(0, 0, Z_SIZE-1)] = 0.5 * (grid[IX!(1, 0, Z_SIZE-1)] + grid[IX!(0, 1, Z_SIZE-1)]);
//     grid[IX!(0, Y_SIZE - 2, Z_SIZE-1)] =
//         0.5 * (grid[IX!(1, Y_SIZE - 2, Z_SIZE-1)] + grid[IX!(0, Y_SIZE - 2, Z_SIZE-1)]);
//     grid[IX!(X_SIZE - 2, 0, Z_SIZE-1)] =
//         0.5 * (grid[IX!(X_SIZE - 2, 0, Z_SIZE-1)] + grid[IX!(X_SIZE - 2, 1, Z_SIZE-1)]);
//     grid[IX!(X_SIZE - 2, Y_SIZE - 2, Z_SIZE-1)] = 0.5
//         * (grid[IX!(X_SIZE - 3, Y_SIZE - 2, Z_SIZE-1)] + grid[IX!(X_SIZE - 2, Y_SIZE- 3, Z_SIZE-1)]);
// }

/// Align velocity with neighbors. diff is viscosity.
fn diffuse<const N: usize>(
    grid: &mut [f32; N],
    prev_grid: &mut [f32; N],
    dt: f32,
    viscosity: f32,
    borders: bool,
    b: u8,
) {
    let diff_rate = dt * viscosity * grid.len() as f32;
    let iterations = 1;
    if viscosity == 0.0 {
        return;
    }

    // For each cell we get contributions from all 6 direct neighbors
    for _ in 0..iterations {
        for ii in 1..X_SIZE - 1 {
            for jj in 1..Y_SIZE - 1 {
                for kk in 1..Z_SIZE - 1 {
                    let ix = IX!(ii, jj, kk);
                    let ix_up = IX!(ii, jj - 1, kk); // 1 row up
                    let ix_down = IX!(ii, jj + 1, kk); // 1 row down
                    let ix_back = IX!(ii, jj, kk - 1); // 1 row back
                    let ix_front = IX!(ii, jj, kk + 1); // 1 row front
                    grid[ix] = (prev_grid[ix]
                        + diff_rate
                            * (grid[ix - 1]
                                + grid[ix + 1]
                                + grid[ix_up]
                                + grid[ix_down]
                                + grid[ix_back]
                                + grid[ix_front]))
                        / (1.0 + 6.0 * diff_rate); // 6 is number of neighbors
                }
            }
        }
        if borders {
            //set_borders(grid, b)
        }
    }
}

/// Process density movement via velocity
/// # Params
/// * `b` - Border type, see set_borders
fn advect<const N: usize>(
    grid: &mut [f32; N],
    prev_grid: &[f32; N],
    vx_grid: &[f32; N],
    vy_grid: &[f32; N],
    vz_grid: &[f32; N],
    dt: f32,
    borders: bool,
    b: u8,
) {
    let dt0 = dt * X_SIZE as f32;

    for ii in 1..X_SIZE - 1 {
        for jj in 1..Y_SIZE - 1 {
            for kk in 1..Z_SIZE - 1 {
                let ix = IX!(ii, jj, kk);
                let mut x = ii as f32 - dt0 * vx_grid[ix];
                let mut y = jj as f32 - dt0 * vy_grid[ix];
                let mut z = kk as f32 - dt0 * vz_grid[ix];

                //X
                if x < 0.5 {
                    x = 0.5
                }
                if x > X_SIZE as f32 - 1.5 {
                    x = X_SIZE as f32 - 1.5
                }
                let ii0 = x as u32;
                let ii1 = ii0 + 1;

                //Y
                if y < 0.5 {
                    y = 0.5
                }

                if y > Y_SIZE as f32 - 1.5 {
                    y = Y_SIZE as f32 - 1.5
                }
                let jj0 = y as u32;
                let jj1 = jj0 + 1;

                //Z
                if z < 0.5 {
                    z = 0.5
                }

                if z > Z_SIZE as f32 - 1.5 {
                    z = Z_SIZE as f32 - 1.5
                }
                let kk0 = z as u32;
                let kk1 = kk0 + 1;

                let s1 = x - ii0 as f32;
                let s0 = 1.0 - s1;
                let t1 = y - jj0 as f32;
                let t0 = 1.0 - t1;
                let u1 = z - kk0 as f32;
                let u0 = 1.0 - u1;

                let ix000 = IX!(ii0, jj0, kk0);
                let ix010 = IX!(ii0, jj1, kk0);
                let ix110 = IX!(ii1, jj1, kk0);
                let ix100 = IX!(ii1, jj0, kk0);
                let ix001 = IX!(ii0, jj0, kk1);
                let ix011 = IX!(ii0, jj1, kk1);
                let ix111 = IX!(ii1, jj1, kk1);
                let ix101 = IX!(ii1, jj0, kk1);

                grid[ix] = s0
                    * (t0 * (u0 * prev_grid[ix000] + u1 * prev_grid[ix001])
                        + t1 * (u0 * prev_grid[ix010] + u1 * prev_grid[ix011]))
                    + s1 * (t0 * (u0 * prev_grid[ix100] + u1 * prev_grid[ix101])
                        + t1 * (u0 * prev_grid[ix110] + u1 * prev_grid[ix111]));
            }
        }
    }
    if borders {
        //set_borders(grid, b);
    }
}

// Forces velocity to be mass conserving
fn project<const N: usize>(
    vx_grid: &mut [f32; N],
    vy_grid: &mut [f32; N],
    vz_grid: &mut [f32; N],
    prev_x: &mut [f32; N],
    prev_y: &mut [f32; N],
    prev_z: &mut [f32; N],
    borders: bool,
) {
    for ii in 1..X_SIZE - 1 {
        for jj in 1..Y_SIZE - 1 {
            for kk in 1..Z_SIZE - 1 {
                let ix = IX!(ii, jj, kk);
                let ix_up = IX!(ii, jj - 1, kk); // 1 row up
                let ix_down = IX!(ii, jj + 1, kk); // 1 row down
                let ix_back = IX!(ii, jj, kk - 1); // 1 row back
                let ix_front = IX!(ii, jj, kk + 1); // 1 row front
                prev_y[ix] = -0.5
                    * Y_SIZE as f32
                    * (vx_grid[ix + 1] - vx_grid[ix - 1] + vy_grid[ix_down] - vy_grid[ix_up]
                        + vz_grid[ix_front]
                        - vz_grid[ix_back]);
                prev_x[ix] = 0.0;
            }
        }
    }
    if borders {
        //set_borders(prev_y, 0);
        //set_borders(prev_x, 0);
        //set_borders(prev_z, 0);
    }

    let iterations = 1;
    for _ in 0..iterations {
        for ii in 1..X_SIZE - 1 {
            for jj in 1..Y_SIZE - 1 {
                for kk in 1..Z_SIZE - 1 {
                    let ix = IX!(ii, jj, kk);
                    let ix_up = IX!(ii, jj - 1, kk); // 1 row up
                    let ix_down = IX!(ii, jj + 1, kk); // 1 row down
                    let ix_back = IX!(ii, jj, kk - 1); // 1 row back
                    let ix_front = IX!(ii, jj, kk + 1); // 1 row front
                    prev_x[ix] = (prev_y[ix]
                        + prev_x[ix - 1]
                        + prev_x[ix + 1]
                        + prev_x[ix_up]
                        + prev_x[ix_down]
                        + prev_x[ix_back]
                        + prev_x[ix_front])
                        / 6.0;
                }
            }
        }
        if borders {
            //set_borders(prev_x, 0)
        }
    }

    for ii in 1..X_SIZE - 1 {
        for jj in 1..Y_SIZE - 1 {
            for kk in 1..Z_SIZE - 1 {
                let ix = IX!(ii, jj, kk);
                let ix_up = IX!(ii, jj - 1, kk); // 1 row up
                let ix_down = IX!(ii, jj + 1, kk); // 1 row down
                let ix_back = IX!(ii, jj, kk - 1); // 1 row back
                let ix_front = IX!(ii, jj, kk + 1); // 1 row front
                vx_grid[ix] -= 0.5 * (prev_x[ix + 1] - prev_x[ix - 1]) / Y_SIZE as f32;
                vy_grid[ix] -= 0.5 * (prev_x[ix_down] - prev_x[ix_up]) / Y_SIZE as f32;
                vz_grid[ix] -= 0.5 * (prev_x[ix_front] - prev_x[ix_back]) / Y_SIZE as f32;
            }
        }
    }
    if borders {
        //set_borders(vx_grid, 1);
        //set_borders(vy_grid, 2);
        //set_borders(vz_grid, 3);
    }
}

/// Step density
fn step_dens<const N: usize>(
    dens_grid: &mut [f32; N],
    grid: &mut Grid<N>,
    dt: f32,
    diff: f32,
    borders: bool,
) {
    // Make a copy of the dens_grid
    // let mut prev_dens_grid = dens_grid.to_vec();
    // let prev_dens_grid = &mut prev_dens_grid[..];
    let prev_dens_grid = &mut dens_grid.clone();

    // Swap binding in preparation for the next swap
    let (prev_dens_grid, dens_grid) = (dens_grid, prev_dens_grid);

    // Process diffusion
    diffuse(dens_grid, prev_dens_grid, dt, diff, borders, 0);

    // Swap bindings, b/c we just updates dens_grid and advect() needs to use
    // that for the previous grid
    let (prev_dens_grid, dens_grid) = (dens_grid, prev_dens_grid);

    // Process velocity of particles
    advect(
        dens_grid,
        prev_dens_grid,
        grid.x_vel,
        grid.y_vel,
        grid.z_vel,
        dt,
        borders,
        0,
    );
}

/// Step velocity
fn step_vel<const N: usize>(
    vx_grid: &mut [f32; N],
    vy_grid: &mut [f32; N],
    vz_grid: &mut [f32; N],
    dt: f32,
    diff: f32,
    borders: bool,
) {
    println!("step_vel: cloning arrays");
    let prev_x = &mut vx_grid.clone();
    let prev_y = &mut vy_grid.clone();
    let prev_z = &mut vz_grid.clone();

    // Swap grids
    println!("step_vel: swapping grids");
    let (prev_x, vx_grid) = (vx_grid, prev_x);
    let (prev_y, vy_grid) = (vy_grid, prev_y);
    let (prev_z, vz_grid) = (vz_grid, prev_z);

    // Diffuse just like with density but with velocity instead
    println!("Starting Diffussion");
    diffuse(vx_grid, prev_x, dt, diff, borders, 1);

    diffuse(vy_grid, prev_y, dt, diff, borders, 2);
    diffuse(vz_grid, prev_z, dt, diff, borders, 3);

    project(vx_grid, vy_grid, vz_grid, prev_x, prev_y, prev_z, borders);

    // Swap grids
    let (prev_x, vx_grid) = (vx_grid, prev_x);
    let (prev_y, vy_grid) = (vy_grid, prev_y);
    let (prev_z, vz_grid) = (vz_grid, prev_z);

    // Advect just like with density
    advect(vx_grid, prev_x, prev_x, prev_y, prev_z, dt, borders, 1);
    advect(vy_grid, prev_y, prev_x, prev_y, prev_z, dt, borders, 2);
    advect(vz_grid, prev_z, prev_x, prev_y, prev_z, dt, borders, 2);

    project(vx_grid, vy_grid, vz_grid, prev_x, prev_y, prev_z, borders);
}

pub fn step_fluid<const N: usize>(
    dens_grid: &mut [f32; N],
    grid: &mut Grid<N>,
    dt: f32,
    viscosity: f32,
    borders: bool,
) {
    // Step density, alter density grid

    println!("Call step_dens");
    step_dens(dens_grid, grid, dt, viscosity, borders);

    println!("Call step_vel");
    println!("{:?}", grid.y_vel.len());
    //step_woot(grid.x_vel, grid.y_vel, grid.z_vel, dt, viscosity, borders);
    step_vel(grid.x_vel, grid.y_vel, grid.z_vel, dt, viscosity, borders);
}
