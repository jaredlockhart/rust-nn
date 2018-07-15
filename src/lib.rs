trait Feedable {
    fn feed(&self, Vec<f64>) -> Vec<f64>;
}

#[derive(Debug)]
struct Neuron {
    weights: Vec<f64>,
}

impl Neuron {
    fn new(num_weights: usize) -> Neuron {
        let mut neuron = Neuron{weights: Vec::new()};
        for n in 0..num_weights {
            neuron.weights.insert(n, 1.0);
        }
        neuron
    }
}

impl Feedable for Neuron {
    fn feed(&self, input: Vec<f64>) -> Vec<f64> {
        let mut sum = 0.0;
        
        for pair in input.iter().zip(self.weights.iter()) {
            let (input_n, weight) = pair;
            sum += input_n * weight;
        }
        
        let mut output: Vec<f64> = Vec::new();
        output.insert(0, sum);
        output
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn new(num_inputs: usize, num_neurons: usize) -> Layer {
        let mut layer = Layer{neurons: Vec::new()};
        for n in 0..num_neurons {
            layer.neurons.insert(n, Neuron::new(num_inputs));
        }
        layer
    }
}

impl Feedable for Layer {
    fn feed(&self, input: Vec<f64>) -> Vec<f64> {
        let mut output: Vec<f64> = Vec::new();

        for pair in input.iter().zip(self.neurons.iter()) {
            let (input_n, neuron) = pair;

        }
        output
    }
}

fn main() {
    let neuron = Neuron::new(3);
    let output = neuron.feed(vec![2.0, 2.0, 2.0]);
    println!("Hello, neuron: {:?}", neuron);
    println!("Output: {:?}", output);
}

