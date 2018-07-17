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

fn perceptron(inputs: &Vec<f64>, weights: Vec<f64>) -> f64 {
    let zipped = inputs.iter().zip(weights.iter());
    let sum: f64 = zipped.map(|(input, weight)| input * weight).sum();
    sigmoid(sum)
}

fn feed_layer(
    inputs: &Vec<f64>,
    weights: Vec<f64>,
    num_inputs: usize,
    num_outputs: usize,
) -> Vec<f64> {
    let mut outputs: Vec<f64> = Vec::new();

    for output_i in 0..num_outputs {
        let start = output_i * num_inputs;
        let end = (output_i + 1) * num_inputs;
        let output = perceptron(inputs, weights[start..end].to_vec());
        outputs.push(output);
    }

    outputs
}

#[derive(Debug)]
struct Network {
    inputs: usize,
    hidden: usize,
    outputs: usize,
    weights: Vec<f64>,
}

impl Network {
    fn new_random(inputs: usize, hidden: usize, outputs: usize) -> Self {
        Network {
            inputs: inputs,
            hidden: hidden,
            outputs: outputs,
            weights: random_weights((inputs * hidden) + (hidden * outputs)),
        }
    }

    fn feed(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let layer_cutoff = self.inputs * self.hidden;
        let layer1_weights = self.weights[..layer_cutoff].to_vec();
        let layer1_output = feed_layer(inputs, layer1_weights, self.inputs, self.hidden);
        let layer2_weights = self.weights[layer_cutoff..].to_vec();
        feed_layer(&layer1_output, layer2_weights, self.hidden, self.outputs)
    }
}

fn main() {
    let size = 2621440;
    let network = Network::new_random(size, 100, size);
    let input = random_weights(size);
    let output = network.feed(&input);
    assert_eq!(output.len(), size);
}
