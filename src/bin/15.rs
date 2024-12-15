use advent_of_code::{grid::Grid, wradd};

advent_of_code::solution!(15);

fn map_arrow_to_direction(arrow: u8) -> (isize, isize) {
    match arrow {
        b'^' => (0, -1),
        b'<' => (-1, 0),
        b'>' => (1, 0),
        b'v' => (0, 1),
        _ => unreachable!("Arrow not recognized: >>{}<<", arrow as char),
    }
}

fn parse_input(input: &str) -> (Grid<u8>, std::str::Bytes) {
    let (warehouse_part, directions_part) = input.split_once("\n\n").unwrap();
    (
        Grid::from(warehouse_part),
        directions_part.trim_end().bytes(),
    )
}

fn calculate_score(warehouse: Grid<u8>) -> usize {
    let mut score = 0;
    for y in 0..warehouse.height {
        for x in 0..warehouse.width {
            match *warehouse.get(x, y).unwrap() {
                b'O' | b'[' => {
                    score += (100 * y) + x;
                }
                _ => (),
            }
        }
    }
    score
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut warehouse, raw_directions) = parse_input(input);
    let (mut x, mut y) = warehouse.get_position_of(b'@').unwrap();

    // don't need the @ now that we have its coords.
    *warehouse.get_mut(x, y).unwrap() = b'.';

    raw_directions
        .filter(|b| *b != b'\n')
        .map(map_arrow_to_direction)
        .for_each(|(dx, dy)| {
            let mut box_in_front = false;
            let (mut nx, mut ny) = (wradd!(x, dx), wradd!(y, dy));

            while let Some(next) = warehouse.get_mut(nx, ny) {
                match *next {
                    b'.' => {
                        if box_in_front {
                            *next = b'O';
                        }
                        *warehouse.get_mut(x, y).unwrap() = b'.';
                        (x, y) = (wradd!(x, dx), wradd!(y, dy));
                        break;
                    }
                    b'O' => {
                        box_in_front = true;
                    }
                    _ => break,
                }
                nx = wradd!(nx, dx);
                ny = wradd!(ny, dy);
            }

            // Uncomment for fancy animation:
            // print!("{}[2J", 27 as char);
            // *warehouse.get_mut(x, y).unwrap() = b'@';
            // println!("{}", warehouse);
            // *warehouse.get_mut(x, y).unwrap() = b'.';
            // println!("Moving {}:{}", dx, dy);
            // thread::sleep(Duration::from_millis(100));
        });

    Some(calculate_score(warehouse))
}

fn stretch_warehouse(warehouse: &mut Grid<u8>) {
    let mut new_warehouse = Grid::from((warehouse.width * 2, warehouse.height, b'.'));

    for y in 0..warehouse.width {
        for x in 0..warehouse.height {
            let new_tile = match warehouse.get(x, y).unwrap() {
                b'#' => Some(b"##"),
                b'@' => Some(b"@."),
                b'O' => Some(b"[]"),
                _ => None,
            };

            if let Some(new_tile) = new_tile {
                *new_warehouse.get_mut(x * 2, y).unwrap() = new_tile[0];
                *new_warehouse.get_mut(x * 2 + 1, y).unwrap() = new_tile[1];
            }
        }
    }

    *warehouse = new_warehouse;
}

// TODO: Can be done better.. No need to set it to b'.' just for the tile
// to be overwritten by the previous box most of the time anyway..
fn move_box(warehouse: &mut Grid<u8>, x: usize, y: usize, dy: isize) {
    *warehouse.get_mut(x, wradd!(y, dy)).unwrap() = b'[';
    *warehouse.get_mut(x + 1, wradd!(y, dy)).unwrap() = b']';
    *warehouse.get_mut(x, y).unwrap() = b'.';
    *warehouse.get_mut(x + 1, y).unwrap() = b'.';
}

/// Always takes the left "[" part of a box!
///
/// If one of the stacktraces ends at a wall,
/// this whole call will evaluate to "false".
///
/// Basically Depth-first-search for a wall.
fn maybe_move_boxes(
    warehouse: &mut Grid<u8>,
    x: usize,
    y: usize,
    dy: isize,
    move_boxes: bool,
) -> bool {
    // left is the item behind the "[" part.
    // right is the item behind the "]" part.
    let (left, right) = (
        warehouse.get(x, wradd!(y, dy)).unwrap(),
        warehouse.get(x + 1, wradd!(y, dy)).unwrap(),
    );

    // This stacktrace of boxes is free to move.
    if *left == b'.' && *right == b'.' {
        if move_boxes {
            move_box(warehouse, x, y, dy);
        }
        return true;
    }

    // We've hit a wall, do not move any boxes involved at all.
    if *left == b'#' || *right == b'#' {
        return false;
    }

    // Scenario:
    //
    //  []
    //  []
    if *left == b'[' {
        if maybe_move_boxes(warehouse, x, wradd!(y, dy), dy, move_boxes) {
            if move_boxes {
                move_box(warehouse, x, y, dy);
            }
            return true;
        };
        return false;
    }

    // Scenario (we split this recursion into two):
    //
    // [][]
    //  []
    if *left == b']' && *right == b'[' {
        if maybe_move_boxes(warehouse, x + 1, wradd!(y, dy), dy, move_boxes)
            && maybe_move_boxes(warehouse, x - 1, wradd!(y, dy), dy, move_boxes)
        {
            if move_boxes {
                move_box(warehouse, x, y, dy);
            }
            return true;
        }
        return false;
    }

    // Scenario:
    //
    //  []..
    //   []
    if *left == b']' {
        if maybe_move_boxes(warehouse, x - 1, wradd!(y, dy), dy, move_boxes) {
            if move_boxes {
                move_box(warehouse, x, y, dy);
            }
            return true;
        }
        return false;
    }

    // Scenario:
    //
    //  ..[]
    //   []
    if *right == b'[' {
        if maybe_move_boxes(warehouse, x + 1, wradd!(y, dy), dy, move_boxes) {
            if move_boxes {
                move_box(warehouse, x, y, dy);
            }
            return true;
        }
        return false;
    }

    unreachable!("Oh no! A box was split in half!");
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut warehouse, raw_directions) = parse_input(input);

    stretch_warehouse(&mut warehouse);

    let (mut x, mut y) = warehouse.get_position_of(b'@').unwrap();
    let mut coord_stack: Vec<(usize, usize, u8)> = Vec::with_capacity(20);

    // don't need the @ now that we have its coords.
    *warehouse.get_mut(x, y).unwrap() = b'.';

    raw_directions
        .filter(|b| *b != b'\n')
        .map(map_arrow_to_direction)
        .for_each(|(dx, dy)| {
            let (mut nx, mut ny) = (wradd!(x, dx), wradd!(y, dy));
            if dy == 0 {
                // Case: Horizontal Movement.
                //
                // While the next item is a box, we add that to a stack.
                // If we encounter a b'.', we pop the stack and replace
                // the current tile with the stored one.
                //
                // If we hit a wall, we discard the stack and do nothing.
                while let Some(next) = warehouse.get_mut(nx, ny) {
                    match *next {
                        b'.' => {
                            while let Some((bx, by, b)) = coord_stack.pop() {
                                *warehouse.get_mut(wradd!(bx, dx), by).unwrap() = b;
                            }
                            (x, y) = (wradd!(x, dx), wradd!(y, dy));
                            *warehouse.get_mut(x, y).unwrap() = b'.';
                            break;
                        }
                        b'[' | b']' => {
                            coord_stack.push((nx, ny, *next));
                        }
                        _ => {
                            coord_stack.clear();
                            break;
                        }
                    }
                    nx = wradd!(nx, dx);
                    ny = wradd!(ny, dy);
                }
            } else {
                // Case: Vertical Movement.
                //
                // If the next item is b'.', wo simply go there.
                // If the next item is a box, we recursively check
                // if those boxes can be moved. If yes, we recursively
                // move those boxes.
                //
                // If the next item is a wall, we do nothing.
                if let Some(next) = warehouse.get_mut(nx, ny) {
                    match *next {
                        b'.' => {
                            (x, y) = (wradd!(x, dx), wradd!(y, dy));
                        }
                        b'[' => {
                            if maybe_move_boxes(&mut warehouse, nx, ny, dy, false) {
                                maybe_move_boxes(&mut warehouse, nx, ny, dy, true);
                                (x, y) = (wradd!(x, dx), wradd!(y, dy));
                            }
                        }
                        b']' => {
                            if maybe_move_boxes(&mut warehouse, nx - 1, ny, dy, false) {
                                maybe_move_boxes(&mut warehouse, nx - 1, ny, dy, true);
                                (x, y) = (wradd!(x, dx), wradd!(y, dy));
                            }
                        }
                        _ => (),
                    }
                }
            }

            // Uncomment for fancy animation:
            // print!("{}[2J", 27 as char);
            // *warehouse.get_mut(x, y).unwrap() = b'@';
            // println!("{}", warehouse);
            // *warehouse.get_mut(x, y).unwrap() = b'.';
            // println!("Moving {}:{}", dx, dy);
            // thread::sleep(Duration::from_millis(20));
        });

    Some(calculate_score(warehouse))
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
