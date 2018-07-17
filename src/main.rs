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
        -magnitude
    }
}

fn random_vector(size: usize) -> Vec<f64> {
    (0..size).map(|_| random_weight()).collect()
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

impl Neuron {
    fn new(num_weights: usize) -> Neuron {
        Neuron {
            weights: random_vector(num_weights),
        }
    }
}

impl Feedable for Neuron {
    fn feed(&self, inputs: Vec<f64>) -> Vec<f64> {
        let zipped = inputs.iter().zip(self.weights.iter());
        let sum = zipped.map(|(input, weight)| input * weight).sum::<f64>();
        let output = sigmoid(sum);
        vec![output]
    }
}

fn main() {
    let neuron = Neuron::new(3);
    let output = neuron.feed(random_vector(3));
    println!("Hello, neuron: {:?}", neuron);
    println!("Output: {:?}", output);
}
