# Phase 4: Config and world record

Status: not started
SPEC: §5, §6 (all), §9 (dedicated-server messages)

## Goal

The config file, the per-world record and the profile-resolution logic exist as plain, unit-tested code, with the YACL screen on the client. Nothing calls them from a world load yet (phase 5 does).

## Scope

1. **`tungsten.config.TungstenConfig`** via YACL's `ConfigClassHandler` → `config/tungsten.json`, keys exactly as SPEC §5. Enums `Profile { OFF, EXACT, OPTIMIZED }` (OPTIMIZED dev-only, see 4) and `Backend { JAVA }`.
   - Logs a reminder at startup while `acceptProfileChange` is true (SPEC §5 proposal).
2. **Client screen** (`tungsten.client.config`): YACL screen with a short description per option, a "Server" group for `acceptProfileChange`, and the Exact tooltip that mentions blending seams (SPEC §8.4). Mod Menu entry point (`modmenu` entrypoint, optional dependency).
3. **`tungsten.record.WorldRecord`**: the `tungsten.json` format (SPEC §6.1). `read(Path worldDir) → Optional<WorldRecord>` (malformed → log + empty), `write(Path worldDir)` (atomic: temp + move), `settingsHash` = SHA-256 over `(profile, backend, compilerVersion)`.
4. **`tungsten.record.ProfileResolver`**: pure function
   `resolve(Optional<WorldRecord>, Availability, TungstenConfig, BuildInfo) → Resolution { effectiveProfile, warning: none | seams(canUseStored), recordToWrite }`
   implementing §6.2 and §6.3. `BuildInfo` says which profiles/backends/compiler versions this build can reproduce.
   - **Dev-only fake Optimized** (SPEC §6.4): `OPTIMIZED` is selectable only when `-Dtungsten.dev.fakeOptimized=true`. It behaves like EXACT at generation time, but counts as Optimized for resolution, so the seams path can be tested.
5. **Dedicated-server decision**: `ServerDecision.from(Resolution, config)` → `proceed` | `refuse(message)`, with the messages from §6.4 and §9 that name the config key and the record field.

## Out of scope

Hooks, screens for seams/unsupported (phase 8), Availability toast (phase 8).

## Acceptance

- [ ] `ProfileResolverTest`: table-driven test with one row per case in §6.2/§6.3 — at least: record OFF; no record; record EXACT same/other compiler version; unavailable + record EXACT; unavailable + record OPTIMIZED; record OPTIMIZED same hash (no warning); record OPTIMIZED other compiler version (warning, `canUseStored=false`); config OPTIMIZED + record EXACT (warning).
- [ ] `WorldRecordTest`: write/read round-trip; malformed JSON → empty + log; unknown `format` → empty + log; missing file → empty.
- [ ] `ServerDecisionTest`: seams + `acceptProfileChange=false` refuses with a message containing `acceptProfileChange`; with `true` proceeds.
- [ ] `runClient`: Mod Menu → Tungsten opens the YACL screen; changing a value writes `run/config/tungsten.json`.
- [ ] `runServer` starts with YACL present and creates the default config.

## Review checklist

- `ProfileResolver` has no I/O and no Minecraft imports; the test table reads like the SPEC table.
- The record's JSON field names match SPEC §6.1 exactly.
- YACL classes are only referenced from the client source set or behind YACL's config API (which is common).

## Notes for later phases

_(fill in)_
