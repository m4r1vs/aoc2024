use std::cmp::Ordering::*;

advent_of_code::solution!(14);

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

const Q_WIDTH: usize = WIDTH / 2;
const Q_HEIGHT: usize = HEIGHT / 2;

/// A robot guarding the bathroom on Easter Bunny Island.
/// Suspiciously similar to the once at the North Pole.
struct BathroomGuardingRobot {
    /// Current position on x-axis
    x: usize,
    /// Current position on y-axis
    y: usize,
    /// Velocity (x, y)
    v: (isize, isize),
}

impl BathroomGuardingRobot {
    /// Wraps a position value to ensure it stays within the bounds [0, limit).
    #[inline(always)]
    fn _wrap(value: isize, limit: usize) -> usize {
        ((value % limit as isize + limit as isize) % limit as isize) as usize
    }

    #[inline(always)]
    fn n_ticks(&mut self, ticks: usize) {
        self.x = Self::_wrap(self.x as isize + self.v.0 * ticks as isize, WIDTH);
        self.y = Self::_wrap(self.y as isize + self.v.1 * ticks as isize, HEIGHT);
    }

    #[inline(always)]
    fn tick_x_once(&mut self) {
        self.x = Self::_wrap(self.x as isize + self.v.0, WIDTH);
    }

    #[inline(always)]
    fn tick_y_once(&mut self) {
        self.y = Self::_wrap(self.y as isize + self.v.1, HEIGHT);
    }
}

/// Stores the amount of robots in each quadrant.
struct Quadrants([usize; 4]);

impl FromIterator<(usize, usize)> for Quadrants {
    fn from_iter<T: IntoIterator<Item = (usize, usize)>>(iter: T) -> Self {
        let mut quadrants: Quadrants = Quadrants([0; 4]);
        for (x, y) in iter {
            match x.cmp(&Q_WIDTH) {
                Greater => match y.cmp(&Q_HEIGHT) {
                    Greater => {
                        quadrants.0[0] += 1;
                    }
                    Less => {
                        quadrants.0[1] += 1;
                    }
                    Equal => (),
                },
                Less => match y.cmp(&Q_HEIGHT) {
                    Greater => {
                        quadrants.0[2] += 1;
                    }
                    Less => {
                        quadrants.0[3] += 1;
                    }
                    Equal => (),
                },
                Equal => (),
            }
        }
        quadrants
    }
}

/// Just simulate every guard once by taking the velocity vector times 100
pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(str::bytes)
            .map(BathroomGuardingRobot::from)
            .map(|mut guard| {
                guard.n_ticks(100);
                (guard.x, guard.y)
            })
            .collect::<Quadrants>()
            .0
            .iter()
            .product(),
    )
}

/// Solve using Chinese Remainder Theorem
/// (See https://www.reddit.com/r/adventofcode/comments/1he0asr/2024_day_14_part_2_why_have_fun_with_image/)
///
/// Basically, we simulate the x coordinates WIDTH times and y coordinates HEIGHT times.
/// For each step, we calculate the average distance from the center. We also keep track
/// of the tick at which the average distance has been the smallest.
///
/// To fasten it up a little more, we only load and simulate every 8th robot.
/// Starts to get flaky at every 11th and beyond...
///
/// After completing both loops, we now know, at which tick the center-distance was
/// at a minimum (for x and y each). We can use the CRT to calculate the position in time,
/// at which both the x-distance and y-distance was the smallest on average.
///
/// That spot in time is the spot in time where we find our Christmas Tree.
pub fn part_two(input: &str) -> Option<usize> {
    let mut current_min_x = usize::MAX;
    let mut current_min_y = usize::MAX;
    let mut min_tick_x = 0;
    let mut min_tick_y = 0;

    let mut guards: Vec<BathroomGuardingRobot> = input
        .lines()
        .step_by(8)
        .map(str::bytes)
        .map(BathroomGuardingRobot::from)
        .collect();

    for tick in 0..WIDTH {
        let mut avg_d_from_ctr = 0;

        for guard in guards.iter_mut() {
            guard.tick_x_once();
            avg_d_from_ctr += (guard.x as isize - Q_WIDTH as isize).pow(2) as usize;
        }

        if avg_d_from_ctr < current_min_x {
            current_min_x = avg_d_from_ctr;
            min_tick_x = tick + 1;
        }
    }

    for tick in 0..HEIGHT {
        let mut avg_d_from_ctr = 0;

        for guard in guards.iter_mut() {
            guard.tick_y_once();
            avg_d_from_ctr += (guard.y as isize - Q_HEIGHT as isize).pow(2) as usize;
        }

        if avg_d_from_ctr < current_min_y {
            current_min_y = avg_d_from_ctr;
            min_tick_y = tick + 1;
        }
    }

    let result = min_tick_x
        + ((modular_inverse(WIDTH, HEIGHT) * (min_tick_y - min_tick_x)) % HEIGHT) * WIDTH;

    // print_robots(input, result);

    Some(result)
}

/// Extended Euclidean Algorithm to find modular inverse:
///
/// ```
/// ax ≡ 1 (mod m) // given a and m, return x
/// ```
fn modular_inverse(a: usize, m: usize) -> usize {
    let mut m0 = m;
    let mut x0 = 0;
    let mut x1 = 1;
    let mut a = a;

    while a > 1 {
        let q = a / m0;
        let t = m0;

        m0 = a % m0;
        a = t;

        let t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    x1
}

#[allow(dead_code)]
/// Print the position of robots at step n to stdout.
/// Used to display the Christmas Tree.
fn print_robots(input: &str, n: usize) {
    let mut area_outside_bathroom: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

    input
        .lines()
        .map(str::bytes)
        .map(BathroomGuardingRobot::from)
        .for_each(|mut e| {
            e.n_ticks(n);
            area_outside_bathroom[e.y][e.x] = true;
        });

    println!();

    for row in area_outside_bathroom {
        for piece in row {
            print!("{}", if piece { "#" } else { " " });
        }
        println!();
    }
}

impl From<std::str::Bytes<'_>> for BathroomGuardingRobot {
    /// Dirty but fast. A line looks like this:
    ///
    /// ```
    /// p=X[X],Y[Y] v=[-]X[X],[-]Y[Y]
    /// ```
    fn from(mut input: std::str::Bytes) -> Self {
        let mut char;
        let mut oom = 0;
        let mut v = (0, 0);

        loop {
            char = input.next_back().unwrap();
            if char == b'-' {
                v.1 = -v.1;
                input.next_back();
                break;
            }
            if char == b',' {
                break;
            }
            v.1 += isize::from(char - b'0') * 10_isize.pow(oom);
            oom += 1;
        }

        oom = 0;

        loop {
            char = input.next_back().unwrap();
            if char == b'-' {
                v.0 = -v.0;
                input.next_back();
                break;
            }
            if char == b'=' {
                break;
            }
            v.0 += isize::from(char - b'0') * 10_isize.pow(oom);
            oom += 1;
        }

        input.next_back();
        input.next_back();

        oom = 0;
        let mut x = 0;
        let mut y = 0;

        loop {
            char = input.next_back().unwrap();
            if char == b',' {
                break;
            }
            y += usize::from(char - b'0') * 10_usize.pow(oom);
            oom += 1;
        }

        oom = 0;

        loop {
            char = input.next_back().unwrap();
            if char == b'=' {
                break;
            }
            x += usize::from(char - b'0') * 10_usize.pow(oom);
            oom += 1;
        }

        Self { x, y, v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
