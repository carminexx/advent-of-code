use anyhow::{Context, Error, Result};
use itertools::{self, Itertools};
use std::{fs, str::FromStr};

#[derive(Copy, Clone, Debug)]
struct Hail {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl FromStr for Hail {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Example input "19, 13, 30 @ -2,  1, -2"
        //                x   y   z    vx  vy  vz

        let (pos, vel) = s
            .splitn(2, " @ ")
            .collect_tuple()
            .context("failed to parse around '@'")?;

        let (x, y, z) = pos
            .split(',')
            .map(str::trim)
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .context("failed to parse position values")?;
        let (vx, vy, vz) = vel
            .split(',')
            .map(str::trim)
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .context("failed to parse velocity values")?;

        Ok(Hail {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        })
    }
}

impl Hail {
    fn intersect(&self, hail_2: &Hail) -> Option<(f64, f64)> {
        /*
            Simple intersection using matrix algebra and Cramer's rule

            For example, considering the first Hail:
            19, 13, 30 @ -2,  1, -2

            (x:19, y:13; speed x: -1, speed y: 1)

            We already have the direction of the hail as an explicit line:

            y = mx + q
            => (1)y = (-1)x + (0)q

            Then we need to make the "starting" position of the hail pass through
            the same line.

            Since q (the intercept) is 0, the gradient (m) can be calculated by
            dividing the v_y by v_x:

            m = 1 / -1 = -1

            We rewrite the line in its implicit form:

            a1 x + b1 y = c1

            a1 is the gradient with the opposed sign (since y = mx + q => -mx +y = q)
            b1 is always 1 since we are "solving" for the x variable

            c1, the intercept, must be calculated in order to have the hail starting point
            passing through the line:

            Since y = mx + q, q = y - mx:

            c1 = y - (gradient * x)

        */

        let gradient_1: f64 = self.vy as f64 / self.vx as f64;
        let gradient_2: f64 = hail_2.vy as f64 / hail_2.vx as f64;

        let a1 = -gradient_1;
        let a2 = -gradient_2;
        let b1 = 1.0;
        let b2 = 1.0;

        let c1 = self.y as f64 - (gradient_1 * self.x as f64);
        let c2 = hail_2.y as f64 - (gradient_2 * hail_2.x as f64);

        /*
            Now we just rewrite the simple system of two lines in a matrix and apply the Cramer's rule:

            [a1 b1] [x] = [c1]
            [a2 b2] [y]   [c2]

            Obviously, if the determinant is 0 the hails never collide (lines are parallel) and we skip them.
        */

        let determinant = (a1 * b2) - (b1 * a2);
        if !determinant.is_normal() {
            return None;
        }

        let x: f64 = ((c1 * b2) - (b1 * c2)) / determinant;
        let y: f64 = ((a1 * c2) - (c1 * a2)) / determinant;

        /*
            Lastly, we check whether the collision happened in the past
            by analyzing the signum of the found point in comparison
            of the starting point and the speed direction

            [shorthand for:
                if (self.vx > 0 && x < self.x)
                || (self.vx < 0 && x > self.x)
                || (hail_2.vx > 0 && x < hail_2.x)
                || (hail_2.vx < 0 && x > hail_2.x)
            ]
        */

        if ((x - self.x as f64).signum() != self.vx.signum() as f64)
            || ((x - hail_2.x as f64).signum() != hail_2.vx.signum() as f64)
        {
            return None; // Collision happened in the past
        }

        Some((x, y))
    }
}

fn solve_part1(input: &str, test_area_start: u64, test_area_end: u64) -> u16 {
    let raw = fs::read_to_string(input).unwrap();
    let hails: Vec<Hail> = raw
        .lines()
        .map(|line| {
            line.parse::<Hail>()
                .context("failed to parse hail values")
                .unwrap()
        })
        .collect();

    let test_range = test_area_start as f64..=test_area_end as f64;

    hails
        .iter()
        .combinations(2)
        .map(|x| match x.first().unwrap().intersect(x.last().unwrap()) {
            Some((x, y)) if test_range.contains(&x) && test_range.contains(&y) => 1,
            _ => 0,
        })
        .sum()
}

fn main() {
    println!(
        "Part 1: {}",
        solve_part1("input.txt", 200_000_000_000_000, 400_000_000_000_000)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_demo_input_for_part_1() {
        assert_eq!(2, solve_part1("demo-input.txt", 7, 27));
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            27328,
            solve_part1("input.txt", 200_000_000_000_000, 400_000_000_000_000)
        );
    }
}
