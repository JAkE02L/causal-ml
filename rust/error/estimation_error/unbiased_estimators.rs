use rand::seq::IndexedRandom;

fn main() {
    let population = [9, 10, 11, 12, 13, 14, 15, 16];
    let num_rows = 5000;
    let num_cols = 10;

    let mut rng = rand::rng();
    let mut samples = vec![vec![0; num_cols]; num_rows];

    for col in 0..num_cols {
        for row in samples.iter_mut() {
            row[col] = *population.choose(&mut rng).unwrap();
        }
    }

    for row in samples.iter().take(6) {
        println!("{row:?}");
    }
}
