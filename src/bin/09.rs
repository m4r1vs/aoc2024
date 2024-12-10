use std::collections::VecDeque;

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

type File = (usize, u8);

#[derive(Debug)]
enum FsEntity {
    File(File),
    FreeSpace(u8),
    Corruption(u8), // Because according to AOC rules, we cannot us
                    // space freed by a moved file for new files :/
}

struct Filesystem {
    content: VecDeque<FsEntity>,
    _index: usize,
    _last_found_indices: [usize; 9],
}

impl Iterator for Filesystem {
    type Item = (usize, FsEntity);

    fn next(&mut self) -> Option<Self::Item> {
        let mut return_entity: Option<(usize, FsEntity)> = None;
        if let Some(item) = self.content.pop_front() {
            match item {
                FsEntity::File(file) => {
                    return_entity = Some((self._index, FsEntity::File(file)));
                    self._index += file.1 as usize;
                }
                FsEntity::FreeSpace(free_size) => {
                    // 1. Get first fitting file from back and replace it with Corruption
                    //
                    // 2. Insert file at current spot
                    //
                    // 3. Calculate Free Space left
                    //
                    // 4. If some left, push_front(free_space_left)

                    let first_fitting_file: Option<File> = if let Some(index) =
                        self.content.iter().rposition(|fse| match fse {
                            FsEntity::File((_, file_size)) => free_size >= *file_size,
                            _ => false,
                        }) {
                        match self.content.get_mut(index) {
                            Some(file) => match file {
                                FsEntity::File(x) => {
                                    let tmp = Some(*x);
                                    *file = FsEntity::Corruption(x.1);
                                    tmp
                                }
                                _ => None,
                            },
                            _ => None,
                        }
                    } else {
                        None
                    };

                    match first_fitting_file {
                        Some(file) => {
                            return_entity = Some((self._index, FsEntity::File(file)));
                            self._index += file.1 as usize;

                            let free_space_remaining = free_size - file.1;
                            if free_space_remaining > 0 {
                                self.content
                                    .push_front(FsEntity::FreeSpace(free_space_remaining));
                            }
                        }
                        None => {
                            return_entity = Some((self._index, FsEntity::FreeSpace(free_size)));
                            self._index += free_size as usize;
                        }
                    }
                }
                FsEntity::Corruption(size) => {
                    return_entity = Some((self._index, FsEntity::FreeSpace(size)));
                    self._index += size as usize;
                }
            }
        }
        return_entity
    }
}

impl From<&str> for Filesystem {
    fn from(input: &str) -> Self {
        let mut content: VecDeque<FsEntity> = VecDeque::new();

        let mut is_file = true;
        let mut id = 0;
        for byte in input.trim_end().bytes() {
            if is_file {
                content.push_back(FsEntity::File((id, byte - 0x30)));
                id += 1;
            } else if byte > 0x30 {
                content.push_back(FsEntity::FreeSpace(byte - 0x30));
            }
            is_file = !is_file;
        }

        Filesystem {
            content,
            _index: 0,
            _last_found_indices: [
                id * 2,
                id * 2,
                id * 2,
                id * 2,
                id * 2,
                id * 2,
                id * 2,
                id * 2,
                id * 2,
            ],
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        Filesystem::from(input)
            .map(|(index, fs_entity)| match fs_entity {
                FsEntity::File(file) => {
                    (index..(index + file.1 as usize)).map(|i| i * file.0).sum()
                }
                _ => 0,
            })
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
