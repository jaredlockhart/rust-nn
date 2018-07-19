extern crate rand;

pub fn random_weight() -> f64 {
    (rand::random::<f64>() * 2.0) - 1.0
}

fn random_weights(n: usize) -> Vec<f64> {
    (0..n).map(|_| random_weight()).collect()
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn perceptron(inputs: &Vec<f64>, weights: &[f64]) -> f64 {
    sigmoid(
        inputs
            .iter()
            .zip(weights.iter())
            .map(|(input, weight)| input * weight)
            .sum(),
    )
}

fn feed_layer(inputs: &Vec<f64>, layer_weights: &[f64]) -> Vec<f64> {
    layer_weights
        .chunks(inputs.len())
        .map(|weights| perceptron(inputs, weights))
        .collect()
}

#[derive(Debug, Clone)]
pub struct Network {
    pub inputs: usize,
    pub hidden: usize,
    pub weights: Vec<f64>,
    pub score: f64,
}

impl Network {
    pub fn new_random(inputs: usize, hidden: usize, outputs: usize) -> Self {
        Network {
            inputs: inputs,
            hidden: hidden,
            weights: random_weights((inputs * hidden) + (hidden * outputs)),
            score: 0.0,
        }
    }

    pub fn feed(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let cutoff = self.inputs * self.hidden;
        let output = feed_layer(inputs, &self.weights[..cutoff]);
        feed_layer(&output, &self.weights[cutoff..])
    }
}
