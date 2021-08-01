use std::ops::Sub;
use std::convert::From;

pub struct Matrix {
    m: [[f32;4];4]
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            m:[[0.0f32;4];4]
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(PartialEq, Debug)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32
}

impl Vec2D {
    pub fn new(x:f32, y:f32) -> Self {
        Self {
            x,y
        }
    } 
    pub fn scale(&self, factor_x: f32, factor_y: f32) -> Vec2D {
        Vec2D {
            x: self.x * factor_x,
            y: self.y * factor_y,
        }
    }
}
impl Vec3D {
    pub fn new(x:f32, y:f32, z:f32) -> Self {
        Vec3D {x,y,z}
    }

    pub fn normalise(&self) -> Self {
        let len: f32 = self.len();

        Vec3D {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            
        }
    }

    pub fn len(&self) -> f32 {
        (
            (self.x * self.x) + 
            (self.y * self.y) + 
            (self.z * self.z) 
        ).sqrt()
    }

    pub fn dot(&self, v2: &Vec3D) -> f32 {
        (self.x * v2.x) + 
        (self.y * v2.y) +
        (self.z * v2.z)
    }

    pub fn project2(
        &self,
        rotation: Vec3D,
        camera: Vec3D,
        fov: f32,
        width:f32,
        height:f32,
        z_near:f32,
        z_far: f32) -> Vec2D {

        let a: f32 = height/width; // aspect ratio
        let fov = 1.0 / (fov * 0.5 / 180.0 * 3.141592).tan();
        /*
        z_near = 0.1;
        z_far = 1000.0;
        fov = 90.0
        */
        //let z_near = z_near + camera.z;

        let mut rotated = 
            (self) 
            .rot_x(rotation.x)
            .rot_y(rotation.y)
            .rot_z(rotation.z);

        let mut matrix = Matrix::new();

        matrix.m[0][0] = a * fov;
        matrix.m[1][1] = fov;
        matrix.m[2][2] = z_far / (z_far - z_near);
        matrix.m[3][2] = (-z_far * z_near) / (z_far - z_near);
        matrix.m[2][3] = 1.0;
        matrix.m[3][3] = 0.0;

        // translate vector before multiplying it
        rotated.z += 3.0;

        let mut d = (rotated - camera).mult_mat_vec(&matrix); // best experience

        // scale
        d.x += 1.0;
        d.y += 1.0;
        d.z += 1.0;

        d.x *= width * 0.5;
        d.y *= height * 0.5;

        //d.x -= width / 0.5;
        // let bx = ((camera.z / d.z) * d.x) + camera.x;
        // let by = ((camera.z / d.z) * d.y) + camera.y;

        Vec2D {
            x: d.x / d.z,
            y: d.y / d.z 
            // x: bx,
            // y: by
        }

    }

    pub fn mult_mat_vec(&self, m: &Matrix) -> Vec3D {
        let mut x = 
            self.x * m.m[0][0] +
            self.y * m.m[1][0] +
            self.z * m.m[2][0] +
            m.m[3][0]
        ;
        let mut y = 
            self.x * m.m[0][1] +
            self.y * m.m[1][1] +
            self.z * m.m[2][1] +
            m.m[3][1]
        ;

        let mut z = 
            self.x * m.m[0][2] +
            self.y * m.m[1][2] +
            self.z * m.m[2][2] +
            m.m[3][2]
        ;

        let w =
            self.x * m.m[0][3] +
            self.y * m.m[1][3] +
            self.z * m.m[2][3] +
            m.m[3][3]
        ;

        if w != 0.0 {
            x /= w;
            y /= w;
            z /= w;
        };

        Vec3D {
            x,
            y,
            z,
        }
    }

    pub fn rot_x(self, angle:f32) -> Self {
        let rx =
        [
            [1.0, 0.0,  0.0],
            [0.0, angle.cos(), angle.sin()],
            [0.0, -angle.sin(), angle.cos()]
        ];
        self.multiply(rx)        
    }

    pub fn rot_y(self, angle:f32) -> Self {
        let ry =
        [
            [angle.cos(), 0.0, -angle.sin()],
            [0.0, 1.0, 0.0],
            [angle.sin(), 0.0, angle.cos()]
        ];
        self.multiply(ry)        
    }

    pub fn rot_z(self, angle:f32) -> Self {
        let rz =
        [
            [angle.cos(),   angle.sin(), 0.0],
            [-angle.sin(),  angle.cos(), 0.0],
            [0.0, 0.0, 1.0]
        ];
        self.multiply(rz)    
    }

    pub fn multiply(self, mat: [[f32;3];3]) -> Self {
        let new_x = 
            mat[0][0] * self.x +
            mat[1][0] * self.y +
            mat[2][0] * self.z
        ;

        let new_y = 
            (mat[0][1] * self.x) +
            (mat[1][1] * self.y) +
            (mat[2][1] * self.z)
        ;

        let new_z = 
            mat[0][2] * self.x +
            mat[1][2] * self.y +
            mat[2][2] * self.z
        ;


        Vec3D {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }
}

impl From<(f32, f32, f32)> for Vec3D {   // Convert tuple into Vec3D
    fn from(pos: (f32,f32,f32)) -> Self {
        Vec3D {
            x: pos.0,
            y: pos.1,
            z: pos.2
        }
    }
}

impl From<&Vec3D> for (f32, f32, f32) { // Convert Vec3D into tuple
    fn from(v: &Vec3D) -> Self {(v.x, v.y, v.z)}
}
impl From<Vec3D> for (f32, f32, f32) { // Convert Vec3D into tuple
    fn from(v: Vec3D) -> Self {(v.x, v.y, v.z)}
}

impl Sub for Vec3D {
    type Output =  Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


#[test] 
fn test() {
    let point_tup: (f32, f32, f32) = (1.0, 2.0, 3.0);
    let point_vec3d: Vec3D = point_tup.into();

    assert_eq!(Vec3D{x: 1.0, y: 2.0, z: 3.0}, point_vec3d);


    let point_3d = Vec3D::new(1.0, 2.0, 3.0);
    let tuple_3d: (f32, f32, f32) = point_vec3d.into();

    assert_eq!((1.0, 2.0, 3.0), tuple_3d);

}
