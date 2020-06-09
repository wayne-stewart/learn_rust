use std::ops;
use crate::math::fequal;
use crate::tuple::Tuple;
use crate::tuple::tuple;
use crate::tuple::tuple_i;
use crate::tuple::point;
use crate::tuple::point_i;
use crate::tuple::vector;
use crate::tuple::vector_i;


// #[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct Matrix2x2 {
    r1c1: f32, r1c2: f32,
    r2c1: f32, r2c2: f32
}

impl Matrix2x2 {
    fn from_f32( 
            a: f32, b: f32, 
            c: f32, d: f32) -> Matrix2x2 {
        Matrix2x2 {
            r1c1: a, r1c2: b,
            r2c1: c, r2c2: d
        }
    }

    fn from_i32 (
            a: i32, b: i32,
            c: i32, d: i32) -> Matrix2x2 {
        Matrix2x2::from_f32(
            a as f32, b as f32, 
            c as f32, d as f32) 
    }

    fn transpose(&self) -> Matrix2x2 {
        Matrix2x2::from_f32(
            self.r1c1, self.r2c1,
            self.r1c2, self.r2c2
        )
    }

    fn determinant(&self) -> f32 {
        self.r1c1 * self.r2c2 - self.r1c2 * self.r2c1
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, other: &Self) -> bool {
        fequal(self.r1c1, other.r1c1) &&
        fequal(self.r1c2, other.r1c2) &&
        fequal(self.r2c1, other.r2c1) &&
        fequal(self.r2c2, other.r2c2)
    }
}

impl ops::Mul for Matrix2x2 {
    type Output = Matrix2x2;
    fn mul(self, rhs: Matrix2x2) -> Matrix2x2 {
        Matrix2x2 {
            r1c1: self.r1c1 * rhs.r1c1 + self.r1c2 * rhs.r2c1,
            r1c2: self.r1c1 * rhs.r1c2 + self.r1c2 * rhs.r2c2,
            r2c1: self.r2c1 * rhs.r1c1 + self.r2c2 * rhs.r2c1,
            r2c2: self.r2c1 * rhs.r1c2 + self.r2c2 * rhs.r2c2
        }
    }
}

// #[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix3x3 {
    r1c1: f32, r1c2: f32, r1c3: f32,
    r2c1: f32, r2c2: f32, r2c3: f32,
    r3c1: f32, r3c2: f32, r3c3: f32
}

impl Matrix3x3 {
    pub fn from_f32(
            a: f32, b: f32, c: f32, 
            d: f32, e: f32, f: f32, 
            g: f32, h: f32, i: f32) -> Matrix3x3 {
        Matrix3x3 {
            r1c1: a, r1c2: b, r1c3: c,
            r2c1: d, r2c2: e, r2c3: f,
            r3c1: g, r3c2: h, r3c3: i
        }    
    }
    
    pub fn from_i32(
            a: i32, b: i32, c: i32, 
            d: i32, e: i32, f: i32, 
            g: i32, h: i32, i: i32) -> Matrix3x3 {
        Matrix3x3::from_f32(
            a as f32, b as f32, c as f32,
            d as f32, e as f32, f as f32,
            g as f32, h as f32, i as f32
        )
    }

    fn transpose(&self) -> Matrix3x3 {
        Matrix3x3::from_f32(
            self.r1c1, self.r2c1, self.r3c1,
            self.r1c2, self.r2c2, self.r3c2,
            self.r1c3, self.r2c3, self.r3c3
        )
    }
    
    fn determinant(&self) -> f32 {
        let a = self.cofactor(0, 0);
        let b = self.cofactor(0, 1);
        let c = self.cofactor(0, 2);
        a * self.r1c1 + b * self.r1c2 + c * self.r1c3
    }

    /*
    getting the submatrix of row, col

    submatrix of 3x3 returns matrix 2x2 created
    by excluding the input row and col
    */
    fn submatrix(&self, row: u8, col: u8) -> Matrix2x2 {
        let arr = [
            self.r1c1, self.r1c2, self.r1c3,
            self.r2c1, self.r2c2, self.r2c3,
            self.r3c1, self.r3c2, self.r3c3];
        let mut t: [f32; 4] = [0.0; 4];
        let mut t_index = 0;
        for y in 0..3 {
            for x in 0..3 {
                if y != row && x != col {
                    let m_index = (y * 3 + x) as usize;
                    t[t_index] = arr[m_index];
                    t_index += 1;
                }
            }
        }
        Matrix2x2::from_f32(t[0],t[1],t[2],t[3])
    }

    /*
        getting the minor of row, cell

        the minor is the determinant of the submatrix at row, col
    */
    fn minor(&self, row: u8, col: u8) -> f32 {
        let submatrix = self.submatrix(row, col);
        submatrix.determinant()
    }

    fn cofactor(&self, row: u8, col: u8) -> f32 {
        let minor = self.minor(row, col);
        cofactor(minor, row, col)
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, other: &Self) -> bool {
        fequal(self.r1c1, other.r1c1) &&
        fequal(self.r1c2, other.r1c2) &&
        fequal(self.r1c3, other.r1c3) &&
        fequal(self.r2c1, other.r2c1) &&
        fequal(self.r2c2, other.r2c2) &&
        fequal(self.r2c3, other.r2c3) &&
        fequal(self.r3c1, other.r3c1) &&
        fequal(self.r3c2, other.r3c2) &&
        fequal(self.r3c3, other.r3c3)
    }
}

impl ops::Mul for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, rhs: Matrix3x3) -> Matrix3x3 {
        Matrix3x3 {
            r1c1: self.r1c1 * rhs.r1c1 + self.r1c2 * rhs.r2c1 + self.r1c3 * rhs.r3c1,
            r1c2: self.r1c1 * rhs.r1c2 + self.r1c2 * rhs.r2c2 + self.r1c3 * rhs.r3c2,
            r1c3: self.r1c1 * rhs.r1c3 + self.r1c2 * rhs.r2c3 + self.r1c3 * rhs.r3c3,
            r2c1: self.r2c1 * rhs.r1c1 + self.r2c2 * rhs.r2c1 + self.r2c3 * rhs.r3c1,
            r2c2: self.r2c1 * rhs.r1c2 + self.r2c2 * rhs.r2c2 + self.r2c3 * rhs.r3c2,
            r2c3: self.r2c1 * rhs.r1c3 + self.r2c2 * rhs.r2c3 + self.r2c3 * rhs.r3c3,
            r3c1: self.r3c1 * rhs.r1c1 + self.r3c2 * rhs.r2c1 + self.r3c3 * rhs.r3c1,
            r3c2: self.r3c1 * rhs.r1c2 + self.r3c2 * rhs.r2c2 + self.r3c3 * rhs.r3c2,
            r3c3: self.r3c1 * rhs.r1c3 + self.r3c2 * rhs.r2c3 + self.r3c3 * rhs.r3c3
        }
    }
}

// #[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    r1c1: f32, r1c2: f32, r1c3: f32, r1c4: f32,
    r2c1: f32, r2c2: f32, r2c3: f32, r2c4: f32,
    r3c1: f32, r3c2: f32, r3c3: f32, r3c4: f32,
    r4c1: f32, r4c2: f32, r4c3: f32, r4c4: f32
}

impl Matrix4x4 {
    fn from_f32(
        a: f32, b: f32, c: f32, d: f32, 
        e: f32, f: f32, g: f32, h: f32,
        i: f32, j: f32, k: f32, l: f32,
        m: f32, n: f32, o: f32, p: f32) -> Matrix4x4 {
        Matrix4x4 {
            r1c1: a, r1c2: b, r1c3: c, r1c4: d,
            r2c1: e, r2c2: f, r2c3: g, r2c4: h,
            r3c1: i, r3c2: j, r3c3: k, r3c4: l,
            r4c1: m, r4c2: n, r4c3: o, r4c4: p
        }
    }
    
    fn from_i32(
        a: i32, b: i32, c: i32, d: i32, 
        e: i32, f: i32, g: i32, h: i32,
        i: i32, j: i32, k: i32, l: i32,
        m: i32, n: i32, o: i32, p: i32) -> Matrix4x4 {
        Matrix4x4::from_f32 (
            a as f32, b as f32, c as f32, d as f32,
            e as f32, f as f32, g as f32, h as f32,
            i as f32, j as f32, k as f32, l as f32,
            m as f32, n as f32, o as f32, p as f32
        )
    }

    fn transpose(&self) -> Matrix4x4 {
        Matrix4x4::from_f32(
            self.r1c1, self.r2c1, self.r3c1, self.r4c1,
            self.r1c2, self.r2c2, self.r3c2, self.r4c2,
            self.r1c3, self.r2c3, self.r3c3, self.r4c3,
            self.r1c4, self.r2c4, self.r3c4, self.r4c4
        )
    }
    
    fn determinant(&self) -> f32 {
        let a = self.cofactor(0, 0);
        let b = self.cofactor(0, 1);
        let c = self.cofactor(0, 2);
        let d = self.cofactor(0, 3);
        a * self.r1c1 + b * self.r1c2 + c * self.r1c3 + d * self.r1c4
    }
    
    /*
        getting the submatrix of row, col
    
        submatrix of 4x4 returns matrix 3x3 created
        by excluding the input row and col
    */
    fn submatrix(&self, row: u8, col: u8) -> Matrix3x3 {
        let arr = [
            self.r1c1, self.r1c2, self.r1c3, self.r1c4,
            self.r2c1, self.r2c2, self.r2c3, self.r2c4,
            self.r3c1, self.r3c2, self.r3c3, self.r3c4,
            self.r4c1, self.r4c2, self.r4c3, self.r4c4];
        let mut t: [f32; 9] = [0.0; 9];
        let mut t_index = 0;
        for y in 0..4 {
            for x in 0..4 {
                if y != row && x != col {
                    let m_index = (y * 4 + x) as usize;
                    t[t_index] = arr[m_index];
                    t_index += 1;
                }
            }
        }
        Matrix3x3::from_f32(t[0],t[1],t[2],t[3],t[4],t[5],t[6],t[7],t[8])
    }
    
    // determinant of the submatrix
    fn minor(&self, row: u8, col: u8) -> f32 {
        let submatrix = self.submatrix(row, col);
        submatrix.determinant()
    }

    fn cofactor(&self, row: u8, col: u8) -> f32 {
        let minor = self.minor(row, col);
        cofactor(minor, row, col)
    }
    
    fn inverse(&self) -> Matrix4x4 {
        let determinant = self.determinant();
        if determinant == 0.0 {
            panic!("Attempting to invert and invertible matrix");
        }
        let mut t: [f32; 16] = [0.0; 16];
        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                let t_index = (col * 4 + row) as usize;
                t[t_index] = c / determinant;
            }
        }
        Matrix4x4::from_f32(t[0],t[1],t[2],t[3],t[4],t[5],t[6],t[7],t[8],t[9],t[10],t[11],t[12],t[13],t[14],t[15])
    }

    fn translation(x: f32, y: f32, z: f32) -> Matrix4x4 {
        let mut a = MATRIX_4X4_IDENTITY;
        a.r1c4 = x;
        a.r2c4 = y;
        a.r3c4 = z;
        return a;
    }
    
    fn scaling(x: f32, y: f32, z: f32) -> Matrix4x4 {
        let mut a = MATRIX_4X4_IDENTITY;
        a.r1c1 = x;
        a.r2c2 = y;
        a.r3c3 = z;
        return a;
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        fequal(self.r1c1, other.r1c1) &&
        fequal(self.r1c2, other.r1c2) &&
        fequal(self.r1c3, other.r1c3) &&
        fequal(self.r1c4, other.r1c4) &&
        fequal(self.r2c1, other.r2c1) &&
        fequal(self.r2c2, other.r2c2) &&
        fequal(self.r2c3, other.r2c3) &&
        fequal(self.r2c4, other.r2c4) &&
        fequal(self.r3c1, other.r3c1) &&
        fequal(self.r3c2, other.r3c2) &&
        fequal(self.r3c3, other.r3c3) &&
        fequal(self.r3c4, other.r3c4) &&
        fequal(self.r4c1, other.r4c1) &&
        fequal(self.r4c2, other.r4c2) &&
        fequal(self.r4c3, other.r4c3) &&
        fequal(self.r4c4, other.r4c4)
    }
}

impl ops::Mul for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            r1c1: self.r1c1 * rhs.r1c1 + self.r1c2 * rhs.r2c1 + self.r1c3 * rhs.r3c1 + self.r1c4 * rhs.r4c1,
            r1c2: self.r1c1 * rhs.r1c2 + self.r1c2 * rhs.r2c2 + self.r1c3 * rhs.r3c2 + self.r1c4 * rhs.r4c2,
            r1c3: self.r1c1 * rhs.r1c3 + self.r1c2 * rhs.r2c3 + self.r1c3 * rhs.r3c3 + self.r1c4 * rhs.r4c3,
            r1c4: self.r1c1 * rhs.r1c4 + self.r1c2 * rhs.r2c4 + self.r1c3 * rhs.r3c4 + self.r1c4 * rhs.r4c4,
            r2c1: self.r2c1 * rhs.r1c1 + self.r2c2 * rhs.r2c1 + self.r2c3 * rhs.r3c1 + self.r2c4 * rhs.r4c1,
            r2c2: self.r2c1 * rhs.r1c2 + self.r2c2 * rhs.r2c2 + self.r2c3 * rhs.r3c2 + self.r2c4 * rhs.r4c2,
            r2c3: self.r2c1 * rhs.r1c3 + self.r2c2 * rhs.r2c3 + self.r2c3 * rhs.r3c3 + self.r2c4 * rhs.r4c3,
            r2c4: self.r2c1 * rhs.r1c4 + self.r2c2 * rhs.r2c4 + self.r2c3 * rhs.r3c4 + self.r2c4 * rhs.r4c4,
            r3c1: self.r3c1 * rhs.r1c1 + self.r3c2 * rhs.r2c1 + self.r3c3 * rhs.r3c1 + self.r3c4 * rhs.r4c1,
            r3c2: self.r3c1 * rhs.r1c2 + self.r3c2 * rhs.r2c2 + self.r3c3 * rhs.r3c2 + self.r3c4 * rhs.r4c2,
            r3c3: self.r3c1 * rhs.r1c3 + self.r3c2 * rhs.r2c3 + self.r3c3 * rhs.r3c3 + self.r3c4 * rhs.r4c3,
            r3c4: self.r3c1 * rhs.r1c4 + self.r3c2 * rhs.r2c4 + self.r3c3 * rhs.r3c4 + self.r3c4 * rhs.r4c4,
            r4c1: self.r4c1 * rhs.r1c1 + self.r4c2 * rhs.r2c1 + self.r4c3 * rhs.r3c1 + self.r4c4 * rhs.r4c1,
            r4c2: self.r4c1 * rhs.r1c2 + self.r4c2 * rhs.r2c2 + self.r4c3 * rhs.r3c2 + self.r4c4 * rhs.r4c2,
            r4c3: self.r4c1 * rhs.r1c3 + self.r4c2 * rhs.r2c3 + self.r4c3 * rhs.r3c3 + self.r4c4 * rhs.r4c3,
            r4c4: self.r4c1 * rhs.r1c4 + self.r4c2 * rhs.r2c4 + self.r4c3 * rhs.r3c4 + self.r4c4 * rhs.r4c4
        }
    }
}

impl ops::Mul<Tuple> for Matrix4x4 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Tuple {
        tuple (
            self.r1c1 * rhs.x + self.r1c2 * rhs.y + self.r1c3 * rhs.z + self.r1c4 * rhs.w,
            self.r2c1 * rhs.x + self.r2c2 * rhs.y + self.r2c3 * rhs.z + self.r2c4 * rhs.w,
            self.r3c1 * rhs.x + self.r3c2 * rhs.y + self.r3c3 * rhs.z + self.r3c4 * rhs.w,
            self.r4c1 * rhs.x + self.r4c2 * rhs.y + self.r4c3 * rhs.z + self.r4c4 * rhs.w
        )
    }
}

const MATRIX_2X2_IDENTITY : Matrix2x2 = Matrix2x2 {
    r1c1: 1.0, r1c2: 0.0,
    r2c1: 0.0, r2c2: 1.0
};

const MATRIX_3X3_IDENTITY : Matrix3x3 = Matrix3x3 {
    r1c1: 1.0, r1c2: 0.0, r1c3: 0.0,
    r2c1: 0.0, r2c2: 1.0, r2c3: 0.0,
    r3c1: 0.0, r3c2: 0.0, r3c3: 1.0
};

const MATRIX_4X4_IDENTITY : Matrix4x4 = Matrix4x4 {
    r1c1: 1.0, r1c2: 0.0, r1c3: 0.0, r1c4: 0.0,
    r2c1: 0.0, r2c2: 1.0, r2c3: 0.0, r2c4: 0.0,
    r3c1: 0.0, r3c2: 0.0, r3c3: 1.0, r3c4: 0.0,
    r4c1: 0.0, r4c2: 0.0, r4c3: 0.0, r4c4: 1.0
};

fn cofactor(minor: f32, row: u8, col: u8) -> f32 {
    if (row + col) % 2 == 0 {
        return minor;
    }
    else {
        return -minor;
    }
}

#[test]
fn matrix2x2_create_test() {
    let a = Matrix2x2::from_i32(1,2,3,4);
    let b = Matrix2x2::from_f32(1.0, 2.0, 3.0, 4.0);
    let c = Matrix2x2::from_i32(-1,2,3,4);
    let d = Matrix2x2::from_i32(1,-1,3,4);
    let e = Matrix2x2::from_i32(1,2,-1,4);
    let f = Matrix2x2::from_i32(1,2,3,-1);
    assert_eq!(a, b);
    assert!(a != c);
    assert!(a != d);
    assert!(a != e);
    assert!(a != f);
}

#[test]
fn matrix3x3_create_test() {
    let a = Matrix3x3::from_i32(1,2,3,4,5,6,7,8,9);
    let b = Matrix3x3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let c = Matrix3x3::from_i32(-1,2,3,4,5,6,7,8,9);
    let d = Matrix3x3::from_i32(1,-1,3,4,5,6,7,8,9);
    let e = Matrix3x3::from_i32(1,2,-1,4,5,6,7,8,9);
    let f = Matrix3x3::from_i32(1,2,3,-1,5,6,7,8,9);
    let g = Matrix3x3::from_i32(1,2,3,4,-1,6,7,8,9);
    let h = Matrix3x3::from_i32(1,2,3,4,5,-1,7,8,9);
    let i = Matrix3x3::from_i32(1,2,3,4,5,6,-1,8,9);
    let j = Matrix3x3::from_i32(1,2,3,4,5,6,7,-1,9);
    let k = Matrix3x3::from_i32(1,2,3,4,5,6,7,8,-1);
    assert_eq!(a, b);
    assert!(a != c);
    assert!(a != d);
    assert!(a != e);
    assert!(a != f);
    assert!(a != g);
    assert!(a != h);
    assert!(a != i);
    assert!(a != j);
    assert!(a != k);
}

#[test]
fn matrix4x4_create_test() {
    let a = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    let b = Matrix4x4::from_f32(1.0,2.0,3.0,4.0,5.0, 6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0);
    let c = Matrix4x4::from_i32(-1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    let d = Matrix4x4::from_i32(1,-1,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    let e = Matrix4x4::from_i32(1,2,-1,4,5,6,7,8,9,10,11,12,13,14,15,16);
    let f = Matrix4x4::from_i32(1,2,3,-1,5,6,7,8,9,10,11,12,13,14,15,16);
    let g = Matrix4x4::from_i32(1,2,3,4,-1,6,7,8,9,10,11,12,13,14,15,16);
    let h = Matrix4x4::from_i32(1,2,3,4,5,-1,7,8,9,10,11,12,13,14,15,16);
    let i = Matrix4x4::from_i32(1,2,3,4,5,6,-1,8,9,10,11,12,13,14,15,16);
    let j = Matrix4x4::from_i32(1,2,3,4,5,6,7,-1,9,10,11,12,13,14,15,16);
    let k = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,-1,10,11,12,13,14,15,16);
    let l = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,-1,11,12,13,14,15,16);
    let m = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,10,-1,12,13,14,15,16);
    let n = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,10,11,-1,13,14,15,16);
    let o = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,10,11,12,-1,14,15,16);
    let p = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,10,11,12,13,-1,15,16);
    let q = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,10,11,12,13,14,-1,16);
    let r = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,-1);
    assert_eq!(a, b);
    assert!(a != c);
    assert!(a != d);
    assert!(a != e);
    assert!(a != f);
    assert!(a != g);
    assert!(a != h);
    assert!(a != i);
    assert!(a != j);
    assert!(a != k);
    assert!(a != l);
    assert!(a != m);
    assert!(a != n);
    assert!(a != o);
    assert!(a != p);
    assert!(a != q);
    assert!(a != r);
}

#[test]
fn matrix2x2_mul_teste() {
    let a = Matrix2x2::from_i32(1,3,5,7);
    let b = Matrix2x2::from_i32(2,4,6,8);
    let r = a * b;
    let t = Matrix2x2::from_i32(20,28,52,76);
    assert_eq!(r, t);
}

#[test]
fn matrix3x3_mul_test() {
    let a = Matrix3x3::from_i32(1,3,5,7,9,11,13,15,17);
    let b = Matrix3x3::from_i32(2,4,6,8,10,12,14,16,18);
    let r = a * b;
    let t = Matrix3x3::from_i32(96,114,132,240,294,348,384,474,564);
    assert_eq!(r, t);
}

#[test]
fn matrix4x4_mul_test() {
    let a = Matrix4x4::from_i32(1,3,5,7,9,11,13,15,17,19,21,23,25,27,29,31);
    let b = Matrix4x4::from_i32(2,4,6,8,10,12,14,16,18,20,22,24,26,28,30,32);
    let r = a * b;
    let t = Matrix4x4::from_i32(304,336,368,400,752,848,944,1040,1200,1360,1520,1680,1648,1872,2096,2320);
    assert_eq!(r, t);
}

#[test]
fn matrix4x4_mul_tuple_test() {
    let a = Matrix4x4::from_i32(1,3,5,7,9,11,13,15,17,19,21,23,25,27,29,31);
    let b = tuple_i(2,4,8,10);
    let r = a * b;
    let t = tuple_i(124,316,508,700);
    assert_eq!(r, t);
}

#[test]
fn matrix_identity_test() {
    let a = Matrix2x2::from_i32(1,2,3,4);
    let b = Matrix3x3::from_i32(1,2,3,4,5,6,7,8,9);
    let c = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);

    let ar = a * MATRIX_2X2_IDENTITY;
    let br = b * MATRIX_3X3_IDENTITY;
    let cr = c * MATRIX_4X4_IDENTITY;

    assert_eq!(a, ar);
    assert_eq!(b, br);
    assert_eq!(c, cr);
}

#[test]
fn matrix_transpose_test() {
    let a = Matrix2x2::from_i32(1,2,3,4);
    let b = Matrix3x3::from_i32(1,2,3,4,5,6,7,8,9);
    let c = Matrix4x4::from_i32(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);

    let ar = a.transpose();
    let br = b.transpose();
    let cr = c.transpose();

    let at = Matrix2x2::from_i32(1,3,2,4);
    let bt = Matrix3x3::from_i32(1,4,7,2,5,8,3,6,9);
    let ct = Matrix4x4::from_i32(1,5,9,13,2,6,10,14,3,7,11,15,4,8,12,16);

    assert_eq!(ar, at);
    assert_eq!(br, bt);
    assert_eq!(cr, ct);
}

#[test]
fn matrix_determinant_test() {
    let a = Matrix2x2::from_i32(1,5,-3,2);
    let ad = a.determinant();
    assert_eq!(ad, 17.0);
}

#[test]
fn matrix3x3_submatrix_test() {
    let a = Matrix3x3::from_i32(1,2,3,4,5,6,7,8,9);
    let b = a.submatrix(0, 2);
    let c = Matrix2x2::from_i32(4,5,7,8);
    assert_eq!(b, c);
}

#[test]
fn matrix4x4_submatrix_test() {
    let a = Matrix4x4::from_i32(1,2,3,4, 5,6,7,8, 9,10,11,12, 13,14,15,16);
    let b = a.submatrix(2, 1);
    let c = Matrix3x3::from_i32(1,3,4, 5,7,8, 13,15,16);
    assert_eq!(b, c);
}

#[test]
fn matrix3x3_minor_cofactor_test() {
    let a = Matrix3x3::from_i32(3,5,0,2,-1,-7,6,-1,5);
    assert_eq!(a.minor(0, 0), -12.0);
    assert_eq!(a.cofactor(0, 0), -12.0);
    assert_eq!(a.minor(1, 0), 25.0);
    assert_eq!(a.cofactor(1, 0), -25.0);
}

#[test]
fn matrix3x3_deteriminant_test() {
    let a = Matrix3x3::from_i32(1,2,6,-5,8,-4,2,6,4);
    assert_eq!(a.cofactor(0,0), 56.0);
    assert_eq!(a.cofactor(0,1), 12.0);
    assert_eq!(a.cofactor(0,2), -46.0);
    assert_eq!(a.determinant(), -196.0);
}

#[test]
fn matrix4x4_determinant_test() {
    let a = Matrix4x4::from_i32(-2,-8,3,5, -3,1,7,3, 1,2,-9,6, -6,7,7,-9);
    assert_eq!(a.cofactor(0,0), 690.0);
    assert_eq!(a.cofactor(0,1), 447.0);
    assert_eq!(a.cofactor(0,2), 210.0);
    assert_eq!(a.cofactor(0,3), 51.0);
    assert_eq!(a.determinant(), -4071.0);
}

#[test]
fn matrix4x4_test_for_invertibility_test() {
    let a = Matrix4x4::from_i32(6,4,4,4, 5,5,7,6, 4,-9,3,-7, 9,1,7,-6);
    let b = Matrix4x4::from_i32(-4,2,-2,-3, 9,6,2,6, 0,-5,1,-5, 0,0,0,0);
    assert_eq!(a.determinant(), -2120.0);
    assert_eq!(b.determinant(), 0.0);
}

#[test]
fn inverse_test1() {
    let a = Matrix4x4::from_i32(-5,2,6,-8, 1,-5,1,8, 7,7,-6,-7, 1,-3,7,4);
    let b = a.inverse();
    assert_eq!(a.determinant(), 532.0);
    assert_eq!(a.cofactor(2,3), -160.0);
    assert!(fequal(b.r4c3, -160.0/532.0));
    assert_eq!(a.cofactor(3,2), 105.0);
    assert!(fequal(b.r3c4, 105.0/532.0));
    let c = Matrix4x4::from_f32(
        0.21805, 0.45113, 0.24060, -0.04511,
        -0.80827, -1.45677, -0.44361, 0.52068,
        -0.07895, -0.22368, -0.05263, 0.19737,
        -0.52256, -0.81391, -0.30075, 0.30639);
    assert_eq!(b, c);
}

#[test]
fn inverse_test2() {
    let a = Matrix4x4::from_i32(8,-5,9,2,7,5,6,1,-6,0,9,6,-3,0,-9,-4);
    let b = a.inverse();
    let c = Matrix4x4::from_f32(
        -0.15385, -0.15385, -0.28205, -0.53846,
        -0.07692, 0.12308, 0.02564, 0.03077,
        0.35897, 0.35897, 0.43590, 0.92308,
        -0.69231, -0.69231, -0.76923, -1.92308);
    assert_eq!(b, c);
}

#[test]
fn inverse_test3() {
    let a = Matrix4x4::from_i32(9,3,0,9,-5,-2,-6,-3,-4,9,6,4,-7,6,6,2);
    let b = a.inverse();
    let c = Matrix4x4::from_f32(
        -0.04074, -0.07778, 0.14444, -0.22222,
        -0.07778, 0.03333, 0.36667, -0.33333,
        -0.02901, -0.14630, -0.10926, 0.12963,
        0.17778, 0.06667, -0.26667, 0.33333);
    assert_eq!(b, c);
}

#[test]
fn matrix4x4_multiply_inverse_test() {
    let a = Matrix4x4::from_i32(3,-9,7,3, 3,-8,2,-9, -4,4,4,1, -6,5,-1,1);
    let b = Matrix4x4::from_i32(8,2,2,2, 3,-1,7,0, 7,0,5,4, 6,-2,0,5);
    let c = a * b;
    let d = b.inverse();
    let e = c * d;
    assert_eq!(a, e);
}

#[test]
fn translate_test() {
    let p = point(-3.0, 4.0, 5.0);
    let v = vector(-3.0, 4.0, 5.0);

    // moves point in direction of translation vector
    let translate = Matrix4x4::translation(5.0, -3.0, 2.0);
    assert_eq!(translate * p, point(2.0, 1.0, 7.0));

    // inverting moves point in oposite direction
    let inverse = translate.inverse();
    assert_eq!(inverse * p, point(-8.0, 7.0, 3.0));

    // translation matrix does not affect vectors
    assert_eq!(translate * v, vector(-3.0, 4.0, 5.0));
}

#[test]
fn scaling_test() {
    let p = point_i(-4, 6, 8);
    let v = vector_i(-4, 6, 8);

    // scale point by each value in the scale matrix
    let scale = Matrix4x4::scaling(2.0, 3.0, 4.0);
    assert_eq!(scale * p, point_i(-8, 18, 32));

    // scaling also applies to vectors
    assert_eq!(scale * v, vector_i(-8, 18, 32));

    // inverse of scaling shrinks by same values
    let inverse = scale.inverse();
    assert_eq!(inverse * v, vector_i(-2, 2, 2));
    assert_eq!(inverse * p, point_i(-2, 2, 2));
}

