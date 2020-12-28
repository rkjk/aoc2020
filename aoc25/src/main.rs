fn get_loop_size(target: u64, seed: u64, remainder: u64) -> usize {
    let mut loop_size = 0;
    let mut init = 1;
    loop {
        loop_size += 1;
        init *= seed;
        init %= remainder;
        if init == target {
            break;
        }
    }
    loop_size
}

fn transform_subject_number(subject_number: u64, loop_size: usize, remainder: u64) -> u64 {
    let mut init = 1;
    for _ in 0..loop_size {
        init *= subject_number;
        init %= remainder;
    }
    init
}

fn main() {
    //let card_public_key = 5764801;
    //let door_public_key = 17807724;
    let card_public_key = 8184785;
    let door_public_key = 5293040;
    let remainder = 20201227;
    let card_loop_size = get_loop_size(card_public_key, 7, remainder);
    let door_loop_size = get_loop_size(door_public_key, 7, remainder);
    println!("Card loop size: {}", card_loop_size);
    println!("Door loop size: {}", door_loop_size);
    println!(
        "Part 1: {}",
        transform_subject_number(card_public_key, door_loop_size, remainder)
    )
}
