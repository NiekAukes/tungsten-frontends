#pragma once
#include "helper_math.h"
#include "xoroshiro.hpp"
#include <array>
#include <cstddef>
#include <vector>
#include <cstdint>
#include <cstdio>
#include <memory>


__host__ __device__ inline double3 operator*(int3 a, double3 b) {
    return make_double3(a.x * b.x, a.y * b.y, a.z * b.z);
}
__host__ __device__ inline double3 operator*(double3 a, int3 b) {
    return make_double3(a.x * b.x, a.y * b.y, a.z * b.z);
}

__host__ __device__ inline double3 operator*(double3 a, double b) {
    return make_double3(a.x * b, a.y * b, a.z * b);
}

__host__ __device__ inline double3 operator-(double3 a, double b) {
    return make_double3(a.x - b, a.y - b, a.z - b);
}

__host__ __device__ inline double3 operator+(double3 a, double b) {
    return make_double3(a.x + b, a.y + b, a.z + b);
}

__host__ __device__ inline double3 operator/(double3 a, double b) {
    return make_double3(a.x / b, a.y / b, a.z / b);
}

// __host__ __device__ inline int3 operator*(int3 a, int3 b) {
//     return make_int3(a.x * b.x, a.y * b.y, a.z * b.z);
// }

// __host__ __device__ inline double3 operator*(double3 a, float b) {
//     return make_double3(a.x * b, a.y * b, a.z * b);
// }

__host__ __device__ inline double3 operator+(double3 a, double3 b) {
    return make_double3(a.x + b.x, a.y + b.y, a.z + b.z);
}
__host__ __device__ inline double3 operator*(double3 a, double3 b) {
    return make_double3(a.x * b.x, a.y * b.y, a.z * b.z);
}
__host__ __device__ inline double3 operator-(double3 a, double3 b) {
    return make_double3(a.x - b.x, a.y - b.y, a.z - b.z);
}

__host__ __device__ inline double3 operator+(int3 a, double3 b) {
    return make_double3(a.x + b.x, a.y + b.y, a.z + b.z);
}
__host__ __device__ inline double3 operator+(double3 a, int3 b) {
    return make_double3(a.x + b.x, a.y + b.y, a.z + b.z);
}


// __host__ __device__ inline int3 operator+(int3 a, int3 b) {
//     return make_int3(a.x + b.x, a.y + b.y, a.z + b.z);
// }

// __host__ __device__ inline double3 operator-(double3 a, double3 b) {
//     return make_double3(a.x - b.x, a.y - b.y, a.z - b.z);
// }

// __host__ __device__ inline double3 operator-(double3 a, double b) {
//     return make_double3(a.x - b, a.y - b, a.z - b);
// }

// __host__ __device__ inline double3 operator+(double3 a, double b) {
//     return make_double3(a.x + b, a.y + b, a.z + b);
// }

// __host__ __device__ inline double3 operator/(double3 a, double b) {
//     return make_double3(a.x / b, a.y / b, a.z / b);
// }

__host__ __device__ inline double2 operator-(double2 a, double2 b) {
    return make_double2(a.x - b.y, a.y - b.y);
}
__host__ __device__ inline double2 operator*(double2 a, double b) {
    return make_double2(a.x * b, a.y * b);
}

__host__ __device__ inline double2 operator+(double2 a, double b) {
    return make_double2(a.x + b, a.y + b);
}
__host__ __device__ inline double2 operator+(double2 a, double2 b) {
    return make_double2(a.x + b.x, a.y + b.y);
}

__host__ __device__ inline double4 operator*(double4 a, double b) {
    return make_double4(a.x * b, a.y * b, a.z * b, a.w * b);
}
__host__ __device__ inline double4 operator-(double4 a, double4 b) {
    return make_double4(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w);
}
__host__ __device__ inline double4 operator+(double4 a, double4 b) {
    return make_double4(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w);
}

template<typename T, typename T2, typename T3>
__device__ T clamp(T value, T2 min_val, T3 max_val) {
    return value < min_val ? min_val : (value > max_val ? max_val : value);
}


inline __device__ __host__ double lerp(double a, double b, double t)
{
    return a + t*(b-a);
}
inline __device__ __host__ double2 lerp(double2 a, double2 b, double t)
{
    return a + (b-a)*t;
}
inline __device__ __host__ double3 lerp(double3 a, double3 b, double t)
{
    return a + (b-a)*t;
}
inline __device__ __host__ double4 lerp(double4 a, double4 b, double t)
{
    return a + (b-a)*t;
}

// Cubic Hermite spline interpolation. Matches hermite() in utilsf64.rs.
//   t  — interpolation parameter in [0, 1]
//   p0 — start value
//   p1 — end value
//   m0 — start tangent
//   m1 — end tangent


__device__ __forceinline__ float hermite(float t, float p0, float p1, float m0, float m1, float h_minus_g) {
   /*
   // 1. Compute intermediate tangents matching Minecraft's:
    // float p = l * (h - g) - (o - n);
    // float q = -m * (h - g) + (o - n);
    let p = (m0 * h_minus_g) - (p1 - p0);
    let q = (-m1 * h_minus_g) + (p1 - p0);

    // 2. Perform localized 32-bit linear interpolations
    let lerp1 = p0 + t * (p1 - p0);
    let lerp2 = p + t * (q - p);

    // 3. Enforce left-to-right evaluation grouping via parentheses
    lerp1 + ((t * (1.0_f32 - t)) * lerp2)*/
    
    // 1. Compute intermediate tangents matching Minecraft's:
    // float p = l * (h - g) - (o - n);
    // float q = -m * (h - g) + (o - n);
    float p = (m0 * h_minus_g) - (p1 - p0);
    float q = (-m1 * h_minus_g) + (p1 - p0);

    // 2. Perform localized 32-bit linear interpolations
    float lerp1 = p0 + t * (p1 - p0);
    float lerp2 = p + t * (q - p);

    // 3. Enforce left-to-right evaluation grouping via parentheses
    return lerp1 + ((t * (1.0f - t)) * lerp2);
}

// Minecraft Perlin noise — matches sample_perlin() in perlin.rs.
//
// NOTE: The per-sampler origin offsets (pns.origin_x/y/z) are NOT applied here.
// The caller must pre-apply them to `pos` before calling (or fold them into
// the origin uniform), since they cannot be stored alongside the perm table
// with the current binding layout.

__device__ int perlin_perm_(const int* perm, int index) {
    return perm[index & 255] & 255;
}

__constant__ float GRADX[16] = { 1.f, -1.f,  1.f, -1.f,  1.f, -1.f,  1.f, -1.f,  0.f,  0.f,  0.f,  0.f,  1.f,  0.f, -1.f,  0.f };
__constant__ float GRADY[16] = { 1.f,  1.f, -1.f, -1.f,  0.f,  0.f,  0.f,  0.f,  1.f, -1.f,  1.f, -1.f,  1.f, -1.f,  1.f, -1.f };
__constant__ float GRADZ[16] = { 0.f,  0.f,  0.f,  0.f,  1.f,  1.f, -1.f, -1.f,  1.f,  1.f, -1.f, -1.f,  0.f,  1.f,  0.f, -1.f };

// Dot product with one of Minecraft's 16 gradient vectors (GRAD3 in perlin.rs).
__device__ float perlin_grad_(int hash, float x, float y, float z) {
    unsigned int h = hash & 15;
    return GRADX[h] * x + GRADY[h] * y + GRADZ[h] * z;
}

// Quintic fade curve: 6t^5 - 15t^4 + 10t^3
__device__ float perlin_fade_(float t) {
    return t * t * t * (t * (t * 6.0f - 15.0f) + 10.0f);
}

__device__ float perlin_lerp_(float delta, float a, float b) {
    return a + delta * (b - a);
}

struct PerlinNoiseGenerator {
    int perm[256];
    float origin_x;
    float origin_y;
    float origin_z;
};

struct InterpolatedNoiseSamplerGPU {
    PerlinNoiseGenerator lower[16];
    PerlinNoiseGenerator upper[16];
    PerlinNoiseGenerator interpolation[8];
};

__constant__ float4 GRADS[16] = {
    { 1.f, 1.f, 0.f, 0.f },  { -1.f, 1.f, 0.f, 0.f }, { 1.f, -1.f, 0.f, 0.f }, { -1.f, -1.f, 0.f, 0.f },
    { 1.f, 0.f, 1.f, 0.f },  { -1.f, 0.f, 1.f, 0.f }, { 1.f, 0.f, -1.f, 0.f }, { -1.f, 0.f, -1.f, 0.f },
    { 0.f, 1.f, 1.f, 0.f },  { 0.f, -1.f, 1.f, 0.f }, { 0.f, 1.f, -1.f, 0.f }, { 0.f, -1.f, -1.f, 0.f },
    { 1.f, 1.f, 0.f, 0.f },  { 0.f, -1.f, 1.f, 0.f }, { -1.f, 1.f, 0.f, 0.f }, { 0.f, -1.f, -1.f, 0.f }
};

__device__ double3 perlin_fade_vec(double3 t) {
    // 6t^5 - 15t^4 + 10t^3 optimized for vector math
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

__device__ double perlin_grad_opt(int hash, double x, double y, double z) {
    // Single lookup of a double3 is faster than 3 lookups of float
    unsigned int h = hash & 15;
    float4 g = GRADS[h];
    return g.x * x + g.y * y + g.z * z;
}

__device__ double perlin(double3 pos, const int8_t* _generator) {
    PerlinNoiseGenerator* generator = (PerlinNoiseGenerator*)_generator;
    double3 rpos = pos + make_double3(generator->origin_x, generator->origin_y, generator->origin_z);
    
    double3 pf = make_double3(floorf(rpos.x), floorf(rpos.y), floorf(rpos.z));
    int3 pi = make_int3((int)pf.x, (int)pf.y, (int)pf.z);
    pi.x &= 255; pi.y &= 255; pi.z &= 255;  // Lattice coordinates
    double3 f = rpos - pf;                     // Fractional part
    double3 v = perlin_fade_vec(f);            // Quintic fade
    
    // Hash coordinates of the 8 cube corners
    const int* p = generator->perm;
    int A  = __ldg(&p[pi.x]) + pi.y;
    int AA = __ldg(&p[A & 255]) + pi.z;
    int AB = __ldg(&p[(A + 1) & 255]) + pi.z;
    int B  = __ldg(&p[(pi.x + 1) & 255]) + pi.y;
    int BA = __ldg(&p[B & 255]) + pi.z;
    int BB = __ldg(&p[(B + 1) & 255]) + pi.z;

    // double grad000 = perlin_grad_opt(__ldg(&p[AA & 255]),       f.x,        f.y,        f.z);
    // double grad100 = perlin_grad_opt(__ldg(&p[(AA + 1) & 255]), f.x,        f.y - 1.f,  f.z);
    // double grad010 = perlin_grad_opt(__ldg(&p[AB & 255]),       f.x,        f.y - 1.f,  f.z);
    // double grad110 = perlin_grad_opt(__ldg(&p[(AB + 1) & 255]), f.x - 1.f,  f.y - 1.f,  f.z);
    // double grad001 = perlin_grad_opt(__ldg(&p[BA & 255]),       f.x - 1.f,  f.y,        f.z - 1.f);
    // double grad101 = perlin_grad_opt(__ldg(&p[(BA + 1) & 255]), f.x,        f.y,        f.z - 1.f);
    // double grad011 = perlin_grad_opt(__ldg(&p[BB & 255]),       f.x - 1.f,  f.y - 1.f,  f.z - 1.f);
    // double grad111 = perlin_grad_opt(__ldg(&p[(BB + 1) & 255]), f.x - 1.f,  f.y - 1.f,  f.z - 1.f);
    double grad000 = perlin_grad_opt(__ldg(&p[AA & 255]),       f.x,       f.y,       f.z);
    double grad100 = perlin_grad_opt(__ldg(&p[BA & 255]),       f.x - 1.0, f.y,       f.z);
    double grad010 = perlin_grad_opt(__ldg(&p[AB & 255]),       f.x,       f.y - 1.0, f.z);
    double grad110 = perlin_grad_opt(__ldg(&p[BB & 255]),       f.x - 1.0, f.y - 1.0, f.z);
    double grad001 = perlin_grad_opt(__ldg(&p[(AA + 1) & 255]), f.x,       f.y,       f.z - 1.0);
    double grad101 = perlin_grad_opt(__ldg(&p[(BA + 1) & 255]), f.x - 1.0, f.y,       f.z - 1.0);
    double grad011 = perlin_grad_opt(__ldg(&p[(AB + 1) & 255]), f.x,       f.y - 1.0, f.z - 1.0);
    double grad111 = perlin_grad_opt(__ldg(&p[(BB + 1) & 255]), f.x - 1.0, f.y - 1.0, f.z - 1.0);
    

    double x0 = lerp(grad000, grad100, v.x);
    double x1 = lerp(grad010, grad110, v.x);
    double y0 = lerp(x0, x1, v.y);
    double x2 = lerp(grad001, grad101, v.x);
    double x3 = lerp(grad011, grad111, v.x);
    double y1 = lerp(x2, x3, v.y);
    
    return lerp(y0, y1, v.z);
}

// Maps a cave value to a scale factor. Matches scale_caves() in utilsf64.rs.
// __device__ float scale_caves(float value) {
//     if (value < -0.75f) {
//         return 0.5f;
//     } else if (value < -0.5f) {
//         return 0.75f;
//     } else if (value < 0.5f) {
//         return 1.0f;
//     } else if (value < 0.75f) {
//         return 2.0f;
//     }
//     return 3.0f;
// }
__device__ float scale_caves(float v) {
    return 0.5f + (v >= -0.75f) * 0.25f + (v >= -0.5f) * 0.25f + (v >= 0.5f) * 1.0f + (v >= 0.75f) * 1.0f;
}

// Maps a tunnel value to a scale factor. Matches scale_tunnels() in utilsf64.rs.
__device__ float scale_tunnels(float v) {
    return 0.75f + (v >= -0.5f) * 0.25f + (v >= 0.0f) * 0.5f + (v >= 0.5f) * 0.5f;
}
// Clamped linear remap from [from_y, to_y] → [from_value, to_value].
// Matches y_clamped_gradient() / clampedMap() in utilsf64.rs.
__device__ float y_clamped_gradient(float y, float from_y, float to_y, float from_value, float to_value) {
    float delta = (y - from_y) / (to_y - from_y);
    if (delta <= 0.0f) {
        return from_value;
    } else if (delta >= 1.0f) {
        return to_value;
    }
    return from_value + delta * (to_value - from_value);
}

// ==========================================
// Trilinear interpolation helpers
// Matches interpolate() / lerp() in mathf64.rs
// ==========================================

__device__ float lerp_(float a, float b, float t) {
    return a + t * (b - a);
}

// Trilinear interpolation of 8 corner values.
// Matches interpolate() in mathf64.rs.
// Corner naming: v{x}{y}{z} where 0 = near, 1 = far.
__device__ float interpolate(
    float v000, float v100, float v010, float v110,
    float v001, float v101, float v011, float v111,
    float fx, float fy, float fz
) {
    float x00 = lerp_(v000, v100, fx);
    float x10 = lerp_(v010, v110, fx);
    float x01 = lerp_(v001, v101, fx);
    float x11 = lerp_(v011, v111, fx);
    float y0 = lerp_(x00, x10, fy);
    float y1 = lerp_(x01, x11, fy);
    return lerp_(y0, y1, fz);
}

// Fractional position within a 4×8×4 grid cell.
// Matches xfract4 / yfract8 / zfract4 in mathf64.rs.
__device__ float xfract4_(unsigned int pos_x) { return (float)(pos_x & 3u) * 0.25f; }
__device__ float yfract8_(unsigned int pos_y) { return (float)(pos_y & 7u) * 0.125f; }
__device__ float zfract4_(unsigned int pos_z) { return (float)(pos_z & 3u) * 0.25f; }

// Fractional position within a 4×16×4 grid cell.
// Matches yfract16 in mathf64.rs.
__device__ float yfract16_(unsigned int pos_y) { return (float)(pos_y & 15u) * 0.0625f; }

// Trilinear interpolation over a 4×8×4 density grid.
// The caller is responsible for fetching the 8 surrounding corner values
// from the coarse grid; this function computes the fractional coordinates
// from `pos` (global voxel position) and interpolates.
// Matches the interpolate484 call pattern emitted by SPMT codegen.
__device__ float interpolate484(
    float v000, float v100, float v010, float v110,
    float v001, float v101, float v011, float v111,
    float xfract, float yfract, float zfract
) {
    return interpolate(
        v000, v100, v010, v110,
        v001, v101, v011, v111,
        xfract, yfract, zfract
    );
}

// Flat 3-D → 1-D index. Matches as_index() in mathf64.rs.
// stride order: z * sy * sx + y * sx + x
__device__ unsigned int grid_index_(unsigned int gx, unsigned int gy, unsigned int gz, unsigned int sx, unsigned int sy) {
    return gz * sy * sx + gy * sx + gx;
}

// ==========================================
// Corner index helpers for 4×8×4 density grid
// Matches cornerx*y*z* in mathf64.rs (y cell size = 8, shift >> 3)
// ==========================================

__device__ uint3 base_grid_(int3 pos) {
    return make_uint3(pos.x >> 2u, pos.y >> 3u, pos.z >> 2u);
}

__device__ unsigned int cornerx0y0z0_8(int3 pos, unsigned int sx, unsigned int sy, unsigned int sz) {
    uint3 g = base_grid_(pos);
    return grid_index_(g.x,     g.y,     g.z,     sx, sy);
}

__device__ unsigned int cornerx4y0z0_8(int3 pos, unsigned int sx, unsigned int sy, unsigned int sz) {
    uint3 g = base_grid_(pos);
    return grid_index_(g.x + 1u, g.y,     g.z,     sx, sy);
}

__device__ unsigned int cornerx0y8z0_8(int3 pos, unsigned int sx, unsigned int sy, unsigned int sz) {
    uint3 g = base_grid_(pos);
    return grid_index_(g.x,     g.y + 1u, g.z,     sx, sy);
}

__device__ unsigned int cornerx4y8z0_8(int3 pos, unsigned int sx, unsigned int sy, unsigned int sz) {
    uint3 g = base_grid_(pos);
    return grid_index_(g.x + 1u, g.y + 1u, g.z,     sx, sy);
}

__device__ unsigned int cornerx0y0z4_8(int3 pos, unsigned int sx, unsigned int sy, unsigned int sz) {
    uint3 g = base_grid_(pos);
    return grid_index_(g.x,     g.y,     g.z + 1u, sx, sy);
}

__device__ unsigned int cornerx4y0z4_8(int3 pos, unsigned int sx, unsigned int sy, unsigned int sz) {
    uint3 g = base_grid_(pos);
    return grid_index_(g.x + 1u, g.y,     g.z + 1u, sx, sy);
}

__device__ unsigned int cornerx0y8z4_8(int3 pos, unsigned int sx, unsigned int sy, unsigned int sz) {
    uint3 g = base_grid_(pos);
    return grid_index_(g.x,     g.y + 1u, g.z + 1u, sx, sy);
}

__device__ unsigned int cornerx4y8z4_8(int3 pos, unsigned int sx, unsigned int sy, unsigned int sz) {
    uint3 g = base_grid_(pos);
    return grid_index_(g.x + 1u, g.y + 1u, g.z + 1u, sx, sy);
}

// ==========================================
// 4×16×4 grid helpers (y cell size = 16)
// ==========================================

__device__ uint3 base_grid_16_(int3 pos) {
    return make_uint3(pos.x >> 2u, pos.y >> 4u, pos.z >> 2u);
}

__device__ unsigned int cornerx0y0z0_16(int3 pos, unsigned int sx, unsigned int sy) {
    uint3 g = base_grid_16_(pos);
    return grid_index_(g.x,     g.y,     g.z,     sx, sy);
}

__device__ unsigned int cornerx4y0z0_16(int3 pos, unsigned int sx, unsigned int sy) {
    uint3 g = base_grid_16_(pos);
    return grid_index_(g.x + 1u, g.y,     g.z,     sx, sy);
}

__device__ unsigned int cornerx0y16z0_16(int3 pos, unsigned int sx, unsigned int sy) {
    uint3 g = base_grid_16_(pos);
    return grid_index_(g.x,     g.y + 1u, g.z,     sx, sy);
}

__device__ unsigned int cornerx4y16z0_16(int3 pos, unsigned int sx, unsigned int sy) {
    uint3 g = base_grid_16_(pos);
    return grid_index_(g.x + 1u, g.y + 1u, g.z,     sx, sy);
}

__device__ unsigned int cornerx0y0z4_16(int3 pos, unsigned int sx, unsigned int sy) {
    uint3 g = base_grid_16_(pos);
    return grid_index_(g.x,     g.y,     g.z + 1u, sx, sy);
}

__device__ unsigned int cornerx4y0z4_16(int3 pos, unsigned int sx, unsigned int sy) {
    uint3 g = base_grid_16_(pos);
    return grid_index_(g.x + 1u, g.y,     g.z + 1u, sx, sy);
}

__device__ unsigned int cornerx0y16z4_16(int3 pos, unsigned int sx, unsigned int sy) {
    uint3 g = base_grid_16_(pos);
    return grid_index_(g.x,     g.y + 1u, g.z + 1u, sx, sy);
}

__device__ unsigned int cornerx4y16z4_16(int3 pos, unsigned int sx, unsigned int sy) {
    uint3 g = base_grid_16_(pos);
    return grid_index_(g.x + 1u, g.y + 1u, g.z + 1u, sx, sy);
}

// ==========================================
// Public fract helpers (called directly as device functions)
// Aliases for the underscore-suffixed private helpers above.
// Matches xfract4 / yfract8 / zfract4 in mathf64.rs.
// ==========================================

__device__ float xfract4(int3 pos) { return xfract4_(pos.x); }
__device__ float yfract8(int3 pos) { return yfract8_(pos.y); }
__device__ float yfract16(int3 pos) { return yfract16_(pos.y); }
__device__ float zfract4(int3 pos) { return zfract4_(pos.z); }

// ==========================================
// Flat 2-D index helpers
// Matches flat_y_zero_index / flat_z_zero_index / biome_column_index in mathf64.rs
// ==========================================

// 3-D → 2-D index with y dimension flattened (y is ignored).
// stride order: z * size_x + x
__device__ unsigned int flat_y_zero_index(int3 pos, unsigned int size_x, unsigned int size_y) {
    return pos.z * size_x + pos.x;
}

// 3-D → 2-D index with z dimension flattened (z is ignored).
// stride order: y * size_x + x
__device__ unsigned int flat_z_zero_index(int3 pos, unsigned int size_x, unsigned int size_y) {
    return pos.y * size_x + pos.x;
}

// Biome column index: shifts x and z right by 2, clears y, then flat_y_zero_index with size_x=4.
// Matches biome_column_index() in mathf64.rs.
__device__ unsigned int biome_column_index(int3 pos) {
    return flat_y_zero_index(make_int3(pos.x >> 2u, 0u, pos.z >> 2u), 5u, 5u);
}

// __device__ float old_blended_noise(
//     double3 rpos3,
//     double xz_scale,
//     double y_scale,
//     double xz_factor,
//     double y_factor,
//     double smear_scale_multiplier
// ) {
//     return 0.0f;
// }

__device__ __forceinline__ int perlin_map_(const PerlinNoiseGenerator* pns, int input) {
    return pns->perm[input & 255] & 255;
}

__device__ __forceinline__ double sample_perlin_section_scaled_(
    const PerlinNoiseGenerator* pns,
    int section_x,
    int section_y,
    int section_z,
    double x,
    double y,
    double z,
    double fade_y
) {
    int i = perlin_map_(pns, section_x);
    int j = perlin_map_(pns, section_x + 1);
    int k = perlin_map_(pns, i + section_y);
    int l = perlin_map_(pns, i + section_y + 1);
    int m = perlin_map_(pns, j + section_y);
    int n = perlin_map_(pns, j + section_y + 1);

    double d = perlin_grad_opt(perlin_map_(pns, k + section_z), x, y, z);
    double e = perlin_grad_opt(perlin_map_(pns, m + section_z), x - 1.0, y, z);
    double f = perlin_grad_opt(perlin_map_(pns, l + section_z), x, y - 1.0, z);
    double g = perlin_grad_opt(perlin_map_(pns, n + section_z), x - 1.0, y - 1.0, z);
    double h = perlin_grad_opt(perlin_map_(pns, k + section_z + 1), x, y, z - 1.0);
    double o = perlin_grad_opt(perlin_map_(pns, m + section_z + 1), x - 1.0, y, z - 1.0);
    double p = perlin_grad_opt(perlin_map_(pns, l + section_z + 1), x, y - 1.0, z - 1.0);
    double q = perlin_grad_opt(perlin_map_(pns, n + section_z + 1), x - 1.0, y - 1.0, z - 1.0);

    double r = perlin_fade_(x);
    double s = perlin_fade_(fade_y);
    double t = perlin_fade_(z);

    double x00 = lerp(d, e, r);
    double x10 = lerp(f, g, r);
    double y0 = lerp(x00, x10, s);
    double x01 = lerp(h, o, r);
    double x11 = lerp(p, q, r);
    double y1 = lerp(x01, x11, s);
    return lerp(y0, y1, t);
}

__device__ __forceinline__ double sample_perlin_scaled_(
    const PerlinNoiseGenerator* pns,
    double x,
    double y,
    double z,
    double y_scale,
    double y_max
) {
    x += pns->origin_x;
    y += pns->origin_y;
    z += pns->origin_z;

    int i = (int)floorf(x);
    int j = (int)floorf(y);
    int k = (int)floorf(z);

    double g = x - (double)i;
    double h = y - (double)j;
    double l = z - (double)k;

    double n = 0.0f;
    if (y_scale != 0.0f) {
        double m = (y_max >= 0.0f && y_max < h) ? y_max : h;
        n = floorf(m / y_scale + 1.0e-7f) * y_scale;
    }

    return sample_perlin_section_scaled_(pns, i, j, k, g, h - n, l, h);
}

const double WRAP = 33554432.0f; // 3.3554432e7
__device__ __forceinline__ double maintain_precision_(double value) {
    return value - floorf(value / WRAP + 0.5f) * WRAP;
}

__device__ __forceinline__ double clamped_lerp_(double start, double end, double delta) {
    double d = clamp(delta, 0.0f, 1.0f);
    return start + d * (end - start);
}

__device__ double base3d_noise(
    double3 p,
    const int8_t* sampler_blob,
    double smear_scale_multiplier,
    double xz_factor,
    double scaled_xz_scale,
    double y_factor,
    double scaled_y_scale
) {
    if (sampler_blob == nullptr) {
        return 0.0f;
    }

    const InterpolatedNoiseSamplerGPU* sampler = (const InterpolatedNoiseSamplerGPU*)sampler_blob;
    const double BASE_3D_XZ_SCALE = 684.412f;

    double d = p.x * scaled_xz_scale * BASE_3D_XZ_SCALE;
    double e = p.y * scaled_y_scale * BASE_3D_XZ_SCALE;
    double f = p.z * scaled_xz_scale * BASE_3D_XZ_SCALE;

    double g = d / xz_factor;
    double h = e / y_factor;
    double iz = f / xz_factor;
    double j = scaled_y_scale * BASE_3D_XZ_SCALE * smear_scale_multiplier;
    double k = j / y_factor;

    double l = 0.0f;
    double m = 0.0f;
    double n = 0.0f;
    double o = 1.0f;
    double inv_o = 1.0f;

    for (int octave = 7; octave >= 0; --octave) {
        n += sample_perlin_scaled_(
            &sampler->interpolation[octave],
            maintain_precision_(g * o),
            maintain_precision_(h * o),
            maintain_precision_(iz * o),
            k * o,
            h * o
        ) * inv_o;
        o *= 0.5f;
        inv_o *= 2.0f;
    }

    double q = (n * 0.1f + 1.0f) * 0.5f;
    bool bl2 = q >= 1.0f;
    bool bl3 = q <= 0.0f;
    o = 1.0f;

    for (int octave = 15; octave >= 0; --octave) {
        double s = maintain_precision_(d * o);
        double t = maintain_precision_(e * o);
        double u = maintain_precision_(f * o);
        double v = j * o;

        if (!bl2) {
            l += sample_perlin_scaled_(&sampler->lower[octave], s, t, u, v, e * o) / o;
        }
        if (!bl3) {
            m += sample_perlin_scaled_(&sampler->upper[octave], s, t, u, v, e * o) / o;
        }

        o *= 0.5f;
    }

    return clamped_lerp_(l * 0.001953125f, m * 0.001953125f, q) * 0.0078125f;
}



/*
/// Port of `InterpolatedNoiseSampler`
pub struct InterpolatedNoiseSampler {
    pub lower: [PerlinNoiseSampler; 16], // rangeClosed(-15, 0)
    pub upper: [PerlinNoiseSampler; 16], // rangeClosed(-15, 0)
    pub interpolation: [PerlinNoiseSampler; 8], // rangeClosed(-7, 0)
                                         // scaled_xz_scale: f64,
                                         // scaled_y_scale: f64,
                                         // xz_factor: f64,
                                         // y_factor: f64,
                                         // smear_scale_multiplier: f64,
}

impl InterpolatedNoiseSampler {
    /// Equivalent to `new InterpolatedNoiseSampler(random, xzScale, yScale, xzFactor, yFactor, smearScaleMultiplier)`.
    pub fn new_boxed(
        rng: &mut Xoroshiro128PlusPlusRandom,
        // xz_scale: f64,
        // y_scale: f64,
        // xz_factor: f64,
        // y_factor: f64,
        // smear_scale_multiplier: f64,
    ) -> Box<InterpolatedNoiseSampler> {
        // let lower = OctavePerlinNoiseSampler::create_legacy(rng, 16); // rangeClosed(-15, 0)
        // let upper = OctavePerlinNoiseSampler::create_legacy(rng, 16);
        // let interpolation = OctavePerlinNoiseSampler::create_legacy(rng, 8); // rangeClosed(-7, 0)

        // allocate empty uninitialized data for the samplers, then fill it in-place to avoid stack overflow from large arrays
        let mut base: Box<InterpolatedNoiseSampler> = unsafe { Box::new_uninit().assume_init() };
        // InterpolatedNoiseSampler {
        //     lower,
        //     upper,
        //     interpolation,
        //     scaled_xz_scale: 684.412 * xz_scale,
        //     scaled_y_scale: 684.412 * y_scale,
        //     xz_factor,
        //     y_factor,
        //     smear_scale_multiplier,
        // }
        create_legacy(rng, &mut base.lower);
        create_legacy(rng, &mut base.upper);
        create_legacy(rng, &mut base.interpolation);
        // base.scaled_xz_scale = 684.412 * xz_scale;
        // base.scaled_y_scale = 684.412 * y_scale;
        // base.xz_factor = xz_factor;
        // base.y_factor = y_factor;
        // base.smear_scale_multiplier = smear_scale_multiplier;

        return base;
    }

    /// Equivalent to `new InterpolatedNoiseSampler(new Xoroshiro128PlusPlusRandom(0L), ...)`.
    pub fn create_base3d(seed: i64) -> Box<InterpolatedNoiseSampler> {
        let mut rng = Xoroshiro128PlusPlusRandom::from_seed(&create_xoroshiro_seed(seed));
        let mut random_splitter = rng.next_splitter();

        let xoroshiro_seed = create_xoroshiro_seed_str("minecraft:terrain");
        let mut rng = random_splitter.split(xoroshiro_seed.seed_lo, xoroshiro_seed.seed_hi);
        Self::new_boxed(&mut rng)
    }
}

pub fn make_base3d_perm_table(seed: i64) -> Box<InterpolatedNoiseSampler> {
    InterpolatedNoiseSampler::create_base3d(seed)
}

const BASE_3D_XZ_SCALE: f64 = 684.412;

#[inline(always)]
pub fn base3d_noise(
    p: Vec3,
    sampler: &InterpolatedNoiseSampler,
    smear_scale_multiplier: f64,
    xz_factor: f64,
    scaled_xz_scale: f64,
    y_factor: f64,
    scaled_y_scale: f64,
) -> f64 {
    let x = p.x;
    let y = p.y;
    let z = p.z;
    let d = x * scaled_xz_scale * BASE_3D_XZ_SCALE;
    let e = y * scaled_y_scale * BASE_3D_XZ_SCALE;
    let f = z * scaled_xz_scale * BASE_3D_XZ_SCALE;

    let g = d / xz_factor;
    let h = e / y_factor;
    let iz = f / xz_factor;
    let j = scaled_y_scale * BASE_3D_XZ_SCALE * smear_scale_multiplier;
    let k = j / y_factor;

    let mut l = 0.0_f64;
    let mut m = 0.0_f64;
    let mut n = 0.0_f64;
    let mut o = 1.0_f64;
    let mut inv_o = 1.0_f64;

    for p in (0..8).rev() {
        n += sample_perlin_scaled(
            &sampler.interpolation[p],
            maintain_precision(g * o),
            maintain_precision(h * o),
            maintain_precision(iz * o),
            k * o,
            h * o,
        ) * inv_o;
        o *= 0.5;
        inv_o *= 2.0;
    }

    let q = (n * 0.1 + 1.0) * 0.5;
    let bl2 = q >= 1.0;
    let bl3 = q <= 0.0;
    o = 1.0;

    for r in (0..16).rev() {
        let s = maintain_precision(d * o);
        let t = maintain_precision(e * o);
        let u = maintain_precision(f * o);
        let v = j * o;

        if !bl2 {
            l += sample_perlin_scaled(&sampler.lower[r], s, t, u, v, e * o) / o;
        }
        if !bl3 {
            m += sample_perlin_scaled(&sampler.upper[r], s, t, u, v, e * o) / o;
        }

        o *= 0.5;
    }

    clamped_lerp(l * 0.001953125, m * 0.001953125, q) * 0.0078125
}

/// Port of `OctavePerlinNoiseSampler.maintainPrecision`.
#[inline(always)]
fn maintain_precision(value: f64) -> f64 {
    value - (value / 3.3554432e7 + 0.5).floor() * 3.3554432e7
}

/// Port of `MathHelper.clampedLerp(start, end, delta)`.
#[inline(always)]
fn clamped_lerp(start: f64, end: f64, delta: f64) -> f64 {
    let d = delta.clamp(0.0, 1.0);
    start + d * (end - start)
}

*/

/*
#[inline(always)]
pub fn advanced_hermite<const N: usize>(
    spline_locations: [f32; N],
    spline_values: [f32; N],
    spline_derivatives: [f32; N],
    coordinate: f32,
    index: i32,
) -> f32 {
    // let h_minus_g = second_value - first_value;
    // hermite(
    //     t,
    //     first_value,
    //     second_value,
    //     first_derivative,
    //     second_derivative,
    //     h_minus_g,
    // )
    let index: usize = index as usize;
    if index == 0 || index >= N {
        // extrapolate
        let value = spline_values[index.min(N - 1)];
        let derivative = spline_derivatives[index.min(N - 1)];
        let location = spline_locations[index.min(N - 1)];
        return value + derivative * (coordinate - location);
    }
    let index_minus_1 = index - 1;
    let h_minus_g = spline_values[index] - spline_values[index_minus_1];
    let t = (coordinate - spline_locations[index_minus_1])
        / (spline_locations[index] - spline_locations[index_minus_1]);
    let value_minus_1 = spline_values[index_minus_1];
    let value = spline_values[index];
    let derivative_minus_1 = spline_derivatives[index_minus_1];
    let derivative = spline_derivatives[index];
    hermite(
        t,
        value_minus_1,
        value,
        derivative_minus_1,
        derivative,
        h_minus_g,
    )
}
*/

template <std::size_t N>
__device__ __forceinline__ double advanced_hermite(
    // const double* spline_locations,
    // const double* spline_values,
    // const double* spline_derivatives,
    const float (&spline_locations)[N],
    const float (&spline_values)[N],
    const float (&spline_derivatives)[N],
    float coordinate,
    int index
) {
    if (index == 0 || index >= N) {
        // extrapolate
        int clamped_index = min(index, (int)N - 1);
        float value = spline_values[clamped_index];
        float derivative = spline_derivatives[clamped_index];
        float location = spline_locations[clamped_index];
        return value + derivative * (coordinate - location);
    }
    int index_minus_1 = index - 1;
    float h_minus_g = spline_values[index] - spline_values[index_minus_1];
    float t = (coordinate - spline_locations[index_minus_1])
        / (spline_locations[index] - spline_locations[index_minus_1]);
    float value_minus_1 = spline_values[index_minus_1];
    float value = spline_values[index];
    float derivative_minus_1 = spline_derivatives[index_minus_1];
    float derivative = spline_derivatives[index];
    return hermite(
        t,
        value_minus_1,
        value,
        derivative_minus_1,
        derivative,
        h_minus_g
    );
}

/*

pub fn binary_search<const N: usize>(arr: [f32; N], target: f32) -> i32 {
    // actually just use linear search since the arrays are small (length 2-11)
    for i in 0..N as usize {
        if arr[i] > target {
            return i as i32;
        }
    }
    return N as i32;
}
*/

// not quite binary search yet, just a linear search for small arrays
// it works better for small arrays than a full binary search
// __device__ __forceinline__ int binary_search(const float* arr, float target, int N) {
//     for (int i = 0; i < N; i++) {
//         if (arr[i] > target) {
//             return i;
//         }
//     }
//     return N;
// }
template <std::size_t N>
__device__ __forceinline__ int binary_search(const float (&arr)[N], float target) {
    for (std::size_t i = 0; i < N; ++i) {
        if (arr[i] > target) {
            return static_cast<int>(i);
        }
    }
    return static_cast<int>(N);
}



// Equivalent to `create_legacy` in utilsf64.rs
template <std::size_t N>
static void create_legacy(PerlinNoiseGenerator (&samplers)[N], Xoroshiro128PlusPlusRandom* rng) {
    if (N == 0) return;

    // Creation order matches Java/Rust: index N-1 first, then N-2 down to 0
    create_perlin_noise_sampler(&samplers[N - 1], rng);
    
    for (int kx = static_cast<int>(N) - 2; kx >= 0; kx--) {
        create_perlin_noise_sampler(&samplers[kx], rng);
    }
}

void create_perlin_noise_sampler(PerlinNoiseGenerator* out, Xoroshiro128PlusPlusRandom* rng) {
    if (!out || !rng) return;
    
    out->origin_x = static_cast<float>(rng->next_double() * 256.0);
    out->origin_y = static_cast<float>(rng->next_double() * 256.0);
    out->origin_z = static_cast<float>(rng->next_double() * 256.0);

    // Initialize default permutation table
    for (int i = 0; i < 256; i++) {
        out->perm[i] = i;
    }
    
    // shuffle
    for (int i = 0; i < 256; i++) {
        int j = i + rng->next_int_bound(256 - i);
        int tmp = out->perm[i];
        out->perm[i] = out->perm[j];
        out->perm[j] = tmp;
    }
}

// Generate a 256-element permutation table matching Minecraft's PerlinNoiseSampler.
// ident_lo/hi and subident_lo/hi are precomputed via MD5(string) at codegen time.
static void make_perm_table(
    PerlinNoiseGenerator* out, int64_t world_seed,
    int64_t ident_lo,    int64_t ident_hi,
    int64_t  subident_index,
    int64_t subident_lo, int64_t subident_hi
) {
    PerlinNoiseGenerator* table = out;
    Xoroshiro128PlusPlusRandom rng_base(create_xoroshiro_seed(world_seed));
    auto rng_parent = rng_base.next_splitter();
    auto rng1 = rng_parent.split(ident_lo, ident_hi);
    for (int i = 0; i < subident_index; i++) {
        rng1.next_splitter();
    }

    auto rng1_split = rng1.next_splitter();
    auto rng2 = rng1_split.split(subident_lo, subident_hi);

    create_perlin_noise_sampler(table, &rng2);
}


/// Equivalent to `InterpolatedNoiseSampler::create_base3d` and `make_base3d_perm_table`
void create_base3d(InterpolatedNoiseSamplerGPU* out, int64_t seed) {
    if (!out) return;

    // 1. Create base RNG from the numerical seed
    Xoroshiro128PlusPlusRandom base_rng = Xoroshiro128PlusPlusRandom(create_xoroshiro_seed(seed));
    
    // 2. Advance the base RNG and get a splitter
    XoroshiroRandomSplitter random_splitter = base_rng.next_splitter();

    // 3. Hash the "minecraft:terrain" string to get the sub-seed
    XoroshiroSeed terrain_seed(2226279196109926164LL, -2108001439914377933LL);
    
    // 4. Split the RNG using the terrain sub-seed
    Xoroshiro128PlusPlusRandom terrain_rng = random_splitter.split(terrain_seed.seed_lo, terrain_seed.seed_hi);

    // 5. Fill the generator arrays in-place
    create_legacy<16>(out->lower, &terrain_rng);
    create_legacy<16>(out->upper, &terrain_rng);
    create_legacy<8>(out->interpolation, &terrain_rng);
}

