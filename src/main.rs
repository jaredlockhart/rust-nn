extern crate rand;

trait Feedable {
    fn feed(&self, Vec<f64>) -> Vec<f64>;
}

#[derive(Debug)]
struct Neuron {
    weights: Vec<f64>,
}

fn random_weight() -> f64 {
    let magnitude = rand::random::<f64>();
    let sign = rand::random::<i32>() % 2 == 0;

    if sign {
        magnitude
    } else {
        -1.0 * magnitude
    }
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-1.0 * x).exp())
}

impl Neuron {
    fn new(num_weights: usize) -> Neuron {
        Neuron{
            weights: (0..num_weights).map(|_| random_weight()).collect::<Vec<f64>>()
        }
    }
}

impl Feedable for Neuron {
    fn feed(&self, input: Vec<f64>) -> Vec<f64> {
        let mut sum = 0.0;
        for (input_n, f64) in input.iter().zip(self.weights.iter()) {
            sum += input_n * f64;
        }
        vec![sigmoid(sum)]
    }
}

fn main() {
    let neuron = Neuron::new(3);
    let output = neuron.feed(vec![2.0, 2.0, 2.0]);
    println!("Hello, neuron: {:?}", neuron);
    println!("Output: {:?}", output);
}