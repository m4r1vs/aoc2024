use std::fmt::Display;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone)]
pub struct Grid<T> {
    pub items: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)] as char).ok();
            }
            writeln!(f).ok();
        }
        Ok(())
    }
}

impl Grid<u8> {
    pub fn from_bytes_with_transformation(
        input: std::str::Bytes,
        transformation: fn(u8) -> u8,
    ) -> Self {
        let mut bytes = Vec::with_capacity(input.len());
        let mut height = 0;

        input
            .filter_map(|b| {
                if b == b'\n' {
                    height += 1;
                    return None;
                } else {
                    return Some(b);
                }
            })
            .map(transformation)
            .for_each(|b| bytes.push(b));

        Self {
            width: bytes.len() / height,
            height,
            items: bytes,
        }
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn get_position_of(&self, needle: T) -> Option<(usize, usize)> {
        self.items
            .iter()
            .position(|hay| *hay == needle)
            .map(|pos| (pos % self.width, pos / self.width))
    }

    pub fn rget_position_of(&self, needle: T) -> Option<(usize, usize)> {
        self.items
            .iter()
            .rposition(|hay| *hay == needle)
            .map(|pos| (pos % self.width, pos / self.width))
    }
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.items.get(self.width * y + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.items.get_mut(self.width * y + x)
    }
}

impl From<&str> for Grid<u8> {
    fn from(input: &str) -> Self {
        let raw: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len();
        let height = raw.len();
        let mut bytes = Vec::with_capacity(width * height);

        raw.iter().for_each(|s| bytes.extend_from_slice(s));

        Grid {
            width,
            height,
            items: bytes,
        }
    }
}

impl<T: std::clone::Clone> From<(usize, usize, T)> for Grid<T> {
    fn from((width, height, initial_value): (usize, usize, T)) -> Self {
        Grid {
            width,
            height,
            items: vec![initial_value; width * height],
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.items[self.width * y + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.items[self.width * y + x]
    }
}
