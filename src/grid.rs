pub struct Grid<T> {
    pub elements: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.elements.get(self.width * y + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.elements.get_mut(self.width * y + x)
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
            elements: bytes,
        }
    }
}

impl From<(usize, usize, bool)> for Grid<bool> {
    fn from((width, height, initial_value): (usize, usize, bool)) -> Self {
        Grid {
            width,
            height,
            elements: vec![initial_value; width * height],
        }
    }
}
