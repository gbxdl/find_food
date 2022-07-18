use crate::grid::Grid;
use fastapprox::faster::tanh;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use rulinalg::utils;
use std::clone::Clone;
use std::iter::zip;

pub struct Prey {
    pub position: usize,
    pub prev_position: usize,
    view_distance: usize,
    pub energy: usize,
    pub split_count: usize,
    pub neural_net: NeuralNet,
}

impl Prey {
    pub fn new(position: usize, view_distance: usize, max_energy: usize, nn: NeuralNet) -> Prey {
        let neural_net = match nn.layers[..] {
            [] => NeuralNet::new(vec![view_distance * 4 + 1, 5, 5]),
            _ => nn.similar_child(),
        };
        Prey {
            position,
            prev_position: position,
            view_distance,
            energy: max_energy,
            split_count: 0,
            neural_net,
        }
    }

    pub fn take_step(&mut self, grid: &Grid) {
        let mut view = self.look(grid);
        self.prev_position = self.position;

        let max_pos = grid.width * grid.height;
        view.push(self.energy as f32);
        let output = self.neural_net.compute(&view);

        // if self.energy == 0 {
        //     self.energy += 1;
        //     return;
        // }

        self.energy -= 1;

        match utils::argmax(&output).0 {
            0 => (),
            1 => self.position = (self.position + 1) % max_pos,
            2 => self.position = (self.position + max_pos - 1) % max_pos,
            3 => self.position = (self.position + grid.width) % max_pos,
            _ => self.position = (self.position + max_pos - grid.width) % max_pos,
        }
    }

    fn look(&self, grid: &Grid) -> Vec<f32> {
        let mut out = vec![];
        let position = self.position;
        let max_pos = grid.width * grid.height;
        for i in 1..self.view_distance + 1 {
            out.push(grid.ternary[(position + i) % max_pos] as f32);
            out.push(grid.ternary[(position + max_pos - i) % max_pos] as f32);
            out.push(grid.ternary[(position + i * grid.width) % max_pos] as f32);
            out.push(grid.ternary[(position + i * (max_pos - grid.width)) % max_pos] as f32);
        }
        out
    }
}

#[derive(Clone)]
pub struct NeuralNet {
    pub layers: Vec<Layer>,
}

impl NeuralNet {
    fn new(architecture: Vec<usize>) -> NeuralNet {
        let mut layers = vec![];
        let mut n_inputs = 0;
        for n_neurons in architecture {
            if n_inputs != 0 {
                layers.push(Layer::new(n_inputs, n_neurons));
            }
            n_inputs = n_neurons;
        }
        NeuralNet { layers }
    }

    fn similar_child(&self) -> NeuralNet {
        let mut layers = vec![];
        for layer in &self.layers {
            layers.push(layer.similar_child());
        }
        NeuralNet { layers }
    }

    fn compute(&self, inputs: &Vec<f32>) -> Vec<f32> {
        let mut outputs = vec![0.0];
        let mut inputs: &Vec<f32> = inputs;

        for layer in &self.layers {
            outputs = layer.compute(inputs);
            inputs = &outputs;
        }
        outputs
    }
}

#[derive(Clone)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}

impl Layer {
    fn new(n_inputs: usize, n_neurons: usize) -> Layer {
        let mut neurons = vec![];

        for _ in 0..n_neurons {
            neurons.push(Neuron::new(n_inputs));
        }
        Layer { neurons }
    }

    fn similar_child(&self) -> Layer {
        let mut neurons = vec![];
        for neuron in &self.neurons {
            neurons.push(neuron.similar_child());
        }
        Layer { neurons }
    }

    fn compute(&self, inputs: &Vec<f32>) -> Vec<f32> {
        let mut outputs = vec![0.0];
        for neuron in &self.neurons {
            outputs.push(neuron.compute(inputs));
        }
        outputs
    }
}

#[derive(Clone)]
pub struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

impl Neuron {
    fn new(n_inputs: usize) -> Neuron {
        let mut neuron = Neuron {
            weights: vec![0.0; n_inputs],
            bias: 0.0,
        };
        neuron.random_mutation();
        neuron
    }

    fn similar_child(&self) -> Neuron {
        let mut neuron = Clone::clone(self);
        neuron.random_mutation();
        neuron
    }

    pub fn random_mutation(&mut self) {
        let prob = 0.1;
        for weigth in &mut self.weights {
            let coin = rand::thread_rng().gen::<f32>();
            if coin < prob {
                *weigth += 0.1 * thread_rng().sample::<f32, _>(StandardNormal);
            }
        }
        let coin = rand::thread_rng().gen::<f32>();
        if coin < prob {
            self.bias += 0.1 * thread_rng().sample::<f32, _>(StandardNormal);
        }
    }

    fn compute(&self, inputs: &Vec<f32>) -> f32 {
        let mut output = 0.0;
        for (input, weight) in zip(inputs, &self.weights) {
            output += input * weight + self.bias;
        }
        tanh(output)
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
