use super::{blend, ArrayView3D, Vec3};
use core::{cmp, num::Float};

extern "C" {
    fn sqrtf(x: f32) -> f32;
    fn floorf(x: f32) -> f32;
    fn ceilf(x: f32) -> f32;
}

pub unsafe fn sphere(
    voxels: &mut ArrayView3D,
    coord: Vec3<f32>,
    radius: f32,
    falloff: f32,
    value: f32,
    operation: blend::Operation,
) {
    let (nx, ny, nz) = voxels.dim;

    let min_coord = coord - radius - falloff;
    let (min_x, min_y, min_z) = (
        floorf(min_coord.x).max(0.0) as usize,
        floorf(min_coord.y).max(0.0) as usize,
        floorf(min_coord.z).max(0.0) as usize,
    );

    let max_coord = coord + radius + falloff;
    let (max_x, max_y, max_z) = (
        cmp::min(nx, ceilf(max_coord.x) as usize),
        cmp::min(ny, ceilf(max_coord.y) as usize),
        cmp::min(nz, ceilf(max_coord.z) as usize),
    );

    for x in min_x..max_x {
        let delta_x = x as f32 - coord.x;
        let delta_x_squared = delta_x * delta_x;

        for y in min_y..max_y {
            let delta_y = y as f32 - coord.y;
            let delta_y_squared = delta_y * delta_y;
            let sum = delta_x_squared + delta_y_squared;

            for z in min_z..max_z {
                let delta_z = z as f32 - coord.z;
                let delta_z_squared = delta_z * delta_z;
                let sum = sum + delta_z_squared;
                let distance = sqrtf(sum);

                if distance < radius {
                    let cell = voxels.get_mut((x, y, z)).unwrap();
                    let value = blend::execute(cell.clone(), value.clone(), operation);
                    *cell = value;
                } else if distance < radius + falloff {
                    let factor = 1.0 - (distance - radius) / falloff;
                    let value = factor * value.clone();
                    let cell = voxels.get_mut((x, y, z)).unwrap();
                    let value = blend::execute(cell.clone(), value, operation);
                    *cell = value;
                }
            }
        }
    }
}
