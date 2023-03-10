use rand::random;

const N_NUMBERS: usize = 6;

#[derive(Debug)]
struct Compte {
    numbers: Vec<u32>,
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Compte {
    fn new() -> Self {
        let mut allowed_numbers: Vec<u32> = vec![
            1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 25, 50, 75, 100,
        ];
        let mut numbers: Vec<u32> = Vec::new();
        numbers.reserve(N_NUMBERS);

        for _ in 0..N_NUMBERS {
            let pick = allowed_numbers.remove(random::<usize>() % allowed_numbers.len());
            numbers.push(pick);
        }

        return Compte { numbers };
    }
}

fn main() {}
