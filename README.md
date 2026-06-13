# Cellular Automata

**A Rust library for generic 2D cellular automaton simulation** — supports arbitrary cell types, custom rules, and both Moore (8-neighbor) and Von Neumann (4-neighbor) neighborhoods. Ships with Conway's Game of Life as a built-in rule.

## Why It Matters

Cellular automata (CA) are discrete computational models where simple local rules produce complex global behavior. Each cell updates based on its neighbors, creating emergent patterns. This library generalizes beyond Game of Life to support *any* rule function and *any* cell type.

Cellular automata have practical applications in:
- **Simulation** — traffic flow (Nagel-Schreckenberg), forest fire spread, epidemic modeling
- **Image processing** — cellular smoothing, erosion/dilation morphology
- **Procedural generation** — cave generation (using CA for dungeon walls), terrain textures
- **Physics** — lattice gas automata for fluid dynamics, Ising model for magnetism

The library's generic design means you can simulate boolean CAs (Game of Life, Wireworld), integer CAs (predator-prey, reaction-diffusion), or any custom state space.

## How It Works

**Generic design**: `Automaton<T>` is parameterized over cell type `T: Clone + Default + PartialEq`. The rule is a function pointer `fn(&[Option<T>], &T) -> T` — it receives the neighbor values (as `Option`, with `None` for out-of-bounds) and the current cell value, returning the next cell value.

**Neighborhoods**:
- **Moore**: 8 surrounding cells (up/down/left/right + 4 diagonals). Standard for Game of Life.
- **Von Neumann**: 4 cardinal neighbors (up/down/left/right only). Used in lattice gas models.

**Simulation**: `step()` creates a copy of the grid, then applies the rule to every cell using the old grid's neighbor values. This synchronous update is essential — cells must all see the same state. `step_n(n)` runs n generations.

**Built-in Conway rule**: `conway_rule` implements the standard Game of Life: live cells with 2–3 neighbors survive, dead cells with exactly 3 neighbors are born.

## Quick Start

```rust
use cellular_automata::{Automaton, Neighborhood, conway_rule};

let mut auto = Automaton::new(10, 10, conway_rule, Neighborhood::Moore);

// Set up a blinker (vertical line → oscillates to horizontal)
auto.set(5, 4, true);
auto.set(5, 5, true);
auto.set(5, 6, true);

// Step one generation
auto.step();

// The blinker should now be horizontal
assert_eq!(*auto.get(4, 5).unwrap(), true);
assert_eq!(*auto.get(6, 5).unwrap(), true);
```

## API

- **`Automaton<T>`** — Generic CA grid
  - `new(width, height, rule, neighborhood)` — Initialize with a rule function
  - `set(x, y, value)` / `get(x, y)` — Cell access
  - `step()` — Advance one generation
  - `step_n(n)` — Advance n generations
  - `count_alive()` — Count non-default cells
- **`Neighborhood`** — Enum: `Moore` (8 neighbors), `VonNeumann` (4 neighbors)
- **`conway_rule(neighbors, current)` → `bool`** — Built-in Conway's Game of Life rule

## Architecture Notes

Provides the simulation framework for SuperInstance computational experiments. The generic rule design enables rapid prototyping of custom cellular automata for any domain. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
