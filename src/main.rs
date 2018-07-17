extern crate rand;

fn random_weight() -> f64 {
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

fn feed_layer(inputs: &Vec<f64>, weights: &[f64]) -> Vec<f64> {
    weights
        .chunks(inputs.len())
        .map(|input_weights| perceptron(inputs, input_weights))
        .collect()
}

#[derive(Debug)]
struct Network {
    inputs: usize,
    hidden: usize,
    weights: Vec<f64>,
}

impl Network {
    fn new_random(inputs: usize, hidden: usize, outputs: usize) -> Self {
        Network {
            inputs: inputs,
            hidden: hidden,
            weights: random_weights((inputs * hidden) + (hidden * outputs)),
        }
    }

    fn feed(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let cutoff = self.inputs * self.hidden;
        let output = feed_layer(inputs, &self.weights[..cutoff]);
        feed_layer(&output, &self.weights[cutoff..])
    }
}

fn main() {
    let size = 2621440;
    let network = Network::new_random(size, 100, size);
    let input = random_weights(size);
    let output = network.feed(&input);
    assert_eq!(output.len(), size);
}
