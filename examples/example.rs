extern crate neuralnetwork;

use neuralnetwork::Network;

fn main() {
    let input = vec!(1.0, 1.0);
    let network = Network::new(vec!(2, 3, 1));
    let output = network.feed(&input);
    println!("{:?}", network);
    println!("Input: {:?}", input);
    println!("Output: {:?}", output);
}
