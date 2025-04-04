use engine::{Pokemon, Move, MoveCategory, Type, Stats, calculate_gen1};

fn main() {
    let pikachu = Pokemon { 
        name: "Pikachu".into(), 
        types: [Type::Electric, Type::None], 
        stats: Stats {lvl: 100, hp: 35, attack: 55, defense: 30, special: 50, speed: 90}, 
    };
    let thunderbolt = Move { 
        name: "Thunderbolt".into(),
        typ: Type::Electric,
        power: 95,
        category: MoveCategory::Special,
    };
    println!("Damage: {}", calculate_gen1(&pikachu, &pikachu, &thunderbolt));
}
