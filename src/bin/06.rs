use advent_of_code::grid::Grid;

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
}

#[derive(Clone)]
struct World {
    obstacles: Grid<Facables>, // true if there is an obstacle
    guard: Guard,
}

impl World {
    fn get(&self, x: usize, y: usize) -> Option<&Facables> {
        self.obstacles.get(x, y)
    }

    fn mark_been(&mut self, x: usize, y: usize) {
        if let Some(current) = self.obstacles.get_mut(x, y) {
            *current = Facables::Traversed;
        };
    }

    fn move_guard(&mut self) -> Option<Facables> {
        let (x, y) = self.guard.simulate_move();

        if x >= self.obstacles.width || y >= self.obstacles.height {
            return None;
        }

        match self.get(x, y) {
            Some(Facables::Obstacle) => {
                self.guard.turn_right();
                self.move_guard()
            }
            Some(Facables::Neuland) => {
                (self.guard.x, self.guard.y) = (x, y);
                self.mark_been(self.guard.x, self.guard.y);
                Some(Facables::Neuland)
            }
            Some(Facables::Traversed) => {
                (self.guard.x, self.guard.y) = (x, y);
                Some(Facables::Traversed)
            }
            None => None,
        }
    }
}

impl From<&str> for World {
    fn from(input: &str) -> Self {
        let mut x = 0;
        let mut y: isize = -1;
        let mut guard_start_x = 0;
        let mut guard_start_y = 0;

        let elements: Vec<Facables> = input
            .lines()
            .flat_map(|line| {
                y += 1;
                x = 0;
                line.bytes()
                    .map(|b| {
                        if b == b'^' {
                            guard_start_x = x;
                            guard_start_y = y as usize;
                        }
                        x += 1;
                        if b == b'#' {
                            Facables::Obstacle
                        } else {
                            Facables::Neuland
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Self {
            obstacles: Grid {
                items: elements,
                width: x,
                height: y as usize,
            },
            guard: Guard {
                x: guard_start_x,
                y: guard_start_y,
                facing: Direction::Up,
            },
        }
    }
}

#[derive(Clone)]
struct Guard {
    x: usize,
    y: usize,
    facing: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn simulate_move(&self) -> (usize, usize) {
        match self.facing {
            Direction::Up => (self.x, self.y - 1),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut world = World::from(input);

    let mut distance_traversed = 1;

    while let Some(cell) = world.move_guard() {
        if cell == Facables::Neuland {
            distance_traversed += 1;
        }
    }

    Some(distance_traversed)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut world = World::from(input);
    let cloned_world = world.clone();

    while world.move_guard().is_some() {}

    let mut obstacles = 0;

    'outer: for (i, _) in world
        .obstacles
        .items
        .iter()
        .enumerate()
        .filter(|(_, t)| *t == &Facables::Traversed)
    {
        let mut new_world = cloned_world.clone();
        let mut been: Grid<Option<[Direction; 4]>> = Grid {
            width: world.obstacles.width,
            height: world.obstacles.height,
            items: vec![None; world.obstacles.width * world.obstacles.height],
        };

        new_world.obstacles.items[i] = Facables::Obstacle;
        while new_world.move_guard().is_some() {
            let entry = been.get_mut(new_world.guard.x, new_world.guard.y).unwrap();
            match entry {
                Some(dir) => {
                    if dir.contains(&new_world.guard.facing) {
                        obstacles += 1;
                        continue 'outer;
                    }
                    dir[match_dir_to_idx(new_world.guard.facing)] = new_world.guard.facing;
                }
                None => {
                    *entry = Some([new_world.guard.facing; 4]);
                }
            }
        }
    }

    Some(obstacles)
}

fn match_dir_to_idx(direction: Direction) -> usize {
    match direction {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
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
