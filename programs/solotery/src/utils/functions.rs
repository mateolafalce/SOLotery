pub fn lamports_to_sol(lamports: u64) -> f64 {
    let sols: f64 = lamports as f64 / 1_000_000_000.0;
    sols
}
