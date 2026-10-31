# Phase 8: Single-player flow and screens

Status: not started
SPEC: §4 (toast), §6.4 (seams dialog), §7.1 (single-player), §7.2 (stage text), §9 (unsupported screen)

## Goal

In single-player, opening or creating a world shows the Tungsten preparation screen, runs the same `TungstenPreparation` as the server in the background, handles every outcome with the right screen, and only then starts the integrated server. Cancelling never leaves the world locked.

## Scope

1. **Mixin `StartIntegratedServerMixin`**: `@Inject(HEAD, cancellable)` on `MinecraftClient.startIntegratedServer(Session, ResourcePackManager, SaveLoader, boolean)`. Re-entrancy flag; on the first call: cancel, store args, open `TungstenPrepareScreen`.
2. **`TungstenPrepareScreen`**: stage text + spinner, no cancel (§7.2). Runs preparation on a background thread; posts results to the render thread.
3. **Outcomes**:
   - `Active` / `Inactive` / OFF → set flag, call `startIntegratedServer` again with the original arguments, register the instance with `TungstenRuntime` once the server exists.
   - `Unsupported` → `TungstenUnsupportedScreen` (reason, copyable details, "Back to title", "Open with Tungsten off" which writes `profile: OFF` and continues).
   - Seams warning → `TungstenSeamsScreen` (Continue / Use stored profile (only if reproducible) / Cancel). The preparation thread waits on the user's choice.
4. **Cleanup on cancel/back**: close the `Session` and the `SaveLoader` exactly like vanilla's cancel paths in `IntegratedServerLoader` (find and cite them). Then show the title or world-list screen.
5. **Availability toast**: once per session on the title screen when unavailable; the reason is also shown in the YACL screen header.
6. Move the shared parts of the dedicated hook (input building) into one place, so both hooks build `PreparationInput` the same way.

## Out of scope

Real Optimized profile. Edit-world "reset Tungsten" button.

## Acceptance

Manual checklist on `runClient`, recorded in the PR (screenshots of each screen):

- [ ] Create new world (default): prepare screen shows the four stages, world opens, `saves/<w>/tungsten.json` = `EXACT`.
- [ ] Load that world again: same, one compile.
- [ ] World with JJThunder datapack added at creation: compiles and opens.
- [ ] Inline custom preset (or a datapack with an unknown density function type): unsupported screen. "Back to title" → title screen; **immediately open the same world again works** (no `session.lock` error). "Open with Tungsten off" → world opens vanilla; next load shows no screen.
- [ ] With `-Dtungsten.dev.fakeOptimized=true` and the record set to `OPTIMIZED` from another compiler version: seams screen without "Use stored profile". Cancel → world list; reopening works. Continue → record rewritten.
- [ ] `enabled=false`: no prepare screen, world opens.
- [ ] Simulate unsupported platform (`-Dos.arch=aarch64` or a dev flag): toast on the title screen once; worlds open vanilla.
- [ ] Join a remote server (dedicated from `runServer`): no prepare screen, no native extraction (`.tungsten/natives` not created on a fresh game dir).
- [ ] Open and close 5 worlds in one session: `TungstenRuntime` instance count returns to 0 each time (log line); heap after GC doesn't grow steadily (rough check with `jcmd GC.class_histogram` for generated classes).

## Review checklist

- The mixin only stores args and opens the screen.
- The background thread never touches Minecraft UI state directly; all screen changes go through `client.execute`.
- Every path out of the flow either starts the server or closes `Session` + `SaveLoader` (draw the paths in the PR description).

## Notes for later phases

_(fill in)_
