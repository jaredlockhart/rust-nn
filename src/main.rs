extern crate rand;

fn random_weight() -> f64 {
    (rand::random::<f64>() * 2.0) - 1.0
}

fn random_weights(size: usize) -> Vec<f64> {
    (0..size).map(|_| random_weight()).collect()
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

trait Feedable {
    fn feed(&self, &Vec<f64>) -> Vec<f64>;
}

#[derive(Debug)]
struct Neuron {
    weights: Vec<f64>,
}

impl Neuron {
    fn new(num_weights: usize) -> Self {
        Self {
            weights: random_weights(num_weights),
        }
    }
}

impl Feedable for Neuron {
    fn feed(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let zipped = inputs.iter().zip(self.weights.iter());
        let sum = zipped.map(|(input, weight)| input * weight).sum::<f64>();
        let output = sigmoid(sum);
        vec![output]
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn new(num_neurons: usize, num_inputs: usize) -> Self {
        Layer {
            neurons: (0..num_neurons).map(|_| Neuron::new(num_inputs)).collect(),
        }
    }
}

impl Feedable for Layer {
    fn feed(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut output: Vec<f64> = Vec::new();

        for neuron in self.neurons.iter() {
            let neuron_output = neuron.feed(inputs);
            output.extend_from_slice(&neuron_output);
        }

        output
    }
}

fn main() {
    let neuron = Neuron::new(3);
    let output = neuron.feed(&random_weights(3));
    let layer = Layer::new(2, 3);
    println!("Hello, neuron: {:?}", neuron);
    println!("Hello, layer: {:?}", layer);
    println!("Output: {:?}", output);
}
