# JNI API of `tungsten-mc-compile-jni`

The mod drives the compiler through `libtungsten_mc_compile_jni.so` (`cargo build --release -p tungsten-mc-compile-jni`, output in `target/release/`). The library exports exactly the six symbols below, for the Java class `tungsten.compiler.NativeCompiler`.

## Java declarations

```java
package tungsten.compiler;

public final class NativeCompiler {
    /** A new compiler session. Never 0. */
    static native long create();
    /** Feeds one resource; see "Resource ids". A later call with the same id replaces the earlier one. */
    static native void loadResource(long handle, String id, byte[] bytes);
    /** The noise settings to compile, e.g. "minecraft:overworld", "minecraft:amplified". */
    static native void setRoot(long handle, String id);
    /** Compiles; returns the result document (JSON, see below). Runs on its own 16 MB-stack thread. */
    static native String compile(long handle);
    /** The compiler crate version, e.g. "0.211.1". */
    static native String version();
    /** Frees the session. 0 is ignored. The handle must not be used afterwards. */
    static native void dispose(long handle);
}

public class TungstenCompilerException extends RuntimeException {
    public TungstenCompilerException(String message) { super(message); }
}
```

`TungstenCompilerException` must have a public `(String)` constructor: the library throws it with `ThrowNew`.

## Handles and threading

- One handle per preparation (SPEC §9.1): `create`, any number of `loadResource`, `setRoot`, `compile`, then `dispose` in a `finally`/try-with-resources. No native state outlives `dispose`.
- A handle is not thread-safe. Use it from one thread at a time. Different handles are independent.
- `compile` can be called again after more `loadResource`/`setRoot` calls. Each call compiles the current state.

## Errors and exceptions

- **The worldgen is unsupported** (a user problem, e.g. an unknown density function type or a missing reference): `compile` returns `"status": "unsupported"`. No exception is thrown.
- **The compiler failed internally without panicking**: `compile` returns `"status": "internal"`.
- **A panic anywhere in the library, a failed JNI call, or handle 0**: the export throws `TungstenCompilerException` with the panic message and returns 0/`null`. Panics never unwind into the JVM. The workspace release profile uses `panic = "unwind"` for this reason.

## Resource ids

`id` is the resource path as Minecraft's resource manager sees it: `<namespace>:worldgen/<registry>/<path>.json`, e.g. `minecraft:worldgen/density_function/overworld/continents.json`. The compiler uses `worldgen/noise_settings/`, `worldgen/density_function/` and `worldgen/noise/`, and ignores every other id. Files must be UTF-8. If any fed file of those three kinds isn't, `compile` reports it as unsupported.

Ids in the result (`reachable`, error `id`) are registry ids: `minecraft:overworld/continents`, `minecraft:continentalness`.

The compiler bundles no worldgen: it sees only what was fed.

## Generated code (fixed)

The shape of the generated Java doesn't depend on the worldgen that was compiled, so the result document doesn't report it. The source of truth is `tungsten-mc-compile/src/contract.rs`. Every compile checks its output against that file and reports any mismatch as `"status": "internal"`.

- The top-level class is `tungsten.generated.GeneratedWorldgen`. Its nested classes are `HostConsts`, `AutoConsts` and `Orchestration` (binary names `GeneratedWorldgen$HostConsts`, …).
- `AutoConsts` is created with `public static AutoConsts create(long seed)`. It is immutable and can be shared between threads.
- `Orchestration` is constructed as `new Orchestration(AutoConsts)`, one instance per thread. It has one method per route, always all seven:

  | Route | Method | Returns | Length |
  |---|---|---|---|
  | `final_density_with_beardify` | `orchestrate_final_density_with_beardify` | `double[]` | 16 × height × 16 |
  | `final_density` | `orchestrate_final_density` | `double[]` | 16 × height × 16 |
  | `vein_gap` | `orchestrate_vein_gap` | `double[]` | 16 × height × 16 |
  | `vein_ridged` | `orchestrate_vein_ridged` | `double[]` | 16 × height × 16 |
  | `vein_toggle` | `orchestrate_vein_toggle` | `double[]` | 16 × height × 16 |
  | `preliminary_surface_level` | `orchestrate_preliminary_surface_level` | `double[]` | 128 × 1 × 128 |
  | `aquifer` | `orchestrate_aquifer` | `byte[]` | 16 × height × 16 |

  Each method has the signature `public T[] orchestrate_<route>(double x, double y, double z, HostConsts host_consts)`. The returned array is an internal buffer that the next call overwrites. A route that the noise settings leave constant still gets its method. `height` is `layout.height` from the result.
- `HostConsts` has exactly these public mutable fields: `BlockBox base_box`, `Jigsaw[] jigsaw_array`, `Piece[] piece_array`, `int aquifer_max_y`, `XoroshiroRandomSplitter splitter`, `EstimatedSurfaceHeight estimate_surface_height`.

A change to any of this bumps the schema version.

## Result document (schema 2)

Every document has `schema` (currently `2`; the Java side must refuse any other value), `compiler_version` and `status`.

### `"status": "success"`

| Field | Meaning |
|---|---|
| `units[]` | `{name, source}`: compilation units to compile together. `name` is the fully qualified top-level class name. Currently always one unit, `tungsten.generated.GeneratedWorldgen`. |
| `reachable[]` | `{kind, id}`, `kind` ∈ `density_function`, `noise`: every density function and noise reached by reference from the root (SPEC §7.5), sorted by kind and id. Inline definitions inside a file have no id and aren't listed. |
| `layout` | `{min_y, height}` of the compiled noise settings. `height` is the y size of the routes (see "Generated code"). |
| `warnings[]` | Non-fatal backend diagnostics (e.g. methods over HotSpot's huge-method limit). For the log. |

Example (vanilla overworld, sources and lists shortened):

```json
{
  "schema": 2,
  "compiler_version": "0.211.1",
  "status": "success",
  "units": [{"name": "tungsten.generated.GeneratedWorldgen", "source": "// Generated by tungsten-wg ...\npackage tungsten.generated;\n..."}],
  "reachable": [
    {"kind": "density_function", "id": "minecraft:overworld/base_3d_noise"},
    {"kind": "density_function", "id": "minecraft:overworld/caves/entrances"},
    {"kind": "noise", "id": "minecraft:continentalness"}
  ],
  "layout": {"min_y": -64, "height": 384},
  "warnings": ["java: static initializer of GeneratedWorldgen estimated at 10608 bytes exceeds HotSpot HugeMethodLimit (8000); ..."]
}
```

### `"status": "unsupported"`

```json
{"schema": 2, "compiler_version": "0.211.1", "status": "unsupported",
 "kind": "density_function", "id": "minecraft:overworld/continents",
 "message": "unknown density function type `minecraft:nope`"}
```

`kind` ∈ `noise_settings`, `density_function`, `noise`. `id` names the file at fault: the file that contains the bad definition, or the missing id for a reference that wasn't provided. `message` is for the log and the screen.

### `"status": "internal"`

```json
{"schema": 2, "compiler_version": "0.211.1", "status": "internal", "message": "..."}
```

A compiler bug the compiler detected itself (e.g. a backend error). It is worded as a bug report, like a `TungstenCompilerException`.
