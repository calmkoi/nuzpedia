use engine::{PokemonGen1, MoveGen1, MoveCategory, TypeGen1, StatsGen1, calc_damage_gen_1};

fn main() {
    let pikachu = PokemonGen1 { 
        name: "Pikachu".into(), 
        types: [TypeGen1::Electric, TypeGen1::None], 
        stats: StatsGen1 {lvl: 100, hp: 35, attack: 55, defense: 30, special: 50, speed: 90}, 
    };
    let thunderbolt = MoveGen1 { 
        name: "Thunderbolt".into(),
        typ: TypeGen1::Electric,
        power: 95,
        category: MoveCategory::Special,
    };
    println!("Damage: {}", calc_damage_gen_1(&pikachu, &pikachu, &thunderbolt));
}
