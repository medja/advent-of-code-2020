const M: usize = 20201227;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let card_key = input[0].parse::<usize>().unwrap();
    let door_key = input[1].parse::<usize>().unwrap();

    let (public_key, private_key) = brute_force_key_pair(card_key, door_key);

    let subject = if public_key == card_key {
        door_key
    } else {
        card_key
    };

    Ok(encrypt(subject, private_key))
}

fn encrypt(subject: usize, private_key: usize) -> usize {
    (0..private_key).fold(1, |key, _| (key * subject) % M)
}

fn brute_force_key_pair(card_key: usize, door_key: usize) -> (usize, usize) {
    let mut public_key = 1;
    let mut private_key = 0;

    loop {
        public_key = (public_key * 7) % M;
        private_key += 1;

        if public_key == card_key || public_key == door_key {
            break (public_key, private_key);
        }
    }
}
