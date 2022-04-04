use std::ops;

pub type Color = (u32, u32, u32);

pub struct PPM {
    width: i32,
    height: i32,
    bounds: Rect,
    buf: Vec<Color>,
}

impl PPM {
    pub fn new(width: i32, height: i32, init_color: Color) -> Self {
        Self {
            width,
            height,
            bounds: Rect(0, 0, width, height),
            buf: vec![init_color; (width * height) as usize],
        }
    }

    pub fn set(
        &mut self,
        &Vec2D(i, j): &Vec2D,
        color: (u32, u32, u32),
    ) {
        if !self.bounds.contains(Vec2D(i, j)) {
            panic!("Point ({}, {}) would be out of bounds ({:?})!", i, j, self.bounds);
        }
        self.buf[(j * self.width + i) as usize] = color;
    }

    pub fn get(
        &self,
        &Vec2D(i, j): &Vec2D,
    ) -> Color {
        self.buf[(j * self.width + i) as usize]
    }

    pub fn draw_rectangle(
        &mut self,
        &rect: &Rect,
        color: (u32, u32, u32),
    ) {
        for v in rect.into_iter() {
            self.set(&v, color);
        }
    }

    pub fn print(&self) {
        println!("P3\n{} {}\n255\n", self.width, self.height);

        for j in 0..self.height {
            for i in 0..self.width {
                let (r, g, b) = self.get(&Vec2D(i, j));
                println!("{} {} {}", r, g, b);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2D(pub i32, pub i32);

impl Vec2D {
    pub fn midpoint(&self, &Vec2D(x2, y2): &Vec2D) -> Vec2D {
        let &Vec2D(x1, y1) = self;
        Vec2D((x1 + x2) / 2, (y1 + y2) / 2)
    }
}

impl ops::Mul<f64> for Vec2D {
    type Output = Vec2D;

    fn mul(self, t: f64) -> Vec2D {
        let Vec2D(x, y) = self;
        Vec2D(
            (x as f64 * t).floor() as i32,
            (y as f64 * t).floor() as i32,
        )
    }
}

impl ops::Mul<Vec2D> for f64 {
    type Output = Vec2D;

    fn mul(self, Vec2D(x, y): Vec2D) -> Vec2D {
        Vec2D(
            (x as f64 * self).floor() as i32,
            (y as f64 * self).floor() as i32,
        )
    }
}

impl ops::Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, Vec2D(x2, y2): Vec2D) -> Vec2D {
        let Vec2D(x1, y1) = self;
        Vec2D(x1 + x2, y1 + y2)
    }
}

impl ops::Sub<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn sub(self, Vec2D(x2, y2): Vec2D) -> Vec2D {
        let Vec2D(x1, y1) = self;
        Vec2D(x1 - x2, y1 - y2)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect(pub i32, pub i32, pub i32, pub i32);

impl Rect {
    pub fn distance_to(&self, &Rect(x2, y2, w2, h2): &Rect) -> Option<i32> {
        let &Rect(x1, y1, w1, h1) = self;
        vec![x2 - (x1 + w1), y2 - (y1 + h1), x1 - (x2 + w2), y1 - (y2 + h2)]
            .iter()
            .filter(|&&d| d >= 0)
            .map(|&d| d)
            .min()
    }

    pub fn contains(&self, Vec2D(i, j): Vec2D) -> bool {
        let &Rect(x, y, w, h) = self;
        x < i && i < (x + w - 1) && y < j && j < (y + h - 1)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn width(&self) -> i32 {
        self.2
    }

    pub fn height(&self) -> i32 {
        self.3
    }
}

impl IntoIterator for Rect {
    type Item = Vec2D;
    type IntoIter = RectIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        RectIntoIterator {
            rect: self,
            cur: Vec2D(self.0, self.1),
        }
    }
}

pub struct RectIntoIterator {
    rect: Rect,
    cur: Vec2D,
}

impl Iterator for RectIntoIterator {
    type Item = Vec2D;

    fn next(&mut self) -> Option<Vec2D> {
        let &mut RectIntoIterator{ rect: Rect(x, y, w, h), cur: Vec2D(i, j) } = self;

        if j == y + h {
            None
        } else if i == x + w - 1 {
            self.cur = Vec2D(x, j + 1);
            Some(Vec2D(i, j))
        } else {
            self.cur = Vec2D(i + 1, j);
            Some(Vec2D(i, j))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_square_rect() {
        let points: Vec<Vec2D> = Rect(1, 1, 2, 2).into_iter().collect();
        assert_eq!(points, vec![
            Vec2D(1, 1),
            Vec2D(2, 1),
            Vec2D(1, 2),
            Vec2D(2, 2),
        ]);
    }

    #[test]
    fn test_rect_distance() {
        let distance = Rect(1, 1, 2, 2).distance_to(&Rect(4, 2, 3, 2));
        assert_eq!(distance, Some(1));
    }

    #[test]
    fn test_rect_distance_same_rect() {
        let distance = Rect(1, 1, 2, 2).distance_to(&Rect(1, 1, 2, 2));
        assert_eq!(distance, None);
    }

    #[test]
    fn test_rect_distance_overlap() {
        let distance = Rect(1, 1, 3, 3).distance_to(&Rect(3, 3, 1, 1));
        assert_eq!(distance, None);
    }
}
