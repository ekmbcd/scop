use std::{ops, f32::consts::PI};

#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
    /// The z component of the vector.
    pub z: f32,
    /// The w component of the vector.
    pub w: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    /// The first column of the matrix.
    pub x: Vector4,
    /// The second column of the matrix.
    pub y: Vector4,
    /// The third column of the matrix.
    pub z: Vector4,
    /// The fourth column of the matrix.
    pub w: Vector4,
}

impl Vector4 {
    fn new(x: f32, y:f32, z:f32, w:f32) -> Self {
        Self {x, y, z, w}
    }

    fn from_arr(arr: [f32; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }

    fn from_slice(arr: &[f32]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }

    fn to_arr(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl ops::Mul<Vector4> for Vector4 {
    type Output = f32;

    fn mul(self, rhs: Vector4) -> f32 {
        // println!("> Foo.add(Bar) was called {:?} {:?}", self, rhs);
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}


impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Matrix4 {
        // println!("matrix");
        let x = Vector4::new(
            self.x * rhs.cx(),
            self.x * rhs.cy(),
            self.x * rhs.cz(),
            self.x * rhs.cw(),
        );
        let y = Vector4::new(
            self.y * rhs.cx(),
            self.y * rhs.cy(),
            self.y * rhs.cz(),
            self.y * rhs.cw(),
        );
        let z = Vector4::new(
            self.z * rhs.cx(),
            self.z * rhs.cy(),
            self.z * rhs.cz(),
            self.z * rhs.cw(),
        );
        let w = Vector4::new(
            self.w * rhs.cx(),
            self.w * rhs.cy(),
            self.w * rhs.cz(),
            self.w * rhs.cw(),
        );

        Matrix4::new(x, y, z, w)
    }
}

impl Matrix4 {
    pub fn from_arr(v: [f32; 16]) -> Self {
        Self {
            x: Vector4::from_slice(&v[..4]), 
            y: Vector4::from_slice(&v[4..8]), 
            z: Vector4::from_slice(&v[8..12]), 
            w: Vector4::from_slice(&v[12..16]),
        }
    }

    pub fn new(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Self {
        Self {
            x: Vector4::new(1.0, 0.0, 0.0, 0.0),
            y: Vector4::new(0.0, 1.0, 0.0, 0.0),
            z: Vector4::new(0.0, 0.0, 1.0, 0.0),
            w: Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn from_angle_x(theta: f32) -> Self {
        let rot = Matrix4::from_arr([
            1.0,          0.0,           0.0,  0.0,
            0.0,  theta.cos(),  -theta.sin(),  0.0,
            0.0,  theta.sin(),   theta.cos(),  0.0,
            0.0,          0.0,           0.0,  1.0
        ]);
        rot
    }

    pub fn from_angle_y(theta: f32) -> Self {
        let rot = Matrix4::from_arr([
             theta.cos(),  0.0,  theta.sin(),  0.0,
                     0.0,  1.0,  0.0,          0.0,
            -theta.sin(),  0.0,  theta.cos(),  0.0,
                     0.0,  0.0,  0.0,          1.0
        ]);
        rot
    }

    fn cx(&self) -> Vector4 {
        Vector4::new(self.x.x, self.y.x, self.z.x, self.w.x)
    }

    fn cy(&self) -> Vector4 {
        Vector4::new(self.x.y, self.y.y, self.z.y, self.w.y)
    }

    fn cz(&self) -> Vector4 {
        Vector4::new(self.x.z, self.y.z, self.z.z, self.w.z)
    }

    fn cw(&self) -> Vector4 {
        Vector4::new(self.x.w, self.y.w, self.z.w, self.w.w)
    }

    // pub fn as_ptr(&self) -> *const f32 {
    //     let arr = [
    //         self.x.x, self.x.y, self.x.z, self.x.w,
    //         self.y.x, self.y.y, self.y.z, self.y.w,
    //         self.z.x, self.z.y, self.z.z, self.z.w,
    //         self.w.x, self.w.y, self.w.z, self.w.w,
    //     ];
    //     // println!("{:?}", arr);
    //     arr.as_ptr()
    // }

    pub fn as_arr(&self) -> [f32; 16] {
        let arr = [
            self.x.x, self.x.y, self.x.z, self.x.w,
            self.y.x, self.y.y, self.y.z, self.y.w,
            self.z.x, self.z.y, self.z.z, self.z.w,
            self.w.x, self.w.y, self.w.z, self.w.w,
        ];
        // println!("{:?}", arr);
        arr
    }
}