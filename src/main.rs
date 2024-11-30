use std::fs;

fn main() {
    let input = fs::read("everybody_codes_e2024_q01_p1.txt").expect("bad reading");

    let result = input.into_iter().map(monster_to_npotions).sum::<u32>();

    println!("{}", result);
}

fn monster_to_npotions(letter: u8) -> u32 {
    match letter as char {
        'A' | 'x' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => panic!("Got a letter which is not A B or C"),
    }
}
