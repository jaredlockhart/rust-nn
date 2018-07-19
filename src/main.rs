pub mod agent;
pub mod network;

fn main() {
    let mut population = agent::random_population(100);

    for _ in 0..1000 {
        population = agent::generation(population);
    }
}
