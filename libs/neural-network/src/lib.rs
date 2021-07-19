#![feature(crate_visibility_modifier)]
use rand::thread_rng;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::fmt;
use std::iter::once;

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    crate fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(rng: &mut dyn rand::RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .cloned()
    }

    

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }
    
}

pub struct LayerTopology {
    pub neurons: usize,
}

struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());

        assert!(neurons
            .iter()
            .all(|neuron| neuron.weights.len() == neurons[0].weights.len()));

        Self { neurons }
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }

    pub fn random(
        rng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize,
    ) -> Self {
        let mut neurons = Vec::new();

        for _ in 0..output_neurons {
            neurons.push(Neuron::random(rng, input_neurons));
        }

        Self { neurons }
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        assert!(!weights.is_empty());

        Self { bias, weights }
    }

    pub fn from_weights(output_neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }

    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}

impl fmt::Debug for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hi: {}", "lol")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod random {
        use super::*;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            approx::assert_relative_eq!(neuron.bias, -0.6255188);
            approx::assert_relative_eq!(
                neuron.weights.as_slice(),
                &[0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
            );
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn test() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            // Ensures `.max()` (our ReLU) works:
            approx::assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

            // `0.5` and `1.0` chosen by a fair dice roll:
            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );

            // We could've written `1.15` right away, but showing the entire
            // formula makes our intentions clearer
        }
    }

    mod weights {
        use super::*;

        #[test]
        fn test() {
            let network = Network::new(vec![
                Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
                Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
            ]);

            let actual: Vec<_> = network.weights().collect();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }

    mod from_weights {
        use super::*;

        #[test]
        fn test() {
            let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];

            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights().collect();

            approx::assert_relative_eq!(actual.as_slice(), weights.as_slice(),);
        }
    }
}
