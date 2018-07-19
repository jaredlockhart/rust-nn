extern crate rand;

use network::random_weight;
use network::Network;
use std::cmp::Ordering;
use std::rc::Rc;

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
        .map(|(sample, target)| 1.0 - (target - sample).abs())
        .sum::<f64>()
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

impl Network {
    fn breed(&self, partner: &Rc<Network>) -> Rc<Network> {
        let mut new_weights: Vec<f64> = Vec::new();

        // Crossover
        let crossover = rand::random::<usize>() % self.weights.len();
        let direction = rand::random::<usize>() % 2 == 0;
        if direction {
            new_weights.extend_from_slice(&self.weights[..crossover]);
            new_weights.extend_from_slice(&partner.weights[crossover..]);
        } else {
            new_weights.extend_from_slice(&partner.weights[..crossover]);
            new_weights.extend_from_slice(&self.weights[crossover..]);
        }

        // Mutation
        let mutation = random_weight() * 0.2;
        new_weights = new_weights.iter().map(|weight| weight * mutation).collect();

        // Random material
        let random_chance = rand::random::<usize>() % 2 == 0;
        if random_chance {
            let random_position = rand::random::<usize>() % new_weights.len();
            new_weights[random_position] = random_weight();
        }

        assert_eq!(self.weights.len(), new_weights.len());

        // Child
        Rc::new(Network {
            inputs: self.inputs,
            hidden: self.hidden,
            weights: new_weights,
            score: self.score,
        })
    }
}

pub fn random_population(popsize: usize) -> Vec<Rc<Network>> {
    (0..popsize)
        .map(|_| Rc::new(Network::new_random(2, 3, 1)))
        .collect()
}

fn random_mate(population: &Vec<Rc<Network>>) -> &Rc<Network> {
    &population[rand::random::<usize>() % population.len()]
}

pub fn generation(population: Vec<Rc<Network>>) -> Vec<Rc<Network>> {
    let mut scored_networks: Vec<(f64, &Rc<Network>)> = population
        .iter()
        .map(|network| (xor_trial(network), network))
        .collect();

    scored_networks[..].sort_unstable_by(|(a, _), (b, _)| cmp_f64(*b, *a));

    let mut improved_networks: Vec<(f64, Rc<Network>)> = scored_networks
        .iter()
        .map(|(new_score, network)| ((new_score - network.score), Rc::clone(network)))
        .collect();

    improved_networks[..].sort_unstable_by(|(a, _), (b, _)| cmp_f64(*b, *a));

    let (top_score, top_network) = &scored_networks[0];
    let (improved_score, improved_network) = &improved_networks[0];
    let (bottom_score, bottom_network) = &scored_networks[scored_networks.len() - 1];
    println!(
        "Top Score: {:?} Impoved Score: {:?} Bottom Score: {:?}",
        top_score, improved_score, bottom_score
    );

    let mut new_population: Vec<Rc<Network>> = Vec::new();

    // Top performers
    let top_cutoff = (scored_networks.len() as f64 * 0.1) as usize;
    let top_networks = &scored_networks[..top_cutoff];
    for (_, network) in top_networks {
        new_population.push(Rc::clone(network));
    }

    // Top performers children
    for (_, parent) in top_networks {
        let mate = random_mate(&population);
        new_population.push(parent.breed(mate));
        new_population.push(mate.breed(parent));
    }

    // Random population members
    let random_networks: Vec<&Rc<Network>> = population
        .iter()
        .filter(|_| rand::random::<usize>() % 3 == 0)
        .collect();

    for random_network in random_networks {
        let mate = random_mate(&population);
        new_population.push(random_network.breed(mate));
    }

    // New Networks
    let networks_remaining = population.len() - new_population.len();
    for _ in 0..networks_remaining {
        new_population.push(Rc::new(Network::new_random(2, 3, 1)));
    }

    assert_eq!(population.len(), new_population.len());
    new_population
}
