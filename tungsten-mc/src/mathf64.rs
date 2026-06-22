#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn fract(&self) -> Vec3 {
        Vec3::new(self.x.fract(), self.y.fract(), self.z.fract())
    }
    pub fn floor(&self) -> Vec3 {
        Vec3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl std::ops::Mul<i32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: i32) -> Vec3 {
        Vec3::new(
            self.x * scalar as f64,
            self.y * scalar as f64,
            self.z * scalar as f64,
        )
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pos3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos3 {
    #[inline(always)]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::Mul<i32> for Pos3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, scalar: i32) -> Pos3 {
        Pos3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl std::ops::Add for Pos3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Pos3) -> Pos3 {
        Pos3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Pos3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Pos3) -> Pos3 {
        Pos3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul for Pos3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Pos3) -> Pos3 {
        Pos3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl std::ops::Add<i32> for Pos3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, scalar: i32) -> Pos3 {
        Pos3::new(self.x + scalar, self.y + scalar, self.z + scalar)
    }
}

impl std::ops::Add<Pos3> for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Pos3) -> Vec3 {
        Vec3::new(
            self.x + other.x as f64,
            self.y + other.y as f64,
            self.z + other.z as f64,
        )
    }
}

impl std::ops::Sub<Pos3> for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Pos3) -> Vec3 {
        Vec3::new(
            self.x - other.x as f64,
            self.y - other.y as f64,
            self.z - other.z as f64,
        )
    }
}

impl std::ops::Mul<Pos3> for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Pos3) -> Vec3 {
        Vec3::new(
            self.x * other.x as f64,
            self.y * other.y as f64,
            self.z * other.z as f64,
        )
    }
}

impl std::ops::Mul<Vec3> for Pos3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x as f64 * other.x,
            self.y as f64 * other.y,
            self.z as f64 * other.z,
        )
    }
}

#[inline(always)]
pub fn as_index(pos: Pos3, size_x: i32, size_y: i32, _size_z: i32) -> usize {
    (pos.z * size_y * size_x + pos.y * size_x + pos.x) as usize
}

#[inline(always)]
pub fn index(pos: PositionIterator) -> usize {
    pos.idx
}

#[inline(always)]
pub fn flat_y_zero_index(pos: Pos3, size_x: i32, _size_z: i32) -> usize {
    // the dimensions have been reduced to 2D by flattening the y dimension, so the index is just x + z * size_x
    (pos.z * size_x + pos.x) as usize
}

#[inline(always)]
pub fn flat_z_zero_index(pos: Pos3, size_x: i32, size_y: i32) -> usize {
    // the dimensions have been reduced to 2D by flattening the z dimension, so the index is just x + y * size_x
    (pos.y * size_x + pos.x) as usize
}

#[inline(always)]
pub fn pow(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

pub struct Iter3D {
    idx: usize,
    x: i32,
    y: i32,
    z: i32,
    pub mx: i32,
    pub my: i32,
    pub mz: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct PositionIterator {
    pub pos: Pos3,
    pub idx: usize,
}

impl Iterator for Iter3D {
    type Item = PositionIterator;
    #[inline(always)]
    fn next(&mut self) -> Option<PositionIterator> {
        if self.z >= self.mz {
            return None;
        }
        let current = self.always_next();
        Some(current)
    }
}

pub struct ComplexIter3D {
    inner: Iter3D,
}

impl Iterator for ComplexIter3D {
    type Item = (usize, [Pos3; 4]);
    #[inline(always)]
    fn next(&mut self) -> Option<(usize, [Pos3; 4])> {
        if self.inner.z >= self.inner.mz {
            return None;
        }

        // the logic is: simply do 4 iterations of the inner iterator and collect the results into an array
        let mut results = [Pos3::new(0, 0, 0); 4];
        for i in 0..4 {
            results[i] = self.inner.always_next().pos;
        }
        Some((self.inner.idx - 4, results))
    }
}

impl Iter3D {
    pub fn always_next(&mut self) -> PositionIterator {
        let current = PositionIterator {
            pos: Pos3::new(self.x, self.y, self.z),
            idx: self.idx,
        };
        self.x += 1;
        self.idx += 1;
        if self.x >= self.mx {
            self.x = 0;
            self.y += 1;
            if self.y >= self.my {
                self.y = 0;
                self.z += 1;
            }
        }
        current
    }

    pub fn chunk(&self) -> ComplexIter3D {
        ComplexIter3D {
            inner: Iter3D {
                idx: self.idx,
                x: self.x,
                y: self.y,
                z: self.z,
                mx: self.mx,
                my: self.my,
                mz: self.mz,
            },
        }
    }
}

impl ExactSizeIterator for Iter3D {
    fn len(&self) -> usize {
        (self.mx * self.my * self.mz) as usize
    }
}

#[inline(always)]
pub fn iter_usize(i: PositionIterator) -> usize {
    i.idx
}

impl Into<Pos3> for PositionIterator {
    #[inline(always)]
    fn into(self) -> Pos3 {
        self.pos
    }
}

#[inline(always)]
pub fn iter_3d(mx: i32, my: i32, mz: i32) -> Iter3D {
    Iter3D {
        idx: 0,
        x: 0,
        y: 0,
        z: 0,
        mx,
        my,
        mz,
    }
}

// ==========================================
// Interpolation functions
// ==========================================

#[inline]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

#[inline]
fn lerp_f32(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

#[inline]
pub fn interpolate(
    v000: f64,
    v100: f64,
    v010: f64,
    v110: f64,
    v001: f64,
    v101: f64,
    v011: f64,
    v111: f64,
    fx: f64,
    fy: f64,
    fz: f64,
) -> f64 {
    // Interpolate along X
    let x00 = lerp(v000, v100, fx);
    let x10 = lerp(v010, v110, fx);
    let x01 = lerp(v001, v101, fx);
    let x11 = lerp(v011, v111, fx);

    // Interpolate along Y
    let y0 = lerp(x00, x10, fy);
    let y1 = lerp(x01, x11, fy);

    // Interpolate along Z
    lerp(y0, y1, fz)
}

#[inline]
fn base_grid(pos3: Pos3) -> Pos3 {
    Pos3 {
        x: pos3.x >> 2,
        y: pos3.y >> 3,
        z: pos3.z >> 2,
    }
}

#[inline]
pub fn cornerx0y0z0_8(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid(pos3);
    let npos = Pos3 {
        x: g.x,
        y: g.y,
        z: g.z,
    };
    as_index(npos, sx, sy, sz)
}

#[inline]
pub fn cornerx4y0z0_8(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid(pos3);
    let npos = Pos3 {
        x: g.x + 1,
        y: g.y,
        z: g.z,
    };
    as_index(npos, sx, sy, sz)
}

#[inline]
pub fn cornerx0y8z0_8(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid(pos3);
    let npos = Pos3 {
        x: g.x,
        y: g.y + 1,
        z: g.z,
    };
    as_index(npos, sx, sy, sz)
}

#[inline]
pub fn cornerx4y8z0_8(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid(pos3);
    let npos = Pos3 {
        x: g.x + 1,
        y: g.y + 1,
        z: g.z,
    };
    as_index(npos, sx, sy, sz)
}

#[inline]
pub fn cornerx0y0z4_8(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid(pos3);
    let npos = Pos3 {
        x: g.x,
        y: g.y,
        z: g.z + 1,
    };
    as_index(npos, sx, sy, sz)
}

#[inline]
pub fn cornerx4y0z4_8(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid(pos3);
    let npos = Pos3 {
        x: g.x + 1,
        y: g.y,
        z: g.z + 1,
    };
    as_index(npos, sx, sy, sz)
}

#[inline]
pub fn cornerx0y8z4_8(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid(pos3);
    let npos = Pos3 {
        x: g.x,
        y: g.y + 1,
        z: g.z + 1,
    };
    let i = as_index(npos, sx, sy, sz);
    i
}

#[inline]
pub fn cornerx4y8z4_8(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid(pos3);
    let npos = Pos3 {
        x: g.x + 1,
        y: g.y + 1,
        z: g.z + 1,
    };
    as_index(npos, sx, sy, sz)
}

#[inline]
pub fn xfract4(pos3: Pos3) -> f64 {
    (pos3.x & 3) as f64 / 4.0
}

#[inline]
pub fn yfract8(pos3: Pos3) -> f64 {
    (pos3.y & 7) as f64 / 8.0
}

#[inline]
pub fn yfract16(pos3: Pos3) -> f64 {
    (pos3.y & 15) as f64 / 16.0
}

#[inline]
pub fn zfract4(pos3: Pos3) -> f64 {
    (pos3.z & 3) as f64 / 4.0
}

// ==========================================
// 4x16x4 grid helpers (y cell size = 16)
// ==========================================

#[inline(always)]
fn base_grid_16(pos3: Pos3) -> Pos3 {
    Pos3 {
        x: pos3.x >> 2,
        y: pos3.y >> 4,
        z: pos3.z >> 2,
    }
}

#[inline]
pub fn cornerx0y0z0_16(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid_16(pos3);
    as_index(
        Pos3 {
            x: g.x,
            y: g.y,
            z: g.z,
        },
        sx,
        sy,
        4,
    )
}

#[inline]
pub fn cornerx4y0z0_16(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid_16(pos3);
    as_index(
        Pos3 {
            x: g.x + 1,
            y: g.y,
            z: g.z,
        },
        sx,
        sy,
        4,
    )
}

#[inline]
pub fn cornerx0y16z0_16(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid_16(pos3);
    as_index(
        Pos3 {
            x: g.x,
            y: g.y + 1,
            z: g.z,
        },
        sx,
        sy,
        4,
    )
}

#[inline]
pub fn cornerx4y16z0_16(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid_16(pos3);
    as_index(
        Pos3 {
            x: g.x + 1,
            y: g.y + 1,
            z: g.z,
        },
        sx,
        sy,
        4,
    )
}

#[inline]
pub fn cornerx0y0z4_16(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid_16(pos3);
    as_index(
        Pos3 {
            x: g.x,
            y: g.y,
            z: g.z + 1,
        },
        sx,
        sy,
        4,
    )
}

#[inline]
pub fn cornerx4y0z4_16(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid_16(pos3);
    as_index(
        Pos3 {
            x: g.x + 1,
            y: g.y,
            z: g.z + 1,
        },
        sx,
        sy,
        4,
    )
}

#[inline]
pub fn cornerx0y16z4_16(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid_16(pos3);
    as_index(
        Pos3 {
            x: g.x,
            y: g.y + 1,
            z: g.z + 1,
        },
        sx,
        sy,
        4,
    )
}

#[inline]
pub fn cornerx4y16z4_16(pos3: Pos3, sx: i32, sy: i32, sz: i32) -> usize {
    let g = base_grid_16(pos3);
    as_index(
        Pos3 {
            x: g.x + 1,
            y: g.y + 1,
            z: g.z + 1,
        },
        sx,
        sy,
        sz,
    )
}

#[inline]
pub fn biome_column_index(pos3: Pos3) -> usize {
    let npos = Pos3 {
        x: pos3.x >> 2,
        y: 0,
        z: pos3.z >> 2,
    };
    flat_y_zero_index(npos, 5, 5)
}
