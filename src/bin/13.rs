use core::str;

advent_of_code::solution!(13);

#[derive(Debug)]
struct SystemOfLinearEquations {
    a: [isize; 2],
    b: [isize; 2],
    c: [isize; 2],
}

impl SystemOfLinearEquations {
    /// See https://en.wikipedia.org/wiki/Cramer%27s_rule#Applications
    /// for a reference.
    /// Takes about 2µs for the 320 equations in the input.
    fn try_solve_cramer(&self) -> Option<(isize, isize)> {
        let det = self.a[0] * self.b[1] - self.b[0] * self.a[1];

        // Using addition and one NEQ check is faster for part two only.
        // Alternative is NEQ || NEQ
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

    #[allow(dead_code)]
    /// Solve the system using Gaussian elimination.
    fn try_solve_gauss_elimination(&self) -> Option<(isize, isize)> {
        // TODO: Implement Gaussian Elimination, it's not working yet!

        // Check if the system is solvable
        let det = self.a[0] * self.b[1] - self.b[0] * self.a[1];

        if det == 0 {
            return None;
        }

        // Create augmented matrix
        let mut matrix = [
            [self.a[0], self.a[1], self.c[0]],
            [self.b[0], self.b[1], self.c[1]],
        ];

        // Eliminate first variable
        let factor = matrix[1][0] / matrix[0][0];

        matrix[1][0] = 0;
        matrix[1][1] -= factor * matrix[0][1];
        matrix[1][2] -= factor * matrix[0][2];

        // Check if the system has a unique solution
        if matrix[1][1] == 0 {
            return None;
        }

        // Back-substitution
        if matrix[1][2] % matrix[1][1] != 0 {
            return None;
        }
        let y = matrix[1][2] / matrix[1][1];

        let x_numerator = matrix[0][2] - matrix[0][1] * y;
        if x_numerator % matrix[0][0] != 0 {
            return None;
        }
        let x = x_numerator / matrix[0][0];

        Some((x, y))
    }
}

impl From<std::str::Bytes<'_>> for SystemOfLinearEquations {
    /// Input has always the following form so I try to parse as efficiently
    /// as possible.. Results in 14µs runtime for the part_one() and part_two().
    /// A C program that uses `sscanf` takes about 80µs for computing the same
    /// data (clang -Ofast). Using the `From<&str>` implementation below is faster than
    /// sscanf but still slower than iterating over the bytestream.
    ///
    /// There doesn't seem to be an equivalent to sscanf in rust without
    /// macros from crates. However, another straighforward implementation
    /// can be seen commented out below. It takes the time of computation
    /// to about 55µs but tolerates other formats of input.
    ///
    /// See this illustration for a reference for the `n` in `nth(n)`
    ///
    /// ```
    ///               111
    ///     0123456789012N01234N 0
    ///     Button A: X+26, Y+56\n   // a[0] and a[1]
    ///
    ///              1111
    ///     1234567890123N01234N
    ///     Button B: X+43, Y+22\n   // b[0] and b[1]
    ///
    ///                  BBBL
    ///     Prize: X=6138, Y=6756    // c1 and c2, respectively
    /// ```
    fn from(mut input: std::str::Bytes) -> Self {
        let mut c1 = 0;
        let mut c2 = 0;

        let mut oom: u32 = 0; // order of magnitude

        // First start consuming the bytes from the back
        // to parse the last two numbers which have a variable
        // order of magnitude.

        // C2
        loop {
            let char = input.next_back().unwrap();

            if char == b'=' {
                break;
            }

            c2 += isize::from(char - b'0') * 10_isize.pow(oom);
            oom += 1;
        }

        oom = 0;

        // See 'B' in the illustration above
        input.next_back();
        input.next_back();
        input.next_back();

        // C1
        loop {
            let char = input.next_back().unwrap();

            if char == b'=' {
                break;
            }

            c1 += isize::from(char - b'0') * 10_isize.pow(oom);
            oom += 1;
        }

        // Now parse the first 4 numbers by consuming the Iterator from its front:
        Self {
            a: [
                isize::from((input.nth(12).unwrap() - b'0') * 10 + (input.next().unwrap() - b'0')),
                isize::from((input.nth(4).unwrap() - b'0') * 10 + (input.next().unwrap() - b'0')),
            ],
            b: [
                isize::from((input.nth(13).unwrap() - b'0') * 10 + (input.next().unwrap() - b'0')),
                isize::from((input.nth(4).unwrap() - b'0') * 10 + (input.next().unwrap() - b'0')),
            ],
            c: [c1, c2],
        }
    }
}

impl From<&str> for SystemOfLinearEquations {
    /// Much cleaner source code but makes the whole program ~25% slower (19µs vs 13µs).
    /// Using unwrap_unchecked has no impact on performance.
    fn from(input: &str) -> Self {
        let (c1, c2) = input.get(51..input.len()).unwrap().split_once(',').unwrap();

        Self {
            a: [
                input.get(12..=13).map(str::parse).unwrap().unwrap(),
                input.get(18..=19).map(str::parse).unwrap().unwrap(),
            ],
            b: [
                input.get(33..=34).map(str::parse).unwrap().unwrap(),
                input.get(39..=40).map(str::parse).unwrap().unwrap(),
            ],
            c: [
                c1.parse().unwrap(),
                c2.get(3..c2.len()).map(str::parse).unwrap().unwrap(),
            ],
        }
    }
}

// impl From<&str> for SystemOfLinearEquations {
//     fn from(input: &str) -> Self {
//         let mut numbers: Vec<isize> = Vec::new();

//         for word in input.split(|c: char| !c.is_numeric() && c != '-' && c != '+') {
//             if !word.is_empty() {
//                 if let Ok(num) = word.parse::<isize>() {
//                     numbers.push(num);
//                 }
//             }
//         }

//         Self {
//             a: [numbers[0], numbers[1]],
//             b: [numbers[2], numbers[3]],
//             c: [numbers[4], numbers[5]],
//         }
//     }
// }

pub fn part_one(input: &str) -> Option<isize> {
    Some(
        input
            .trim_end()
            .split("\n\n")
            .map(str::bytes)
            .map(SystemOfLinearEquations::from)
            .filter_map(|e| e.try_solve_cramer())
            .map(|(a, b)| 3 * a + b)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(
        input
            .trim_end()
            .split("\n\n")
            .map(str::bytes)
            .map(SystemOfLinearEquations::from)
            .map(|mut e| {
                e.c[0] += 10000000000000;
                e.c[1] += 10000000000000;
                e
            })
            .filter_map(|e| e.try_solve_cramer())
            .map(|(a, b)| 3 * a + b)
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
