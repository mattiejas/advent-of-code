use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundingBox<T> {
    pub start: Coord<T>,
    pub end: Coord<T>,
}

impl<T> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> BoundingBox<T>
where
    T: std::cmp::PartialOrd + std::ops::Sub<Output = T> + Mul<Output = T> + Copy,
{
    pub fn new(start: Coord<T>, end: Coord<T>) -> Self {
        Self { start, end }
    }

    pub fn width(&self) -> T {
        self.end.x - self.start.x
    }

    pub fn height(&self) -> T {
        self.end.y - self.start.y
    }

    pub fn area(&self) -> T {
        self.width() * self.height()
    }

    pub fn contains(&self, coord: &Coord<T>) -> bool {
        coord.x >= self.start.x
            && coord.x <= self.end.x
            && coord.y >= self.start.y
            && coord.y <= self.end.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounding_box_contains() {
        // Arrange
        let start = Coord::new(0, 0);
        let end = Coord::new(3, 4);
        let bounding_box = BoundingBox::new(start, end);
        let coord = Coord::new(1, 2);

        // Act
        let result = bounding_box.contains(&coord);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn bounding_box_does_not_contain() {
        // Arrange
        let start = Coord::new(0, 0);
        let end = Coord::new(3, 4);
        let bounding_box = BoundingBox::new(start, end);
        let coord = Coord::new(4, 5);

        // Act
        let result = bounding_box.contains(&coord);

        // Assert
        assert_eq!(result, false);
    }
}
