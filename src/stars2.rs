use rand::{thread_rng,Rng};

/*
My first attempt at data oriented design.
*/

pub struct Stars {
    pub stars: Vec<(f32, f32, f32)>,
    // colors: Vec>
    width: f32,
    height: f32,
}

impl Stars {
    pub fn new(number_of_stars: usize, width: f32, height: f32) -> Self {
        let mut rng = thread_rng();
        let mut stars: Vec<(f32,f32,f32)> = Vec::new();

        for _i in 0..number_of_stars {
            stars.push(
                (
                    // rng.gen_range(-width..width),
                    // rng.gen_range(-height..height),
                    // rng.gen_range(0.2..100.0)
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
            )
        };
        
        Stars {
            stars,
            width,
            height,
        }
    }

    pub fn add_star(&mut self, amount:usize) {
        let mut rng = thread_rng();

        for _ in 0..amount{
            self.stars.push(
                (
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )            
            )
        }
    }
    pub fn delete_star(&mut self, amount:usize) {
        for _ in 0..amount{
            self.stars.pop();
        };
    }

    pub fn step(&mut self, dx: f32, dy: f32) {
        for (x, y, z) in self.stars.iter_mut() 
        {
            *x += dx / (*z * 5.0).abs();

            if *x > 1.0 {*x = -1.0} else if *x < -1.0 {*x = 1.0};

            *y += dy / (*z * 5.0).abs();
            
            if *y > 1.0 {*y = -1.0} else if *y < -1.0 {*y = 1.0};
        }
    }
}