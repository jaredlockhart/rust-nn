#![feature(core)]

use std::num::Float;
use std::fmt;
extern crate rand;

// sigmoid
fn sigmoid(x : f64) -> f64 {
    return 1.0 / (1.0 + std::f64::consts::E.powf(-x));
}

// A neuron is a collection of weights
#[derive(Debug)]
pub struct Neuron {
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

    fn feed(&self, pattern: &Vec<f64>) -> f64 {
        let mut sum = 0.0;
        for i in range(0, pattern.len()) {
            sum += pattern[i] * self.weights[i];
        }
        return sigmoid(sum);
    }
}

impl fmt::Display for Neuron {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Neuron: {:?}", self.weights)
    }
}

// A layer is a collection of Neurons
#[derive(Debug)]
pub struct Layer {
    neurons : Vec<Neuron>
}

impl Layer {
    fn new(num_neurons : i64, num_inputs: i64) -> Layer {
        let mut neurons = Vec::new();

        for _ in range(0, num_neurons) {
            let neuron = Neuron::new(num_inputs);
            neurons.push(neuron);
        }

        return Layer{ neurons: neurons };
    }

    fn feed(&self, pattern : &Vec<f64>) -> Vec<f64> {
        let mut output = Vec::new();

        for neuron in self.neurons.iter() {
            output.push(neuron.feed(pattern));
        }
        return output;
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Layer:\n");
        for neuron in self.neurons.iter() {
            write!(formatter, "\t{:?}\n", neuron);
        }
        return write!(formatter, "\n");
    }
}

// A Network is a collection of Layers
#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>
}

impl Network {
    pub fn new(topology : Vec<i64>) -> Network {
        let mut layers = Vec::new();

        for i in range(0, topology.len() - 1) {
            let num_inputs = topology[i];
            let num_outputs = topology[i+1];
            layers.push(Layer::new(num_outputs, num_inputs));
        }

        return Network{ layers: layers };
    }

    pub fn feed(&self, pattern : &Vec<f64>) -> Vec<f64> {
        let mut output = pattern.clone();

        for layer in self.layers.iter() {
            output = layer.feed(&output);
        }
        return output;
    }
}

impl fmt::Display for Network {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Network:\n");
        for layer in self.layers.iter() {
            write!(formatter, "\t{:?}\n", layer);
        }
        return write!(formatter, "\n");
    }
}

