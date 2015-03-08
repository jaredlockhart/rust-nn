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
        let _ = write!(formatter, "Layer:\n");
        for neuron in self.neurons.iter() {
            let _ = write!(formatter, "\t{:?}\n", neuron);
        }
        return write!(formatter, "\n");
    }
}

//#[derive(Debug)]
struct LayerMatrix {
    weights: Vec<Vec<f64>>,
    activation_function: fn(f64) -> f64 // map to range -1..1
}

impl LayerMatrix {
    fn new(weights: Vec<Vec<f64>>, activation_function: fn(f64) -> f64) -> LayerMatrix {
        return LayerMatrix{ weights: weights, activation_function : activation_function };
    }

    fn feed(&self, pattern : &Vec<f64>) -> Vec<f64> {
        let mut output = Vec::with_capacity(self.weights.len());

        for neuron_weights in self.weights.iter() {
            let mut res = 0f64;
            for i in 0..neuron_weights.len() {
                res += neuron_weights[i] * pattern[i];
            }
            output.push((self.activation_function)(res));
        }
        return output;
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
        let _ = write!(formatter, "Network:\n");
        for layer in self.layers.iter() {
            let _ = write!(formatter, "\t{:?}\n", layer);
        }
        return write!(formatter, "\n");
    }
}

// A HopfieldNetwork is a single layer network with matrix compute by hebb or hamming algorithm
//#[derive(Debug)]
pub struct HopfieldNetwork {
    layer: LayerMatrix
}

impl HopfieldNetwork {
    pub fn new(patterns: Vec<Vec<f64>>, activation_function: fn(f64) -> f64) -> HopfieldNetwork {
        let layer = LayerMatrix::new(hebb_study(patterns), activation_function);

        HopfieldNetwork{ layer: layer }
    }

    pub fn feed(&self, pattern : &Vec<f64>) -> Vec<f64> {
        self.layer.feed(pattern)
    }
}

fn hebb_study(patterns : Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let cnt = patterns.len();
    let size = patterns[0].len();
    let mut weights : Vec<Vec<f64>> = Vec::with_capacity(size);

    for r in 0..size {
        weights.push(Vec::with_capacity(size));
        for c in 0..size {
            if r == c {
                weights[r].push(0f64);
            } else {
                let mut sum = 0f64;
                for k in 0..cnt {
                    sum += patterns[k][r] * patterns[k][c];
                }
                weights[r].push(sum);
            }
        }
    }

    weights
}
