# cellular-automata

A generic, parameterizable 2D **cellular automaton** library in Rust supporting arbitrary cell types, user-defined transition rules, and both Moore and von Neumann neighborhoods. Ships with Conway's Game of Life as a built-in rule.

## Why It Matters

Unlike fixed-purpose automaton simulators, this crate provides a **generic framework** where:

- Cell type `T` is user-defined (bool, enum, integer, custom struct)
- The transition function `RuleFn<T>` is any `fn(&[Option<T>], &T) -> T`
- The neighborhood geometry is selectable (Moore = 8 neighbors, von Neumann = 4)

This generality supports:

- **Multi-state automata** — cyclic cellular automata, rock-paper-scissors models
- **Continuous CAs** — reaction-diffusion (Gray-Scott) with `f64` cells
- **Lattice models** — Ising model spin dynamics with `i8` cells
- **Ecological models** — predator-prey cellular automata with agent-type cells

## How It Works

### Generic Grid Architecture

```rust
pub struct Automaton<T: Clone> {
    grid: Vec<Vec<T>>,
    rule: RuleFn<T>,
    neighborhood: Neighborhood,
}
```

The grid is a dense `Vec<Vec<T>>` (row-major). Each step clones the grid (double-buffered update) and applies the rule:

$$S_{t+1}(x, y) = f\big(\{S_t(n_i)\}_{\text{neighbors}}, S_t(x,y)\big)$$

### Neighborhoods

| Neighborhood | Cells | Offsets |
|-------------|-------|---------|
| **Moore** | 8 | (±1,±1), (0,±1), (±1,0) |
| **von Neumann** | 4 | (0,±1), (±1,0) |

Moore is standard for Life-like CAs. Von Neumann is used in lattice gas automata and some epidemiological models (SIR with nearest-neighbor contact).

### Conway's Rule (Built-in)

```rust
pub fn conway_rule(neighbors: &[Option<bool>], current: &bool) -> bool {
    let alive = neighbors.iter().filter(|n| n.unwrap_or(false)).count();
    match (current, alive) {
        (true, 2) | (true, 3) => true,   // survival
        (false, 3) => true,               // birth
        _ => false,                       // death/stasis
    }
}
```

### Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| `set(x, y, v)` | O(1) | O(1) |
| `get(x, y)` | O(1) | O(1) |
| `step()` | O(W × H × |N|) | O(W × H) |
| `step_n(n)` | O(n × W × H × |N|) | O(W × H) |

Where |N| is the neighborhood size (8 for Moore, 4 for von Neumann).

The dominant cost is the grid clone + full scan. For a W×H grid with Moore neighborhood, each step performs W×H×8 neighbor lookups plus W×H rule evaluations.

### Boundary Handling

Out-of-bounds neighbors return `None`, which the rule function handles explicitly. This is equivalent to a **fixed (dead) boundary** for boolean CAs.

## Quick Start

```rust
use cellular_automata::{Automaton, Neighborhood, conway_rule};

let mut auto = Automaton::new(5, 5, conway_rule, Neighborhood::Moore);

// Place a blinker (vertical line → oscillates to horizontal)
auto.set(2, 1, true);
auto.set(2, 2, true);
auto.set(2, 3, true);

auto.step(); // becomes horizontal
assert!(auto.get(1, 2));
assert!(auto.get(3, 2));
auto.step(); // back to vertical
assert!(auto.get(2, 1));
```

## API

| Type / Method | Description |
|---------------|-------------|
| `Automaton::new(w, h, rule, neighborhood)` | Generic constructor |
| `set(x, y, value)` | Set cell state |
| `get(x, y) → Option<&T>` | Read cell state |
| `step()` | One synchronous generation |
| `step_n(n)` | Advance n generations |
| `count_alive()` | Count non-default cells |
| `conway_rule` | Built-in Game of Life rule |
| `Neighborhood::Moore` / `VonNeumann` | Geometry selector |

## Architecture Notes

The **γ + η = C** link: the neighborhood collection (γ) gathers the local context for each cell, while the user-supplied rule function (η) applies the deterministic transition. Together they conserve the synchronicity invariant C — all cells in generation $t+1$ are computed from the snapshot of generation $t$ (via the clone before the update loop), ensuring no information propagates within a single step. This is the mathematical definition of a CA map $F: S^T \to S^T$ applied uniformly.

## References

- von Neumann, J. (1966). *Theory of Self-Reproducing Automata.* University of Illinois Press.
- Wolfram, S. (1984). *Universality and Complexity in Cellular Automata.* Physica D, 10(1–2), 1–35.
- Cook, M. (2004). *Universality in Elementary Cellular Automata.* Complex Systems, 15(1), 1–40. (Rule 110 is Turing-complete.)
- Gardner, M. (1970). *The Fantastic Combinations of John Conway's New Solitaire Game "Life."* Scientific American.
- Toffoli, T., & Margolus, N. (1987). *Cellular Automata Machines.* MIT Press.

## License

MIT
