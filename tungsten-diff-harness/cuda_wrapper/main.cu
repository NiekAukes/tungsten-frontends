// CUDA half of the differential harness's Phase 1: a tiny extern "C" wrapper
// that instantiates the generated CudaPipeline_final_density, runs it, and
// flattens its std::vector<float> output into a raw buffer the harness can
// read via `libloading`.
//
// Compiled by the harness (see src/cuda_build.rs) with:
//   nvcc --shared -Xcompiler -fPIC -o libcuda_pipeline.so cuda_wrapper.cu \
//        -I <generated_dir>
//
// `density_function.cu` and `orchestration.cu` are the files emitted by
// tungsten-mc-compile (see tungsten-mc-compile/src/main.rs). They are
// #included (not compiled as separate translation units) so the single nvcc
// invocation above is enough. `orchestration.cu` / `density_function.cu` are
// expected to themselves `#include "helpers.cu"` for any shared CUDA device
// helpers; that file does not exist yet, so this wrapper will fail to build
// until it is provided alongside the generated sources.
#include <cstdint>
#include <cstring>
#include <vector>

#include <cuda_runtime.h>

#include "density_function.cu"
#include "orchestration.cu"

extern "C" {

// FFI entry point loaded by the diff harness via `libloading`.
//
// Writes `out_len` floats to `out_ptr`. `out_len` must equal DIMS
// (16 * 384 * 16 = 98304); mismatches return a non-zero code and write
// nothing.
int32_t run_cuda_pipeline(
    float origin_x,
    float origin_y,
    float origin_z,
    uint64_t world_seed,
    float *out_ptr,
    size_t out_len) {
  constexpr size_t DIMS = 16 * 384 * 16;
  if (out_len != DIMS) {
    return 1;
  }

  float3 origin = make_float3(origin_x, origin_y, origin_z);

  CudaPipeline_final_density pipeline(world_seed);
  std::vector<float> result = pipeline.run(origin);

  if (result.size() != DIMS) {
    return 2;
  }

  std::memcpy(out_ptr, result.data(), DIMS * sizeof(float));
  return 0;
}

} // extern "C"
