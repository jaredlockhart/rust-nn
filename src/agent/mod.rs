mod network;

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

fn xor_trial(network: &network::Network) -> f64 {
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
    network: network::Network,
}

pub fn generation(popsize: usize) {
    let mut agents: Vec<Agent> = Vec::new();

    for agent_i in 0..popsize {
        let network = network::Network::new_random(2, 3, 1);
        let score = xor_trial(&network);
        agents.push(Agent {
            score: score,
            network: network,
        });
    }

    agents[..].sort_unstable_by(|a, b| cmp_f64(b.score, a.score));
    for agent in agents.iter() {
        println!("Score: {:?}", agent.score);
    }
}
