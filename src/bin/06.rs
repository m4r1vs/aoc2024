use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(PartialEq, Hash, Eq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, PartialEq)]
enum Facables {
    Obstacle,  // Cannot Move here
    Neuland,   // Guard has not been here
    Traversed, // Been here already
    Bedrock,   // Out of Map
}

#[derive(Clone)]
struct Guard {
    x: isize,
    y: isize,
    facing: Direction,
    map: GuardMap,
}

impl Guard {
    fn move_self(&mut self) -> u32 {
        while *self.get_element_infront() == Facables::Obstacle {
            self.turn_self();
        }

        (self.x, self.y) = self.simulate_move();

        if *self.map.get(self.x, self.y) == Facables::Neuland {
            self.map.mark_been(self.x as usize, self.y as usize);
            return 1;
        }

        0
    }

    fn turn_self(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn simulate_move(&self) -> (isize, isize) {
        match self.facing {
            Direction::Up => (self.x, self.y - 1),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
        }
    }

    fn get_element_infront(&self) -> &Facables {
        let (x, y) = self.simulate_move();
        self.map.get(x, y)
    }
}

#[derive(Clone)]
struct GuardMap {
    obstacles: Vec<Vec<Facables>>, // true if there is an obstacle
    guard_start_x: usize,
    guard_start_y: usize,
}

impl GuardMap {
    fn get(&self, x: isize, y: isize) -> &Facables {
        self.obstacles
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .unwrap_or(&Facables::Bedrock)
    }

    fn mark_been(&mut self, x: usize, y: usize) {
        self.obstacles[y][x] = Facables::Traversed;
    }

    fn add_obstacle(&mut self, x: usize, y: usize) -> bool {
        if self.get(x as isize, y as isize) == &Facables::Bedrock {
            false
        } else {
            self.obstacles[y][x] = Facables::Obstacle;
            true
        }
    }
}

impl From<&str> for GuardMap {
    fn from(input: &str) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut guard_start_x = 0;
        let mut guard_start_y = 0;

        let obstacles: Vec<Vec<Facables>> = input
            .lines()
            .map(|row| {
                x = 0;
                let returned = row
                    .bytes()
                    .map(|b| {
                        if b == b'^' {
                            guard_start_x = x;
                            guard_start_y = y;
                        }
                        x += 1;
                        if b == b'#' {
                            Facables::Obstacle
                        } else {
                            Facables::Neuland
                        }
                    })
                    .collect();

                y += 1;
                returned
            })
            .collect();

        Self {
            obstacles,
            guard_start_y,
            guard_start_x,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard = Guard {
        x: 0,
        y: 0,
        facing: Direction::Up,
        map: GuardMap::from(input),
    };

    // A little not-so-nice way of getting starting coords. TODO!
    guard.x = guard.map.guard_start_x as isize;
    guard.y = guard.map.guard_start_y as isize;

    let mut distance_traversed = 1;

    while *guard.get_element_infront() != Facables::Bedrock {
        distance_traversed += guard.move_self();
    }

    Some(distance_traversed)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut guard = Guard {
        x: 0,
        y: 0,
        facing: Direction::Up,
        map: GuardMap::from(input),
    };

    // A little not-so-nice way of getting starting coords. TODO!
    guard.x = guard.map.guard_start_x as isize;
    guard.y = guard.map.guard_start_y as isize;

    while *guard.get_element_infront() != Facables::Bedrock {
        guard.move_self();
    }

    let mut obstacles = 0;

    for (y, tiles) in guard.clone().map.obstacles.iter().enumerate() {
        for (x, _) in tiles
            .iter()
            .enumerate()
            .filter(|t| t.1 == &Facables::Traversed)
        {
            let mut been_set: HashSet<(isize, isize, Direction)> = HashSet::new();

            guard = Guard {
                x: 0,
                y: 0,
                facing: Direction::Up,
                map: GuardMap::from(input),
            };

            guard.x = guard.map.guard_start_x as isize;
            guard.y = guard.map.guard_start_y as isize;

            guard.map.add_obstacle(x, y);

            while *guard.get_element_infront() != Facables::Bedrock {
                if !been_set.insert((guard.x, guard.y, guard.facing)) {
                    obstacles += 1;
                    break;
                }
                guard.move_self();
            }
        }
    }

    Some(obstacles)
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
