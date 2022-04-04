extern crate sierpinski;
extern crate rand;
extern crate clap;

use clap::Parser;
use rand::prelude::SliceRandom;
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use sierpinski::{PPM, Color, Vec2D};

const SPACING: i32 = 10;
const WHITE: Color = (255, 255, 255);
const BLACK: Color = (0, 0, 0);

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, default_value_t = 10000)]
    n: i32,

    #[clap(long)]
    animate: bool,
}

fn main() {
    let args = Args::parse();
    let mut sierpinski = Sierpinski::new(600, args.n, args.animate);
    sierpinski.iterate();

    if !args.animate {
        sierpinski.print_image();
    }
}

struct Sierpinski {
    image: PPM,
    rnd: ThreadRng,
    n: i32,
    corners: Vec<Vec2D>,

    animate: bool,
}

impl Sierpinski {
    pub fn new (side: i32, n: i32, animate: bool) -> Self {
        let mut image = PPM::new(side, side, WHITE);

        let h = side - SPACING * 2;

        let corners = vec![
            Vec2D(side / 2, SPACING),
            Vec2D(SPACING, h + SPACING),
            Vec2D(h + SPACING, h + SPACING),
        ];

        for corner in corners.iter() {
            image.set(corner, BLACK);
        }

        Self {
            image,
            rnd: thread_rng(),
            n,
            corners,
            animate,
        }
    }

    pub fn iterate(&mut self) {
        let mut v = self.random_point_in_triangle();
        self.image.set(&v, BLACK);

        for _ in 0..self.n {
            let corner = self.random_corner();
            v = corner.midpoint(&v);
            self.image.set(&v, BLACK);

            if self.animate {
                self.print_image();
            }
        }
    }

    fn random_corner(&mut self) -> &Vec2D {
        self.corners.choose(&mut self.rnd).unwrap()
    }

    // Generating random points in a triangle: https://blogs.sas.com/content/iml/2020/10/19/random-points-in-triangle.html
    fn random_point_in_triangle(&mut self) -> Vec2D {
        let a = self.corners[1] - self.corners[0];
        let b = self.corners[2] - self.corners[0];
        let mut u1: f64 = self.rnd.gen();
        let mut u2: f64 = self.rnd.gen();

        if u1 + u2 > 1.0 {
            u1 = 1.0 - u1;
            u2 = 1.0 - u2;
        }

        let w = u1 * a + u2 * b;
        w + self.corners[0]
    }

    pub fn print_image(&self) {
        self.image.print();
    }
}
