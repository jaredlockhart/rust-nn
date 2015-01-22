use std::fmt;
use std::rand;

// A neuron is a collection of weights
struct Neuron {
    weights : Vec<f64> 
}

impl Neuron {
    fn new(num_weights: i64) -> Neuron {
        let mut weights = Vec::new(); 

        for _ in range(0, num_weights) {
            // A random weight in [-1.0, 1.0]
            let weight = rand::random::<f64>() * 2.0 - 1.0;
            weights.push(weight);
        } 

        return Neuron{ weights: weights } 
    }
}

impl std::fmt::Show for Neuron {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Neuron: {:?}", self.weights)
    }
}

// A layer is a collection of Neurons
struct Layer {
    neurons : Vec<Neuron>
}

impl Layer {
    fn new(num_neurons : i64, num_inputs: i64) -> Layer {
        let mut neurons = Vec::new();

        for _ in range(0, num_neurons) {
            let neuron = Neuron::new(num_inputs);
            neurons.push(neuron);
        }

        return Layer{ neurons: neurons } 
    }
}

impl std::fmt::Show for Layer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Layer:\n");
        for neuron in self.neurons.iter() {
            write!(formatter, "\t{:?}\n", neuron);
        }
        return write!(formatter, "\n");
    }
}

// A Network is a collection of Layers
struct Network {
    layers: Vec<Layer>
}

impl Network {
    fn new(num_inputs : i64, num_hidden : i64, num_outputs : i64) -> Network {
        let mut layers = Vec::new();

        layers.push(Layer::new(num_outputs, num_hidden));
        layers.push(Layer::new(num_hidden, num_inputs));

        return Network{ layers: layers } 
    }
}

impl std::fmt::Show for Network {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Network:\n");
        for layer in self.layers.iter() {
            write!(formatter, "\t{:?}\n", layer);
        }
        return write!(formatter, "\n");
    }
}

fn main() {
    let network = Network::new(2, 3, 1);
    println!("{:?}", network);
}
