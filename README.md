# nuzpedia

nuzpedia is a suite of tools to assist with Pokémon Nuzlocke challenges, starting with a comprehensive damage calculator.

## Current Features

### Generation 1 damage calculator (Partial Implementation)
- Accurate damage calculation formulas
- Type effectiveness multipliers
- STAB (Same-Type Attack Bonus)
- Generation 1-specific critical hit mechanics
- Damage roll options:
    - High/low bounds
    - Average (Mean)
    - Random

## Future Roadmap

### Damage Calculator
- [ ] Complete Generation 1 implementation
- [ ] Extend support through Generation 9

### Planned Modules
- **Team Builder/Planner**
  - Type coverage analysis
  - Move synergy evaluation
  - Encounter location database
- **Nuzlocke Tracker**
  - Route encounter management
  - Death tracking
  - Rule customization
- **Trainer Database**
  - Opponent team previews
  - Move sets and abilities

## Project Structure

This project is organised as a Cargo workspace with two primary crates (more to be added as development progresses):

```mermaid
graph TD
    workspace[nuzpedia]
    workspace --> engine[engine crate]
    workspace --> cli[cli crate]

    engine --> damage[damage.rs]
    engine --> pokemon[pokemon.rs]
    engine --> types[types.rs]
    engine --> poke_move[poke_move.rs]
    
    cli --> main[main.rs]
    main --> engine
```

### Crates Overview

- `engine`: Core calculation logic for Pokemon battles
    - `damage.rs`: Damage calculation formulas
    - `pokemon.rs`: Pokemon data and stats
    - `types.rs`: Type effectiveness and interactions
    - `poke_move.rs`: Move data and effects
- `cli`: Command-line interface for interacting with the engine

### `engine` Architecture

```mermaid
graph TD
    lib[lib.rs] --> damage
    lib --> pokemon
    lib --> types
    lib --> poke_move
    
    damage --> pokemon
    damage --> types
    damage --> poke_move
    poke_move --> types
    
    pokemon --> types
```

### `cli` Implementation

```mermaid
graph LR
    cli[CLI] --> engine[Engine API]
    engine --> calc[Damage Calculator]
    engine --> data[Pokémon Data]
```
## Contributing

On the off chance this repo eventually gains some traction/popularity, feel free to contribute!

### Local Development Setup

1. Clone and enter repo:

```bash
git clone git@github.com:calmkoi/nuzpedia.git
cd nuzpedia
```

2. Install Rust (via rustup) if not already installed
3. Build and run

```bash
cargo build
cargo run
```

4. Commit changes
5. Reset workspace (if required)
