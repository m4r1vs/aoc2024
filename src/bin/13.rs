advent_of_code::solution!(13);

#[derive(Debug)]
struct SystemOfLinearEquations {
    a: [isize; 2],
    b: [isize; 2],
    c: [isize; 2],
}

impl SystemOfLinearEquations {
    fn try_solve_cramer(&self) -> Option<(isize, isize)> {
        let det = self.a[0] * self.b[1] - self.b[0] * self.a[1];

        if ((self.c[0] * self.b[1] - self.b[0] * self.c[1]) % det)
            + ((self.a[0] * self.c[1] - self.c[0] * self.a[1]) % det)
            != 0
        {
            return None;
        }

        Some((
            ((self.c[0] * self.b[1] - self.b[0] * self.c[1]) / det),
            ((self.a[0] * self.c[1] - self.c[0] * self.a[1]) / det),
        ))
    }
}

impl From<Vec<u8>> for SystemOfLinearEquations {
    /// Input has following form always so I try to parse as efficiently
    /// as possible.. idk if this is massive overhead since we have to
    /// convert the Bytestream into a heap alloced vector first but it
    /// works now..
    ///
    /// Button A: X+26, Y+56
    /// Button B: X+43, Y+22
    /// Prize: X=6138, Y=6756
    ///
    fn from(input: Vec<u8>) -> Self {
        let mut c1 = 0;
        let mut c2 = 0;

        let mut i = 1;
        let mut e: u32 = 0;
        while input[input.len() - i] != b'=' {
            c2 += isize::from(input[input.len() - i] - b'0') * 10_isize.pow(e);
            (i, e) = (i + 1, e + 1);
        }

        e = 0;

        for _ in 0..4 {
            i += 1;
        }

        while input[input.len() - i] != b'=' {
            c1 += isize::from(input[input.len() - i] - b'0') * 10_isize.pow(e);
            (i, e) = (i + 1, e + 1);
        }

        Self {
            a: [
                isize::from((input[12] - b'0') * 10 + (input[13] - b'0')),
                isize::from((input[18] - b'0') * 10 + (input[19] - b'0')),
            ],
            b: [
                isize::from((input[33] - b'0') * 10 + (input[34] - b'0')),
                isize::from((input[39] - b'0') * 10 + (input[40] - b'0')),
            ],
            c: [c1, c2],
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .trim_end()
            .split("\n\n")
            .map(str::bytes)
            .map(Iterator::collect::<Vec<u8>>)
            .map(SystemOfLinearEquations::from)
            .filter_map(|e| e.try_solve_cramer())
            .map(|(a, b)| 3 * a as usize + b as usize)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .trim_end()
            .split("\n\n")
            .map(str::bytes)
            .map(Iterator::collect::<Vec<u8>>)
            .map(SystemOfLinearEquations::from)
            .map(|mut e| {
                e.c[0] += 10000000000000;
                e.c[1] += 10000000000000;
                e
            })
            .filter_map(|e| e.try_solve_cramer())
            .map(|(a, b)| 3 * a as usize + b as usize)
            .sum(),
    )
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
