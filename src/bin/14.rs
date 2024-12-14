advent_of_code::solution!(14);

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

const Q_WIDTH: isize = WIDTH / 2;
const Q_HEIGHT: isize = HEIGHT / 2;

struct ToiletGuard {
    x: isize,
    y: isize,
    v: (isize, isize),
}

impl ToiletGuard {
    #[inline(always)]
    fn tick(&mut self, ticks: isize) {
        self.x = (self.x + self.v.0 * ticks) % WIDTH;
        self.y = (self.y + self.v.1 * ticks) % HEIGHT;

        if self.x < 0 {
            self.x = WIDTH + self.x;
        }

        if self.y < 0 {
            self.y = HEIGHT + self.y;
        }
    }
}

impl From<std::str::Bytes<'_>> for ToiletGuard {
    /// a line looks like this:
    ///
    /// p=X[X],Y[Y] v=[-]X[X],[-]Y[Y]
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
            y += isize::from(char - b'0') * 10_isize.pow(oom);
            oom += 1;
        }

        oom = 0;

        loop {
            char = input.next_back().unwrap();
            if char == b'=' {
                break;
            }
            x += isize::from(char - b'0') * 10_isize.pow(oom);
            oom += 1;
        }

        Self { x, y, v }
    }
}

struct Quadrants([usize; 4]);

impl FromIterator<(isize, isize)> for Quadrants {
    fn from_iter<T: IntoIterator<Item = (isize, isize)>>(iter: T) -> Self {
        let mut quadrants: Quadrants = Quadrants([0; 4]);
        for (x, y) in iter {
            if x > Q_WIDTH {
                if y > Q_HEIGHT {
                    quadrants.0[0] += 1;
                } else if y < Q_HEIGHT {
                    quadrants.0[1] += 1;
                }
            } else if x < Q_WIDTH {
                if y > Q_HEIGHT {
                    quadrants.0[2] += 1;
                } else if y < Q_HEIGHT {
                    quadrants.0[3] += 1;
                }
            }
        }
        return quadrants;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(str::bytes)
            .map(ToiletGuard::from)
            .map(|mut e| {
                e.tick(100);
                (e.x, e.y)
            })
            .collect::<Quadrants>()
            .0
            .iter()
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut guards: Vec<ToiletGuard> = input
        .lines()
        .map(str::bytes)
        .map(ToiletGuard::from)
        .collect();

    let mut seconds_ellapsed = 0;

    'main_loop: loop {
        seconds_ellapsed += 1;

        let mut area_outside_bathroom: [[bool; WIDTH as usize]; HEIGHT as usize] =
            [[false; WIDTH as usize]; HEIGHT as usize];

        guards.iter_mut().for_each(|guard| {
            guard.tick(1);
            area_outside_bathroom[guard.y as usize][guard.x as usize] = true;
        });

        // check if the area contains a 3x3 spot that is completely filled.
        // If it does, we assume it to be a christmas tree.
        for y in 0..(HEIGHT - 2) {
            'row_loop: for x in 0..(WIDTH - 2) {
                for i in 0..3 {
                    for j in 0..3 {
                        if !area_outside_bathroom[y as usize + i][x as usize + j] {
                            continue 'row_loop;
                        }
                    }
                }
                break 'main_loop;
            }
        }
    }

    Some(seconds_ellapsed)
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
