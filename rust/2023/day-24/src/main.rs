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
    fn intersect_2d_cramer(&self, hail_2: &Hail) -> Option<(f64, f64)> {
        /*
            Approach A) Simple intersection using matrix algebra and Cramer's rule

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

    fn intersect_2d_line_segments(&self, hail_2: &Hail) -> Option<(f64, f64)> {
        /*
           Approach B) Using numerical analysis to find the intersection point of two segments

           This approach uses the algorithm proposed by Paul Bourke (see: https://paulbourke.net/geometry/pointlineplane/)
           Here, instead of solving for the infinite line, only analyze the intersection point between two segments

           The segments are calculated as: starting point + ending point; where the ending point is the position
           of the Hail after a n number of nanoseconds passed.
           The number of nanoseconds must be found by trial-and-error, since the problem doesn't state how much time passed.
           With a value of 10, the solution works both for the sample input and my full input.
           Higher values of n overflows the calculation of the ua parameter.

           Algorithm:

           Given two segments defined as (where P1..Pn are the x,y points coordinates):
           Pa = P1 + ua ( P2 - P1 )
           Pb = P3 + ub ( P4 - P3 )

           Solving both for Pa = Pb (the intersection point):

           x1 + ua (x2 - x1) = x3 + ub (x4 - x3)
           y1 + ua (y2 - y1) = y3 + ub (y4 - y3)

           Then, solve for either ua or ub, and input the resulting value in:

           x = x1 + ua (x2 - x1)
           y = y1 + ua (y2 - y1)
        */

        let elapsed_nanoseconds = 10;

        let x1 = self.x;
        let x2 = self.x + self.vx * elapsed_nanoseconds;

        let x3 = hail_2.x;
        let x4 = hail_2.x + hail_2.vx * elapsed_nanoseconds;

        let y1 = self.y;
        let y2 = self.y + self.vy * elapsed_nanoseconds;

        let y3 = hail_2.y;
        let y4 = hail_2.y + hail_2.vy * elapsed_nanoseconds;

        let denominator = ((y4 - y3) * (x2 - x1)) - ((x4 - x3) * (y2 - y1));

        if denominator == 0 {
            return None;
        }

        let ua: f64 =
            (((x4 - x3) * (y1 - y3)) - ((y4 - y3) * (x1 - x3))) as f64 / denominator as f64;

        let x = x1 as f64 + (ua * (x2 - x1) as f64);
        let y = y1 as f64 + (ua * (y2 - y1) as f64);

        if ((x - self.x as f64).signum() != self.vx.signum() as f64)
            || ((x - hail_2.x as f64).signum() != hail_2.vx.signum() as f64)
        {
            return None; // Collision happened in the past
        }

        Some((x, y))
    }
}

fn solve_part1(
    input: &str,
    test_area_start: u64,
    test_area_end: u64,
    intersect_method: fn(&Hail, &Hail) -> Option<(f64, f64)>,
) -> u16 {
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
        .map(
            |x| match intersect_method(x.first().unwrap(), x.last().unwrap()) {
                Some((x, y)) if test_range.contains(&x) && test_range.contains(&y) => 1,
                _ => 0,
            },
        )
        .sum()
}

fn main() {
    println!(
        "Part 1 with approach A: {}",
        solve_part1(
            "input.txt",
            200_000_000_000_000,
            400_000_000_000_000,
            Hail::intersect_2d_cramer
        )
    );
    println!(
        "Part 1 with approach B: {}",
        solve_part1(
            "input.txt",
            200_000_000_000_000,
            400_000_000_000_000,
            Hail::intersect_2d_line_segments
        )
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_demo_input_for_part_1() {
        assert_eq!(
            2,
            solve_part1("demo-input.txt", 7, 27, Hail::intersect_2d_cramer)
        );
    }

    #[test]
    fn test_demo_input_for_part_1_with_line_segments() {
        assert_eq!(
            2,
            solve_part1("demo-input.txt", 7, 27, Hail::intersect_2d_line_segments)
        );
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            27328,
            solve_part1(
                "input.txt",
                200_000_000_000_000,
                400_000_000_000_000,
                Hail::intersect_2d_cramer
            )
        );

        assert_eq!(
            27328,
            solve_part1(
                "input.txt",
                200_000_000_000_000,
                400_000_000_000_000,
                Hail::intersect_2d_line_segments
            )
        );
    }
}
