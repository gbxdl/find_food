mod agents;
mod grid;

use agents::NeuralNet;
use agents::Prey;
use grid::Grid;

use rand::{thread_rng, Rng};

use std::clone::Clone;

pub fn fresh_start() -> SimulationState {
    let config = SimulationConfig {
        prob_prey: 0.05,
        prob_food: 0.1,
        grid_width: 32,
        grid_height: 32,
        max_energy_prey: 10,
        max_energy_predator: 10,
        view_distance_prey: 3,
        view_distance_predator: 3,
        max_split_count_prey: 2,
        max_split_count_predator: 2,
        energy_gain_predator: 10,
    };
    let grid = Grid::new(
        config.grid_width,
        config.grid_height,
        config.prob_prey,
        config.prob_food,
    );
    let mut prey_list: Vec<Prey> = vec![];

    for (position, value) in grid.ternary.iter().enumerate() {
        if *value == -1 {
            prey_list.push(Prey::new(
                position,
                config.view_distance_prey,
                config.max_energy_prey,
                NeuralNet { layers: vec![] },
            ));
        }
    }
    SimulationState {
        config,
        grid,
        prey_list,
        running: false,
    }
}

pub struct SimulationConfig {
    prob_prey: f32,
    prob_food: f32,
    pub grid_width: usize,
    pub grid_height: usize,
    max_energy_prey: usize,
    max_energy_predator: usize,
    view_distance_prey: usize,
    view_distance_predator: usize,
    max_split_count_prey: usize,
    max_split_count_predator: usize,
    energy_gain_predator: usize,
}

pub struct SimulationState {
    pub config: SimulationConfig,
    pub grid: Grid,
    prey_list: Vec<Prey>,
    pub running: bool,
}

impl SimulationState {
    pub fn update_grid(&mut self) {
        let mut grid = Grid::new(self.config.grid_width, self.config.grid_height, 0.0, 0.0);

        // keep food around
        for pos in 0..grid.ternary.len() {
            if self.grid.ternary[pos] == 1 {
                grid.ternary[pos] = 1;
            }
        }

        let mut remove_indices_prey = vec![];
        let mut add_positions_nn_prey = vec![];

        for (i, prey) in self.prey_list.iter_mut().enumerate() {
            if prey.split_count == self.config.max_split_count_prey {
                add_positions_nn_prey.push((prey.prev_position, Clone::clone(&prey.neural_net)));
                prey.split_count = 0;
            }
            if grid.ternary[prey.position] == 1 {
                prey.split_count += 1;
                prey.energy += self.config.max_energy_prey;
            }
            grid.ternary[prey.position] = -1;
            if prey.energy == 0 {
                remove_indices_prey.push(i);
            }
        }

        // delete prey
        for i in remove_indices_prey.iter().rev() {
            self.prey_list.remove(*i);
        }
        // add prey babies.
        for (pos, nn) in add_positions_nn_prey {
            if grid.ternary[pos] == 0 {
                self.prey_list.push(Prey::new(
                    pos,
                    self.config.view_distance_prey,
                    self.config.max_energy_prey,
                    nn,
                ));
                grid.ternary[pos] = -1;
            }
        }

        for pos in 0..grid.ternary.len() {
            let prob_food_at_pos = 1.0 / grid.ternary.len() as f32;
            if grid.ternary[pos] == 0 {
                let num = rand::thread_rng().gen::<f32>();

                if num < prob_food_at_pos {
                    grid.ternary[pos] = 1;
                }
            }
        }

        self.grid = grid;
    }
}

pub fn take_step(state: &mut SimulationState) {
    for prey in &mut *state.prey_list {
        prey.take_step(&state.grid);
    }
    state.update_grid();
}
