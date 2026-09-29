fn main() {
    let population = [9, 10, 11, 12, 13, 14, 15, 16];
    let num_rows = 5000;
    let num_cols = 10;

    // Minimal Xorshift64 PRNG using pure std
    let mut state: u64 = 42;
    let mut next_usize = |limit: usize| -> usize {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state as usize) % limit
    };

    let mut samples = vec![vec![0; num_cols]; num_rows];
    for col in 0..num_cols {
        for row in samples.iter_mut() {
            let idx = next_usize(population.len());
            row[col] = population[idx];
        }
    }

    for row in samples.iter().take(6) {
        println!("{row:?}");
    }
}
