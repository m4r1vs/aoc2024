advent_of_code::solution!(12);

struct Garden {
    plants: Vec<Vec<u8>>,
}

impl Garden {
    fn get_mut_ptr(&mut self, x: usize, y: usize) -> Option<*mut u8> {
        self.plants
            .get_mut(y)
            .and_then(|row| row.get_mut(x).map(|b| b as *mut u8))
    }
}

impl From<&str> for Garden {
    fn from(input: &str) -> Self {
        Self {
            plants: input
                .lines()
                .map(str::bytes)
                .map(Iterator::collect)
                .collect(),
        }
    }
}

unsafe fn push_new_plants(
    plant: *mut u8,
    plant_x: usize,
    plant_y: usize,
    garden: &mut Garden,
    stack: &mut Vec<(*mut u8, (usize, usize))>,
    inner_stack: &mut Vec<(*mut u8, (usize, usize))>,
) -> usize {
    let mut fencing_needed = 0;
    for (x, y) in [(1, 0), (0, 1), (-1, 0), (0, -1)].map(|(dx, dy)| {
        (
            plant_x.wrapping_add_signed(dx),
            plant_y.wrapping_add_signed(dy),
        )
    }) {
        match garden.get_mut_ptr(x, y) {
            Some(new_plant) => {
                // found plant already processed
                if *new_plant > b'Z' {
                    // plant already processed and value was different to current
                    if *new_plant - b'Z' != *plant {
                        fencing_needed += 1;
                    }
                    continue;
                }
                if *new_plant != *plant {
                    // found plant other than the current one, needs to be processed in other
                    // iteration
                    fencing_needed += 1;
                    stack.push((new_plant, (x, y)));
                    continue;
                }
                if new_plant != plant {
                    // found plant same as current one
                    // but not the one we started at, needs to be processed in current iteration
                    inner_stack.push((new_plant, (x, y)));
                    continue;
                }
            }
            None => {
                fencing_needed += 1;
            }
        }
    }
    fencing_needed
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;

    unsafe {
        let mut garden = Garden::from(input);

        let mut stack: Vec<(*mut u8, (usize, usize))> = Vec::with_capacity(100);
        let mut inner_stack: Vec<(*mut u8, (usize, usize))> = Vec::with_capacity(200);
        let mut plant_count = 1;

        stack.push((garden.get_mut_ptr(0, 0).unwrap_unchecked(), (0, 0)));

        while let Some((plant, (mut x, mut y))) = stack.pop() {
            if *plant > b'Z' {
                // println!("Plant is already processed. Continue.");
                continue;
            }

            // println!("Started processing {} at {}:{}", *plant as char, x, y);

            let mut fencing_needed =
                push_new_plants(plant, x, y, &mut garden, &mut stack, &mut inner_stack);

            while let Some((new_plant, (newx, newy))) = inner_stack.pop() {
                if *new_plant > b'Z' {
                    continue;
                }

                x = newx;
                y = newy;

                // println!(
                //     "New Plant is {} at {}:{}. Fencing needed: {}",
                //     *new_plant as char, x, y, fencing_needed
                // );

                plant_count += 1;

                *new_plant += b'Z';

                fencing_needed +=
                    push_new_plants(plant, x, y, &mut garden, &mut stack, &mut inner_stack);
            }

            // println!("Found {} occurances of {}", plant_count, *plant as char);
            // println!("Fencing: {}", fencing_needed);

            sum += fencing_needed * plant_count;

            *plant = 181; // b'Z' * 2 + 1
            plant_count = 1;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.len())
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
