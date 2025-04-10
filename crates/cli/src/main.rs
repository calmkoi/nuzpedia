use engine::{
    PokemonGen1, MoveGen1, MoveCategory, TypeGen1, StatsGen1, 
    DamageRoll, StatusGen1, calc_damage_gen_1
};

fn main() {
    // Example Pokémon
    let pikachu = PokemonGen1 { 
        name: "Pikachu".into(),
        types: [TypeGen1::Electric, TypeGen1::None],
        stats: StatsGen1 {
            lvl: 100,
            hp: 35,
            attack: 55,
            defense: 30,
            special: 50,
            speed: 90,
        },
        status: StatusGen1::Healthy,
        ..Default::default()
    };

    let charizard = PokemonGen1 {
        name: "Charizard".into(),
        types: [TypeGen1::Fire, TypeGen1::Flying],
        stats: StatsGen1 {
            lvl: 100,
            hp: 78,
            attack: 84,
            defense: 78,
            special: 85,
            speed: 100,
        },
        ..Default::default()
    };

    // Example moves
    let thunderbolt = MoveGen1 {
        name: "Thunderbolt".into(),
        typ: TypeGen1::Electric,
        power: 95,
        category: MoveCategory::Special,
    };

    let flamethrower = MoveGen1 {
        name: "Flamethrower".into(),
        typ: TypeGen1::Fire,
        power: 95,
        category: MoveCategory::Special,
    };

    // Showcase damage calculation features
    println!("=== Gen 1 Damage Calculator ===");
    
    // Basic damage
    println!(
        "Pikachu's Thunderbolt vs Charizard: {} (Max)",
        calc_damage_gen_1(&pikachu, &charizard, &thunderbolt, false, DamageRoll::Max)
    );

    // Burned physical attacker
    let mut burned_charizard = charizard.clone();
    burned_charizard.status = StatusGen1::Burned;
    let slash = MoveGen1 {
        name: "Slash".into(),
        typ: TypeGen1::Normal,
        power: 70,
        category: MoveCategory::Physical,
    };
    
    println!(
        "Burned Charizard's Slash vs Pikachu: {} (Average)",
        calc_damage_gen_1(&burned_charizard, &pikachu, &slash, false, DamageRoll::Average)
    );

    // Critical hit
    println!(
        "Pikachu's Thunderbolt (critical) vs Charizard: {} (Random)",
        calc_damage_gen_1(&pikachu, &charizard, &thunderbolt, true, DamageRoll::Random)
    );

    // STAB demonstration
    println!(
        "Charizard's Flamethrower vs Pikachu: {} (Min)",
        calc_damage_gen_1(&charizard, &pikachu, &flamethrower, false, DamageRoll::Min)
    );
}
