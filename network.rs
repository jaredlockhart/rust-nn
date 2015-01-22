use std::fmt;
use std::rand;

// Generate a random weight [-1.0, 1.0]
fn get_weight() -> f64 {
    return rand::random::<f64>() * 2.0 - 1.0;
}

// Generate a vector of random weights
fn get_weights(num_weights : i64) -> Vec<f64> {
    let mut weights = Vec::new(); 

    for _ in range(0, num_weights) {
        let weight = get_weight(); 
        weights.push(weight);
    } 

    return weights;
}

// A neuron is a collection of weights
struct Neuron {
    weights : Vec<f64> 
}

impl Neuron {
    fn new(num_weights: i64) -> Neuron {
        return Neuron{ weights: get_weights(num_weights) } 
    }
    
}

impl std::fmt::Show for Neuron {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Weights: {:?}", self.weights)
    }
}

fn get_neurons(num_neurons : i64, num_inputs : i64) -> Vec<Neuron> {
    let mut neurons = Vec::new();

    for _ in range(0, num_neurons) {
        let neuron = Neuron::new(num_inputs);
        neurons.push(neuron);
    }

    return neurons;
}

struct Layer {
    neurons : Vec<Neuron>
}

impl Layer {
    fn new(num_neurons : i64, num_inputs: i64) -> Layer {
        return Layer{ neurons: get_neurons(num_neurons, num_inputs) } 
    }
    
}

impl std::fmt::Show for Layer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Layer:\n");
        for neuron in self.neurons.iter() {
            write!(formatter, "Neuron: {:?}\n", neuron);
        }
        return write!(formatter, "\n");
    }
}

fn main() {

    let neuron = Neuron::new(3); 
    println!("{:?}", neuron); 

    let layer = Layer::new(3, 2);
    println!("{:?}", layer);
}
