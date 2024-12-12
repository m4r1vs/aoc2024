use advent_of_code::{wradd, Grid};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        calc_fences_and_corners(
            &mut Garden {
                plants: Grid::from(input),
            },
            true,
        )
        .fences,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        calc_fences_and_corners(
            &mut Garden {
                plants: Grid::from(input),
            },
            false,
        )
        .corners,
    )
}

struct Garden {
    plants: Grid<u8>,
}

/// This is wrapping a custom Grid struct (see lib.rs) with helper functions to mark plants
/// as visited whilst retaining their values.
///
/// It's a little faster to operate on a single grid than having a second one for storing visited
/// locations. We just do += b'Z' which is at most 180 and can happen only once max.
impl Garden {
    /// Return true if the value of the given plant matches the plant at the given coordinates.
    ///
    /// It will ignore any marks that might have been set on the internal plant.
    /// We're assuming that the given plant is either unmarked or the plant we're
    /// comparing it to is also marked.
    fn are_plants_equal(&self, x: usize, y: usize, foreign_plant: u8) -> bool {
        if let Some(own_plant) = self.plants.get(x, y) {
            if *own_plant == foreign_plant {
                return true;
            }

            // safe to not check > b'Z' since it wraps around to a value > 180 anyway
            if *own_plant - b'Z' == foreign_plant {
                return true;
            }
        }
        false
    }

    /// Panics if out of range!
    /// Return true if the plant has been marked visited.
    fn is_marked(&self, x: usize, y: usize) -> bool {
        self.plants.get(x, y).map(|p| *p > b'Z').unwrap()
    }

    /// Mark a plant at given coordinates as visited.
    /// Make sure to only call it once per plant or else the original value is lost!
    fn mark(&mut self, x: usize, y: usize) {
        if let Some(p) = self.plants.get_mut(x, y) {
            *p += b'Z';
        }
    }
}

struct FencesAndCorners {
    fences: usize,
    corners: usize,
}

fn calc_fences_and_corners(garden: &mut Garden, skip_the_corners: bool) -> FencesAndCorners {
    let mut plant_queue = Vec::with_capacity(100);

    let mut corners_sum = 0;
    let mut fences_sum = 0;

    for y in 0..garden.plants.height {
        for x in 0..garden.plants.width {
            if garden.is_marked(x, y) {
                continue;
            }

            let plant = *garden.plants.get(x, y).unwrap();

            let mut plant_count = 0;
            let mut corners = 0;
            let mut fences = 0;

            plant_queue.push((x, y));

            garden.mark(x, y);

            while let Some((x, y)) = plant_queue.pop() {
                plant_count += 1;

                // Check right, bottom, left and top neighbors
                for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let (nx, ny) = (wradd!(x, dx), wradd!(y, dy));

                    if garden.are_plants_equal(nx, ny, plant) {
                        if !garden.is_marked(nx, ny) {
                            garden.mark(nx, ny);
                            plant_queue.push((nx, ny));
                        }
                    } else {
                        fences += 1;

                        if !skip_the_corners {
                            let (rx, ry) = (-dy, dx); // rotate clockwise
                            let (lx, ly) = (dy, -dx); // rotate counter-clockwise

                            // Check if we have corners (the middle edge is the "current" one).
                            // The `plant` is marked with "^"
                            //
                            //   |_| => 2 corners (both expressions are true)
                            //    ^
                            //
                            //   __| => 1 corner (only the second/clockwise expression is true)
                            //    ^

                            corners += usize::from(
                                !garden.are_plants_equal(wradd!(x, lx), wradd!(y, ly), plant)
                                    || garden.are_plants_equal(
                                        wradd!(x, lx, dx),
                                        wradd!(y, ly, dy),
                                        plant,
                                    ),
                            );

                            corners += usize::from(
                                !garden.are_plants_equal(wradd!(x, rx), wradd!(y, ry), plant)
                                    || garden.are_plants_equal(
                                        wradd!(x, rx, dx),
                                        wradd!(y, ry, dy),
                                        plant,
                                    ),
                            );
                        }
                    }
                }
            }

            fences_sum += plant_count * fences;

            if !skip_the_corners {
                corners_sum += plant_count * (corners / 2);
            }
        }
    }

    FencesAndCorners {
        fences: fences_sum,
        corners: corners_sum,
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
