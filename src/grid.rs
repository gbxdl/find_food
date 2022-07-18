use rand::Rng;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub ternary: Vec<i8>,
}

impl Grid {
    pub fn new(width: usize, height: usize, prob_prey: f32, prob_food: f32) -> Grid {
        let mut grid = Grid {
            width,
            height,
            ternary: vec![0; width * height],
        };
        if prob_prey > 0.0 || prob_food > 0.0 {
            grid.fill_random_grid(prob_prey, prob_food);
        }
        grid
    }

    fn fill_random_grid(&mut self, prob_prey: f32, prob_food: f32) {
        for i in 0..self.width * self.height {
            let num = rand::thread_rng().gen::<f32>();
            if num < prob_prey {
                self.ternary[i] = -1;
            } else if num > 1.0 - prob_food {
                self.ternary[i] = 1;
            }
        }
    }

    pub fn show(&self) {
        for (i, val) in self.ternary.iter().enumerate() {
            if i % self.width == 0 {
                print!("\n");
            }
            match val {
                0 => print!(". "),
                1 => print!("X "),
                -1 => print!("O "),
                _ => print!("_ "),
            }
        }
        print!("\n");
    }
}
