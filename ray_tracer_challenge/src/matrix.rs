use std::ops;
use crate::math::fequal;
use crate::tuple::Tuple;
use crate::tuple::tuple;
use crate::tuple::tuple_i;
use crate::tuple::point;
use crate::tuple::vector;


// #[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix2x2 {
    r1c1: f32,
    r1c2: f32,
    
    r2c1: f32,
    r2c2: f32
}

// #[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix3x3 {
    r1c1: f32,
    r1c2: f32,
    r1c3: f32,
    
    r2c1: f32,
    r2c2: f32,
    r2c3: f32,
    
    r3c1: f32,
    r3c2: f32,
    r3c3: f32
}

// #[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    r1c1: f32,
    r1c2: f32,
    r1c3: f32,
    r1c4: f32,

    r2c1: f32,
    r2c2: f32,
    r2c3: f32,
    r2c4: f32,
    
    r3c1: f32,
    r3c2: f32,
    r3c3: f32,
    r3c4: f32,
    
    r4c1: f32,
    r4c2: f32,
    r4c3: f32,
    r4c4: f32
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, other: &Self) -> bool {
        fequal(self.r1c1, other.r1c1) &&
        fequal(self.r1c2, other.r1c2) &&
        fequal(self.r2c1, other.r2c1) &&
        fequal(self.r2c2, other.r2c2)
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

pub fn matrix2x2_create(
    a: f32, b: f32, 
    c: f32, d: f32) -> Matrix2x2 {
     Matrix2x2 {
        r1c1: a, r1c2: b,
        r2c1: c, r2c2: d
    }
}

fn matrix2x2_create_i (
    a:i32,b:i32,
    c:i32,d:i32) -> Matrix2x2
{
    matrix2x2_create(
        a as f32, b as f32, 
        c as f32, d as f32) 
}

pub fn matrix3x3_create(
    a: f32, b: f32, c: f32, 
    d: f32, e: f32, f: f32, 
    g: f32, h: f32, i: f32) -> Matrix3x3 {
    Matrix3x3 {
        r1c1: a, r1c2: b, r1c3: c,
        r2c1: d, r2c2: e, r2c3: f,
        r3c1: g, r3c2: h, r3c3: i
    }    
}

pub fn matrix3x3_create_i(
    a: i32, b: i32, c: i32, 
    d: i32, e: i32, f: i32, 
    g: i32, h: i32, i: i32) -> Matrix3x3 {
    matrix3x3_create(
        a as f32, b as f32, c as f32,
        d as f32, e as f32, f as f32,
        g as f32, h as f32, i as f32
    )
}

pub fn matrix4x4_create(
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

pub fn matrix4x4_create_i(
    a: i32, b: i32, c: i32, d: i32, 
    e: i32, f: i32, g: i32, h: i32,
    i: i32, j: i32, k: i32, l: i32,
    m: i32, n: i32, o: i32, p: i32) -> Matrix4x4 {
    matrix4x4_create (
        a as f32, b as f32, c as f32, d as f32,
        e as f32, f as f32, g as f32, h as f32,
        i as f32, j as f32, k as f32, l as f32,
        m as f32, n as f32, o as f32, p as f32
    )
}

fn matrix2x2_transpose(m: &Matrix2x2) -> Matrix2x2 {
    matrix2x2_create(
        m.r1c1, m.r2c1,
        m.r1c2, m.r2c2
    )
}

fn matrix3x3_transpose(m: &Matrix3x3) -> Matrix3x3 {
    matrix3x3_create(
        m.r1c1, m.r2c1, m.r3c1,
        m.r1c2, m.r2c2, m.r3c2,
        m.r1c3, m.r2c3, m.r3c3
    )
}

fn matrix4x4_transpose(m: &Matrix4x4) -> Matrix4x4 {
    matrix4x4_create(
        m.r1c1, m.r2c1, m.r3c1, m.r4c1,
        m.r1c2, m.r2c2, m.r3c2, m.r4c2,
        m.r1c3, m.r2c3, m.r3c3, m.r4c3,
        m.r1c4, m.r2c4, m.r3c4, m.r4c4
    )
}

fn matrix2x2_determinant(m: &Matrix2x2) -> f32 {
    m.r1c1 * m.r2c2 - m.r1c2 * m.r2c1
}

fn matrix3x3_determinant(m: &Matrix3x3) -> f32 {
    let a = matrix3x3_cofactor(&m, 0, 0);
    let b = matrix3x3_cofactor(&m, 0, 1);
    let c = matrix3x3_cofactor(&m, 0, 2);
    a * m.r1c1 + b * m.r1c2 + c * m.r1c3
}

fn matrix4x4_determinant(m: &Matrix4x4) -> f32 {
    let a = matrix4x4_cofactor(&m, 0, 0);
    let b = matrix4x4_cofactor(&m, 0, 1);
    let c = matrix4x4_cofactor(&m, 0, 2);
    let d = matrix4x4_cofactor(&m, 0, 3);
    a * m.r1c1 + b * m.r1c2 + c * m.r1c3 + d * m.r1c4
}

/*
    getting the submatrix of row, col

    submatrix of 3x3 returns matrix 2x2 created
    by excluding the input row and col
*/
fn matrix3x3_submatrix(m: &Matrix3x3, row: u8, col: u8) -> Matrix2x2 {
    let arr = [
            m.r1c1, m.r1c2, m.r1c3,
            m.r2c1, m.r2c2, m.r2c3,
            m.r3c1, m.r3c2, m.r3c3];
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
    matrix2x2_create(t[0],t[1],t[2],t[3])
}

/*
    getting the submatrix of row, col

    submatrix of 4x4 returns matrix 3x3 created
    by excluding the input row and col
*/
fn matrix4x4_submatrix(m: &Matrix4x4, row: u8, col: u8) -> Matrix3x3 {
    let arr = [
            m.r1c1, m.r1c2, m.r1c3, m.r1c4,
            m.r2c1, m.r2c2, m.r2c3, m.r2c4,
            m.r3c1, m.r3c2, m.r3c3, m.r3c4,
            m.r4c1, m.r4c2, m.r4c3, m.r4c4];
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
    matrix3x3_create(t[0],t[1],t[2],t[3],t[4],t[5],t[6],t[7],t[8])
}

/*
    getting the minor of row, cell

    the minor is the determinant of the submatrix at row, col
*/
fn matrix3x3_minor(m: &Matrix3x3, row: u8, col: u8) -> f32 {
    let submatrix = matrix3x3_submatrix(&m, row, col);
    matrix2x2_determinant(&submatrix)
}

fn matrix4x4_minor(m: &Matrix4x4, row: u8, col: u8) -> f32 {
    let submatrix = matrix4x4_submatrix(&m, row, col);
    matrix3x3_determinant(&submatrix)
}

fn cofactor(minor: f32, row: u8, col: u8) -> f32 {
    if (row + col) % 2 == 0 {
        return minor;
    }
    else {
        return -minor;
    }
}

fn matrix3x3_cofactor(m: &Matrix3x3, row: u8, col: u8) -> f32 {
    let minor = matrix3x3_minor(&m, row, col);
    cofactor(minor, row, col)
}

fn matrix4x4_cofactor(m: &Matrix4x4, row: u8, col: u8) -> f32 {
    let minor = matrix4x4_minor(&m, row, col);
    cofactor(minor, row, col)
}

fn matrix4x4_inverse(m: &Matrix4x4) -> Matrix4x4 {
    let determinant = matrix4x4_determinant(&m);
    if determinant == 0.0 {
        panic!("Attempting to invert and invertible matrix");
    }
    let mut t: [f32; 16] = [0.0; 16];
    for row in 0..4 {
        for col in 0..4 {
            let c = matrix4x4_cofactor(&m, row, col);
            let t_index = (col * 4 + row) as usize;
            t[t_index] = c / determinant;
        }
    }
    matrix4x4_create(t[0],t[1],t[2],t[3],t[4],t[5],t[6],t[7],t[8],t[9],t[10],t[11],t[12],t[13],t[14],t[15])
}

fn translation_create(x: f32, y: f32, z: f32) -> Matrix4x4 {
    let mut a = MATRIX_4X4_IDENTITY;
    a.r1c4 = x;
    a.r2c4 = y;
    a.r3c4 = z;
    return a;
}

#[test]
fn matrix2x2_create_test() {
    let a = matrix2x2_create_i(1,2,3,4);
    let b = Matrix2x2 {
        r1c1: 1.0, r1c2: 2.0,
        r2c1: 3.0, r2c2: 4.0
    };
    let c = matrix2x2_create_i(-1,2,3,4);
    let d = matrix2x2_create_i(1,-1,3,4);
    let e = matrix2x2_create_i(1,2,-1,4);
    let f = matrix2x2_create_i(1,2,3,-1);
    assert_eq!(a, b);
    assert!(a != c);
    assert!(a != d);
    assert!(a != e);
    assert!(a != f);
}

#[test]
fn matrix3x3_create_test() {
    let a = matrix3x3_create_i(1,2,3,4,5,6,7,8,9);
    let b = Matrix3x3 {
        r1c1: 1.0, r1c2: 2.0, r1c3: 3.0,
        r2c1: 4.0, r2c2: 5.0, r2c3: 6.0, 
        r3c1: 7.0, r3c2: 8.0, r3c3: 9.0
    };
    let c = matrix3x3_create_i(-1,2,3,4,5,6,7,8,9);
    let d = matrix3x3_create_i(1,-1,3,4,5,6,7,8,9);
    let e = matrix3x3_create_i(1,2,-1,4,5,6,7,8,9);
    let f = matrix3x3_create_i(1,2,3,-1,5,6,7,8,9);
    let g = matrix3x3_create_i(1,2,3,4,-1,6,7,8,9);
    let h = matrix3x3_create_i(1,2,3,4,5,-1,7,8,9);
    let i = matrix3x3_create_i(1,2,3,4,5,6,-1,8,9);
    let j = matrix3x3_create_i(1,2,3,4,5,6,7,-1,9);
    let k = matrix3x3_create_i(1,2,3,4,5,6,7,8,-1);
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
    let a = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    let b = Matrix4x4 {
        r1c1: 1.0, r1c2: 2.0, r1c3: 3.0, r1c4: 4.0,
        r2c1: 5.0, r2c2: 6.0, r2c3: 7.0, r2c4: 8.0,
        r3c1: 9.0, r3c2: 10.0, r3c3: 11.0, r3c4: 12.0,
        r4c1: 13.0, r4c2: 14.0, r4c3: 15.0, r4c4: 16.0
    };
    let c = matrix4x4_create_i(-1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    let d = matrix4x4_create_i(1,-1,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    let e = matrix4x4_create_i(1,2,-1,4,5,6,7,8,9,10,11,12,13,14,15,16);
    let f = matrix4x4_create_i(1,2,3,-1,5,6,7,8,9,10,11,12,13,14,15,16);
    let g = matrix4x4_create_i(1,2,3,4,-1,6,7,8,9,10,11,12,13,14,15,16);
    let h = matrix4x4_create_i(1,2,3,4,5,-1,7,8,9,10,11,12,13,14,15,16);
    let i = matrix4x4_create_i(1,2,3,4,5,6,-1,8,9,10,11,12,13,14,15,16);
    let j = matrix4x4_create_i(1,2,3,4,5,6,7,-1,9,10,11,12,13,14,15,16);
    let k = matrix4x4_create_i(1,2,3,4,5,6,7,8,-1,10,11,12,13,14,15,16);
    let l = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,-1,11,12,13,14,15,16);
    let m = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,10,-1,12,13,14,15,16);
    let n = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,10,11,-1,13,14,15,16);
    let o = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,10,11,12,-1,14,15,16);
    let p = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,10,11,12,13,-1,15,16);
    let q = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,10,11,12,13,14,-1,16);
    let r = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,-1);
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
    let a = matrix2x2_create_i(1,3,5,7);
    let b = matrix2x2_create_i(2,4,6,8);
    let r = a * b;
    let t = matrix2x2_create_i(20,28,52,76);
    assert_eq!(r, t);
}

#[test]
fn matrix3x3_mul_test() {
    let a = matrix3x3_create_i(1,3,5,7,9,11,13,15,17);
    let b = matrix3x3_create_i(2,4,6,8,10,12,14,16,18);
    let r = a * b;
    let t = matrix3x3_create_i(96,114,132,240,294,348,384,474,564);
    assert_eq!(r, t);
}

#[test]
fn matrix4x4_mul_test() {
    let a = matrix4x4_create_i(1,3,5,7,9,11,13,15,17,19,21,23,25,27,29,31);
    let b = matrix4x4_create_i(2,4,6,8,10,12,14,16,18,20,22,24,26,28,30,32);
    let r = a * b;
    let t = matrix4x4_create_i(304,336,368,400,752,848,944,1040,1200,1360,1520,1680,1648,1872,2096,2320);
    assert_eq!(r, t);
}

#[test]
fn matrix4x4_mul_tuple_test() {
    let a = matrix4x4_create_i(1,3,5,7,9,11,13,15,17,19,21,23,25,27,29,31);
    let b = tuple_i(2,4,8,10);
    let r = a * b;
    let t = tuple_i(124,316,508,700);
    assert_eq!(r, t);
}

#[test]
fn matrix_identity_test() {
    let a = matrix2x2_create_i(1,2,3,4);
    let b = matrix3x3_create_i(1,2,3,4,5,6,7,8,9);
    let c = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);

    let ar = a * MATRIX_2X2_IDENTITY;
    let br = b * MATRIX_3X3_IDENTITY;
    let cr = c * MATRIX_4X4_IDENTITY;

    assert_eq!(a, ar);
    assert_eq!(b, br);
    assert_eq!(c, cr);
}

#[test]
fn matrix_transpose_test() {
    let a = matrix2x2_create_i(1,2,3,4);
    let b = matrix3x3_create_i(1,2,3,4,5,6,7,8,9);
    let c = matrix4x4_create_i(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);

    let ar = matrix2x2_transpose(&a);
    let br = matrix3x3_transpose(&b);
    let cr = matrix4x4_transpose(&c);

    let at = matrix2x2_create_i(1,3,2,4);
    let bt = matrix3x3_create_i(1,4,7,2,5,8,3,6,9);
    let ct = matrix4x4_create_i(1,5,9,13,2,6,10,14,3,7,11,15,4,8,12,16);

    assert_eq!(ar, at);
    assert_eq!(br, bt);
    assert_eq!(cr, ct);
}

#[test]
fn matrix_determinant_test() {
    let a = matrix2x2_create_i(1,5,-3,2);
    let ad = matrix2x2_determinant(&a);
    assert_eq!(ad, 17.0);
}

#[test]
fn matrix3x3_submatrix_test() {
    let a = matrix3x3_create_i(1,2,3,4,5,6,7,8,9);
    let b = matrix3x3_submatrix(&a, 0, 2);
    let c = matrix2x2_create_i(4,5,7,8);
    assert_eq!(b, c);
}

#[test]
fn matrix4x4_submatrix_test() {
    let a = matrix4x4_create_i(1,2,3,4, 5,6,7,8, 9,10,11,12, 13,14,15,16);
    let b = matrix4x4_submatrix(&a, 2, 1);
    let c = matrix3x3_create_i(1,3,4, 5,7,8, 13,15,16);
    assert_eq!(b, c);
}

#[test]
fn matrix3x3_minor_cofactor_test() {
    let a = matrix3x3_create_i(3,5,0,2,-1,-7,6,-1,5);
    let b = matrix3x3_minor(&a, 0, 0);
    let c = matrix3x3_cofactor(&a, 0, 0);
    let d = matrix3x3_minor(&a, 1, 0);
    let e = matrix3x3_cofactor(&a, 1, 0);
    assert_eq!(b, -12.0);
    assert_eq!(c, -12.0);
    assert_eq!(d, 25.0);
    assert_eq!(e, -25.0);
}

#[test]
fn matrix3x3_deteriminant_test() {
    let a = matrix3x3_create_i(1,2,6,-5,8,-4,2,6,4);
    assert_eq!(matrix3x3_cofactor(&a,0,0), 56.0);
    assert_eq!(matrix3x3_cofactor(&a,0,1), 12.0);
    assert_eq!(matrix3x3_cofactor(&a,0,2), -46.0);
    assert_eq!(matrix3x3_determinant(&a), -196.0);
}

#[test]
fn matrix4x4_determinant_test() {
    let a = matrix4x4_create_i(-2,-8,3,5, -3,1,7,3, 1,2,-9,6, -6,7,7,-9);
    assert_eq!(matrix4x4_cofactor(&a,0,0), 690.0);
    assert_eq!(matrix4x4_cofactor(&a,0,1), 447.0);
    assert_eq!(matrix4x4_cofactor(&a,0,2), 210.0);
    assert_eq!(matrix4x4_cofactor(&a,0,3), 51.0);
    assert_eq!(matrix4x4_determinant(&a), -4071.0);
}

#[test]
fn matrix4x4_test_for_invertibility_test() {
    let a = matrix4x4_create_i(6,4,4,4, 5,5,7,6, 4,-9,3,-7, 9,1,7,-6);
    let b = matrix4x4_create_i(-4,2,-2,-3, 9,6,2,6, 0,-5,1,-5, 0,0,0,0);
    assert_eq!(matrix4x4_determinant(&a), -2120.0);
    assert_eq!(matrix4x4_determinant(&b), 0.0);
}

#[test]
fn matrix4x4_inverse_test1() {
    let a = matrix4x4_create_i(-5,2,6,-8, 1,-5,1,8, 7,7,-6,-7, 1,-3,7,4);
    let b = matrix4x4_inverse(&a);
    assert_eq!(matrix4x4_determinant(&a), 532.0);
    assert_eq!(matrix4x4_cofactor(&a,2,3), -160.0);
    assert!(fequal(b.r4c3, -160.0/532.0));
    //assert_eq!(b.r4c3, -160.0/532.0);
    assert_eq!(matrix4x4_cofactor(&a,3,2), 105.0);
    assert!(fequal(b.r3c4, 105.0/532.0));
    let c = matrix4x4_create(
        0.21805, 0.45113, 0.24060, -0.04511,
        -0.80827, -1.45677, -0.44361, 0.52068,
        -0.07895, -0.22368, -0.05263, 0.19737,
        -0.52256, -0.81391, -0.30075, 0.30639);
    assert_eq!(b, c);
}

#[test]
fn matrix4x4_inverse_test2() {
    let a = matrix4x4_create_i(8,-5,9,2,7,5,6,1,-6,0,9,6,-3,0,-9,-4);
    let b = matrix4x4_inverse(&a);
    let c = matrix4x4_create(
        -0.15385, -0.15385, -0.28205, -0.53846,
        -0.07692, 0.12308, 0.02564, 0.03077,
        0.35897, 0.35897, 0.43590, 0.92308,
        -0.69231, -0.69231, -0.76923, -1.92308);
    assert_eq!(b, c);
}

#[test]
fn matrix4x4_inverse_test3() {
    let a = matrix4x4_create_i(9,3,0,9,-5,-2,-6,-3,-4,9,6,4,-7,6,6,2);
    let b = matrix4x4_inverse(&a);
    let c = matrix4x4_create(
        -0.04074, -0.07778, 0.14444, -0.22222,
        -0.07778, 0.03333, 0.36667, -0.33333,
        -0.02901, -0.14630, -0.10926, 0.12963,
        0.17778, 0.06667, -0.26667, 0.33333);
    assert_eq!(b, c);
}

#[test]
fn matrix4x4_multiply_inverse_test() {
    let a = matrix4x4_create_i(3,-9,7,3, 3,-8,2,-9, -4,4,4,1, -6,5,-1,1);
    let b = matrix4x4_create_i(8,2,2,2, 3,-1,7,0, 7,0,5,4, 6,-2,0,5);
    let c = a * b;
    let d = matrix4x4_inverse(&b);
    let e = c * d;
    assert_eq!(a, e);
}

#[test]
fn translate_test() {
    let p = point(-3.0, 4.0, 5.0);
    let v = vector(-3.0, 4.0, 5.0);

    // moves point in direction of translation vector
    let translate = translation_create(5.0, -3.0, 2.0);
    assert_eq!(translate * p, point(2.0, 1.0, 7.0));

    // inverting moves point in oposite direction
    let inverse = matrix4x4_inverse(&translate);
    assert_eq!(inverse * p, point(-8.0, 7.0, 3.0));

    // translation matrix does not affect vectors
    assert_eq!(translate * v, vector(-3.0, 4.0, 5.0));
}