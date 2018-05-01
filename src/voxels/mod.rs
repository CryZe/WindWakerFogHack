use libtww::prelude::*;

pub mod blend;
pub mod marching_cubes;
pub mod sphere;

pub struct ArrayView3D {
    pub data: Vec<f32>,
    pub dim: (usize, usize, usize),
}

impl ArrayView3D {
    pub fn new((x, y, z): (usize, usize, usize)) -> Self {
        Self {
            data: vec![1.0; x * y * z],
            dim: (x, y, z),
        }
    }

    pub fn get(&self, (x, y, z): (usize, usize, usize)) -> Option<&f32> {
        let (nx, ny, nz) = self.dim;
        if x < nx && y < ny && z < nz {
            Some(&self.data[z + y * nz + x * ny * nz])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, (x, y, z): (usize, usize, usize)) -> Option<&mut f32> {
        let (nx, ny, nz) = self.dim;
        if x < nx && y < ny && z < nz {
            Some(&mut self.data[z + y * nz + x * ny * nz])
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Self {
        Self { a, b, c }
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub coord: Vec3<f32>,
}

impl Vertex {
    pub fn new(coord: Vec3<f32>) -> Self {
        Self { coord }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

use core::ops::{Add, Sub};

impl<T: Add> Add for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Add + Copy> Add<T> for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn add(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T: Sub> Sub for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Sub + Copy> Sub<T> for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn sub(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}
