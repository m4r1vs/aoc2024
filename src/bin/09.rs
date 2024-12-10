advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut defragged = parse_input(input);
    let mut last_find_index = 0;
    while let Some(id) = defragged.pop() {
        if id == usize::MAX {
            continue;
        }
        match defragged[last_find_index..]
            .iter_mut()
            .enumerate()
            .find(|(_, s)| *s == &usize::MAX)
        {
            Some((i, free)) => {
                *free = id;
                last_find_index += i + 1;
            }
            None => {
                defragged.push(id);
                break;
            }
        };
    }
    Some(defragged.iter().enumerate().map(|(i, id)| i * id).sum())
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut defragged: Vec<usize> = Vec::new();

    let mut is_file = true;
    let mut id = 0;
    for byte in input.trim_end().bytes() {
        if is_file {
            for _ in 0..(byte - 0x30) {
                defragged.push(id);
            }
            id += 1;
        } else {
            for _ in 0..(byte - 0x30) {
                defragged.push(usize::MAX)
            }
        }
        is_file = !is_file;
    }

    defragged
}

#[derive(Clone)]
struct File {
    id: usize,
    size: u8,
}

enum FsEntity {
    File(File),
    FreeSpace(u8),
    Corruption(u8), // Because according to AOC rules, we cannot use
                    // space freed by a moved file for new files :/
}

struct Filesystem {
    file_queue: Vec<FsEntity>,

    /// For each size (1 - 9), cache the last spot
    /// where we found a file of that size or smaller
    index_cache: [usize; 10],

    /// The Index needed for calculating the AOC "Checksum"
    aoc_index: usize,
}

impl Iterator for Filesystem {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.file_queue.pop() {
            Some(FsEntity::FreeSpace(free_size)) => {
                // 1. Get first fitting file from File Queue and replace it with Corruption
                // 1.1 Start search at index saved in cache and save index of found item
                //
                // 2. Insert file at current spot
                //
                // 3. Calculate Free Space left
                //
                // 4. If some left, push_front(free_space_left)

                match self
                    .file_queue
                    .iter_mut()
                    .skip(self.index_cache[free_size as usize])
                    .enumerate()
                    .find(|(_, file_system_entry)| match file_system_entry {
                        FsEntity::File(file) => free_size >= file.size,
                        _ => false,
                    }) {
                    Some((i, entry_of_first_fitting_file)) => match entry_of_first_fitting_file {
                        FsEntity::File(file) => {
                            self.index_cache[free_size as usize] += i + 1;

                            let i = self.aoc_index;
                            let s = file.size as usize;
                            let id = file.id;
                            self.aoc_index += file.size as usize;

                            let free_space_remaining = free_size - file.size;

                            *entry_of_first_fitting_file = FsEntity::Corruption(file.size);

                            if free_space_remaining > 0 {
                                self.file_queue
                                    .push(FsEntity::FreeSpace(free_space_remaining));
                            }

                            // Math Background:
                            //      for k in 0..(size - 1) { (i+k) * id }
                            // Reduces to:
                            //      (id * size * (2i + s - 1)) / 2

                            Some((id * s * (2 * i + s - 1)) / 2)
                        }
                        _ => unreachable!(),
                    },
                    _ => {
                        self.aoc_index += free_size as usize;
                        Some(0)
                    }
                }
            }
            Some(FsEntity::File(file)) => {
                let i = self.aoc_index;
                let s = file.size as usize;
                self.aoc_index += file.size as usize;
                Some((file.id * s * (2 * i + s - 1)) / 2) // See Math Background above
            }
            Some(FsEntity::Corruption(size)) => {
                self.aoc_index += size as usize;
                Some(0)
            }
            None => None,
        }
    }
}

impl From<&str> for Filesystem {
    fn from(input: &str) -> Self {
        let mut file_queue: Vec<FsEntity> = Vec::new();

        let mut is_file = true;
        let mut id = 0;
        for byte in input.trim_end().bytes() {
            if is_file {
                file_queue.push(FsEntity::File(File {
                    id,
                    size: byte - 0x30,
                }));
                id += 1;
            } else if byte > 0x30 {
                file_queue.push(FsEntity::FreeSpace(byte - 0x30));
            }
            is_file = !is_file;
        }

        file_queue.reverse();

        Filesystem {
            file_queue,
            aoc_index: 0,
            index_cache: [usize::MAX, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Filesystem::from(input).sum())
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
