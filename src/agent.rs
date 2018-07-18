extern crate rand;

use network::Network;
use std::cmp::Ordering;

fn cmp_f64(a: f64, b: f64) -> Ordering {
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn signal_error(samples: Vec<f64>, targets: &[f64]) -> f64 {
    samples
        .iter()
        .zip(targets.iter())
        .map(|(sample, target)| (sample - target).powf(2.0))
        .sum::<f64>()
        .sqrt()
}

fn xor_trial(network: &Network) -> f64 {
    let trials = [
        ([0.0, 0.0], [0.0]),
        ([0.0, 1.0], [1.0]),
        ([1.0, 0.0], [1.0]),
        ([1.0, 1.0], [0.0]),
    ];
    trials
        .iter()
        .map(|(input, output)| signal_error(network.feed(&input.to_vec()), output))
        .sum()
}

struct Agent {
    score: f64,
    network: Network,
}

impl Agent {
    fn breed(&mut self, partner: &mut Agent) {
        let crossover = rand::random::<usize>() % self.network.weights.len();
    }
}

pub fn generation(popsize: usize) {
    let mut agents: Vec<Agent> = (0..popsize)
        .map(|_| {
            let network = Network::new_random(2, 3, 1);
            let score = xor_trial(&network);
            Agent {
                score: score,
                network: network,
            }
        })
        .collect();

    agents[..].sort_unstable_by(|a, b| cmp_f64(b.score, a.score));
    for agent in agents.iter() {
        println!("Score: {:?}", agent.score);
    }
}
