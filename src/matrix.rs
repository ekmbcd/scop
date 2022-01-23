#![allow(dead_code)]

use std::{ops, f32::consts::PI};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    array: [f32; 16]
}

#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    pub array: [f32; 4]
}

impl Vector4 {
    fn new(array: [f32; 4]) -> Self {
        Self {array}
    }

    fn x(&self) -> f32 {
        self.array[0]
    }

    fn y(&self) -> f32 {
        self.array[1]
    }

    fn z(&self) -> f32 {
        self.array[2]
    }

    fn w(&self) -> f32 {
        self.array[3]
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.array.as_ptr()
    }
}

impl ops::Mul<Vector4> for Vector4 {
    type Output = f32;

    fn mul(self, rhs: Vector4) -> f32 {
        self.x() * rhs.x() 
        + self.y() * rhs.y() 
        + self.z() * rhs.z() 
        + self.w() * rhs.w()
    }
}

impl ops::Mul<Vector4> for &[f32] {
    type Output = f32;

    fn mul(self, rhs: Vector4) -> f32 {
        self[0] * rhs.x() 
        + self[1] * rhs.y() 
        + self[2] * rhs.z() 
        + self[3] * rhs.w()
    }
}


impl Matrix4 {
    pub fn new(array: [f32; 16]) -> Self {
        Self { 
            array 
        }
    }

    pub fn from_vec4(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
        Self::new([
            x.x(), x.y(), x.z(), x.w(),
            y.x(), y.y(), y.z(), y.w(),
            z.x(), z.y(), z.z(), z.w(),
            w.x(), w.y(), w.z(), w.w(),
        ]) 
    }

    pub fn x(&self) -> &[f32] {
        &self.array[0..4]
    }

    pub fn y(&self) -> &[f32] {
        &self.array[4..8]
    }

    pub fn z(&self) -> &[f32] {
        &self.array[8..12]
    }

    pub fn w(&self) -> &[f32] {
        &self.array[12..16]
    }

    fn cx(&self) -> Vector4 {
        Vector4::new([
            self.x()[0], self.y()[0], self.z()[0], self.w()[0]
        ])
    }

    fn cy(&self) -> Vector4 {
        Vector4::new([
            self.x()[1], self.y()[1], self.z()[1], self.w()[1]
        ])
    }

    fn cz(&self) -> Vector4 {
        Vector4::new([
            self.x()[2], self.y()[2], self.z()[2], self.w()[2]
        ])
    }

    fn cw(&self) -> Vector4 {
        Vector4::new([
            self.x()[3], self.y()[3], self.z()[3], self.w()[3]
        ])
    }


    pub fn identity() -> Self {
        Self::new([
            1.0,  0.0,  0.0,  0.0,
            0.0,  1.0,  0.0,  0.0,
            0.0,  0.0,  1.0,  0.0,
            0.0,  0.0,  0.0,  1.0
        ])
    }

    pub fn from_angle_x(theta: f32) -> Self {
        Self::new([
            1.0,  0.0,          0.0,           0.0,
            0.0,  theta.cos(),  -theta.sin(),  0.0,
            0.0,  theta.sin(),  theta.cos(),   0.0,
            0.0,  0.0,          0.0,           1.0
        ])
    }

    pub fn from_angle_y(theta: f32) -> Self {
        Self::new([
            theta.cos(),   0.0,  theta.sin(),  0.0,
            0.0,           1.0,  0.0,          0.0,
            -theta.sin(),  0.0,  theta.cos(),  0.0,
            0.0,           0.0,  0.0,          1.0
        ])
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.array.as_ptr()
    }

    pub fn from_scale(scale: f32) -> Self {
        Self::new([
            scale,  0.0,    0.0,    0.0,
            0.0,    scale,  0.0,    0.0,
            0.0,    0.0,    scale,  0.0,
            0.0,    0.0,    0.0,    1.0
        ])
    }

    pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
        Self::new([
            1.0,  0.0,  0.0,  0.0,
            0.0,  1.0,  0.0,  0.0,
            0.0,  0.0,  1.0,  0.0,
            x,    y,    z,    1.0

        ])
    }

    pub fn perspective(
        fovy: f32, // field of view (in degrees) in the y direction
        aspect: f32, // aspect ratio (width/height)
        near: f32, // distance of the near clipping plane
        far: f32 // distance of the far clipping plane
    ) -> Self {
        // degrees to radians
        let fovy = fovy * PI / 180.0;
        let f = 1.0 / (fovy / 2.0).tan();

        Self::new([
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (far + near) / (near - far), -1.0,  // ERROR???
            0.0, 0.0, 2.0 * far * near / (near - far), 0.0
       ])
    }
}

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let x = Vector4::new([
            self.x() * rhs.cx(),
            self.x() * rhs.cy(),
            self.x() * rhs.cz(),
            self.x() * rhs.cw(),
        ]);
        let y = Vector4::new([
            self.y() * rhs.cx(),
            self.y() * rhs.cy(),
            self.y() * rhs.cz(),
            self.y() * rhs.cw(),
        ]);
        let z = Vector4::new([
            self.z() * rhs.cx(),
            self.z() * rhs.cy(),
            self.z() * rhs.cz(),
            self.z() * rhs.cw(),
        ]);
        let w = Vector4::new([
            self.w() * rhs.cx(),
            self.w() * rhs.cy(),
            self.w() * rhs.cz(),
            self.w() * rhs.cw(),
        ]);

        Matrix4::from_vec4(x, y, z, w)
    }
}