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

    fn feed(&self, inputs: &Vec<f64>) -> f64 {
        let zipped = inputs.iter().zip(self.weights.iter());
        let sum: f64 = zipped.map(|(input, weight)| input * weight).sum();
        sigmoid(sum)
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn new(num_inputs: usize, num_outputs: usize) -> Self {
        Layer {
            neurons: (0..num_outputs).map(|_| Neuron::new(num_inputs)).collect(),
        }
    }

    fn feed(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut output: Vec<f64> = Vec::new();

        for neuron in self.neurons.iter() {
            let neuron_output = neuron.feed(inputs);
            output.push(neuron_output);
        }

        output
    }
}

#[derive(Debug)]
struct Network {
    layer1: Layer,
    layer2: Layer,
}

impl Network {
    fn new(num_inputs: usize, num_hidden: usize, num_outputs: usize) -> Self {
        Network {
            layer1: Layer::new(num_inputs, num_hidden),
            layer2: Layer::new(num_hidden, num_outputs),
        }
    }

    fn feed(&self, inputs: &Vec<f64>) -> Vec<f64> {
        self.layer2.feed(&self.layer1.feed(inputs))
    }
}

fn main() {
    let neuron = Neuron::new(3);
    let layer = Layer::new(2, 3);
    let network = Network::new(2, 3, 1);
    let input = vec![1.0, 1.0];
    let output = network.feed(&input);
    println!("Hello, neuron: {:?}", neuron);
    println!("Hello, layer: {:?}", layer);
    println!("Hello, network: {:?}", network);
    println!("\nInput: {:?}", input);
    println!("\nOutput: {:?}", output);
}
